use crate::{constant, resources, routes, squire};
use actix;
use actix_web::{rt, web, Error, HttpRequest, HttpResponse};
use actix_ws::AggregatedMessage;
use fernet::Fernet;
use futures::future;
use futures::stream::StreamExt;
use std::sync::Arc;
use std::time::Duration;

/// Streams system resources via websocket through a loop.
///
/// # Arguments
///
/// * `request` - A reference to the Actix web `HttpRequest` object.
async fn send_system_resources(
    request: HttpRequest,
    mut session: actix_ws::Session,
    config: web::Data<Arc<squire::settings::Config>>,
) {
    let host = request.connection_info().host().to_string();
    let disk_stats = resources::stream::get_disk_stats();
    loop {
        let mut system_resources = resources::stream::system_resources(&config);
        system_resources.insert("disk_info".to_string(), disk_stats.clone());
        let serialized = serde_json::to_string(&system_resources).unwrap();
        match session.text(serialized).await {
            Ok(_) => (),
            Err(err) => {
                log::info!("Connection from '{}' has been {}", host, err.to_string().to_lowercase());
                break;
            }
        }
        // 500ms / 0.5s delay
        rt::time::sleep(Duration::from_secs(1)).await;
    }
}

/// Receives messages from the client and sends them back.
///
/// # Summary
///
/// Handles text, binary, and ping messages from the client.
///
/// # References
///
/// * [AggregatedMessage](https://docs.rs/actix-web/4.0.0-beta.8/actix_web/websocket/struct.AggregatedMessage.html)
/// * [ProtocolError](https://docs.rs/actix-web/4.0.0-beta.8/actix_web/websocket/enum.ProtocolError.html)
/// * [Session](https://docs.rs/actix-web/4.0.0-beta.8/actix_web/websocket/struct.Session.html)
/// * [Stream](https://docs.rs/futures/0.3.17/futures/stream/trait.Stream.html)
/// * [Unpin](https://doc.rust-lang.org/std/marker/trait.Unpin.html)
///
/// # Arguments
///
/// * `session` - A reference to the Actix web `Session` object.
/// * `stream` - A stream of `AggregatedMessage` objects.
async fn receive_messages(
    mut session: actix_ws::Session,
    mut stream: impl futures::Stream<Item=Result<AggregatedMessage, actix_ws::ProtocolError>> + Unpin,
) {
    while let Some(msg) = stream.next().await {
        match msg {
            Ok(AggregatedMessage::Text(text)) => {
                // echo text message
                session.text(text).await.unwrap();
            }
            Ok(AggregatedMessage::Binary(bin)) => {
                // echo binary message
                session.binary(bin).await.unwrap();
            }
            Ok(AggregatedMessage::Ping(msg)) => {
                // respond to PING frame with PONG frame
                session.pong(&msg).await.unwrap();
            }
            _ => {}
        }
    }
}

/// Handles the session by closing it after a certain duration.
///
/// # Arguments
///
/// * `session` - A reference to the Actix web `Session` object.
/// * `duration` - Duration in seconds to keep the session alive.
async fn session_handler(session: actix_ws::Session, duration: i64) {
    let session = session.clone();
    actix::spawn(async move {
        rt::time::sleep(Duration::from_secs(duration as u64)).await;
        let _ = session.close(None).await;
    });
}

/// Handles the WebSocket endpoint for system resources.
///
/// # Arguments
///
/// * `request` - A reference to the Actix web `HttpRequest` object.
/// * `fernet` - Fernet object to encrypt the auth payload that will be set as `session_token` cookie.
/// * `session_info` - Session struct that holds the `session_mapping` to handle sessions.
/// * `config` - Configuration data for the application.
/// * `stream` - A stream of `Payload` objects.
///
/// # Returns
///
/// Returns an `HttpResponse` with the appropriate status code.
#[route("/ws/system", method = "GET")]
async fn echo(
    request: HttpRequest,
    fernet: web::Data<Arc<Fernet>>,
    session_info: web::Data<Arc<constant::Session>>,
    config: web::Data<Arc<squire::settings::Config>>,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    log::info!("Websocket connection initiated");
    let auth_response = squire::authenticator::verify_token(&request, &config, &fernet, &session_info);
    if !auth_response.ok {
        return Ok(routes::auth::failed_auth(auth_response));
    }
    let (response, session, stream) = match actix_ws::handle(&request, stream) {
        Ok(result) => result,
        Err(_) => {
            return Ok(HttpResponse::ServiceUnavailable().finish());
        }
    };
    let stream = stream
        .aggregate_continuations();
    rt::spawn(async move {
        log::warn!("Connection established");
        let send_task = send_system_resources(request.clone(), session.clone(), config.clone());
        let receive_task = receive_messages(session.clone(), stream);
        let session_task = session_handler(session.clone(), config.session_duration);
        future::join3(send_task, receive_task, session_task).await;
    });
    Ok(response)
}

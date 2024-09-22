use crate::{constant, resources, routes, squire};
use actix_web::rt::time::sleep;
use actix_web::{rt, web, Error, HttpRequest, HttpResponse};
use actix_ws::AggregatedMessage;
use fernet::Fernet;
use std::sync::Arc;
use std::time::Duration;
use futures::future;
use futures::stream::StreamExt;


async fn send_system_resources(mut session: actix_ws::Session) {
    loop {
        let system_resources = resources::stream::system_resources();
        let strigified = serde_json::to_string(&system_resources).unwrap();
        session.text(strigified).await.unwrap();
        sleep(Duration::from_secs(1)).await;
    }
}

async fn receive_messages(
    mut session: actix_ws::Session,
    mut stream: impl futures::Stream<Item=Result<AggregatedMessage, actix_ws::ProtocolError>> + Unpin
) {
    while let Some(msg) = stream.next().await {
        match msg {
            Ok(AggregatedMessage::Text(text)) => {
                println!("Text: {:?}", &text);
                session.text(text).await.unwrap();
            }
            _ => {}
        }
    }
}


#[route("/ws/system", method = "GET")]
async fn echo(
    request: HttpRequest,
    fernet: web::Data<Arc<Fernet>>,
    session_info: web::Data<Arc<constant::Session>>,
    config: web::Data<Arc<squire::settings::Config>>,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let auth_response = squire::authenticator::verify_token(&request, &config, &fernet, &session_info);
    if !auth_response.ok {
        return Ok(routes::auth::failed_auth(auth_response));
    }
    let (response, session, stream) = actix_ws::handle(&request, stream)?;
    // todo: implement a session timeout here
    let stream = stream
        .aggregate_continuations()
        // aggregate continuation frames up to 1MiB
        // todo: check and remove limit if necessary
        .max_continuation_size(2_usize.pow(20));
    rt::spawn(async move {
        let send_task = send_system_resources(session.clone());
        let receive_task = receive_messages(session, stream);
        future::join(send_task, receive_task).await;
    });
    // respond immediately with response connected to WS session
    Ok(response)
}

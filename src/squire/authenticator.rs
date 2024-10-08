use std::collections::HashMap;
use std::sync::Arc;

use actix_web::http::header::HeaderValue;
use actix_web::{web, HttpRequest};
use chrono::Utc;
use fernet::Fernet;

use crate::constant;
use crate::squire;

/// Represents user credentials extracted from an authorization header.
///
/// Contains the username, signature, and timestamp obtained by decoding and parsing the authorization header.
struct Credentials {
    username: String,
    signature: String,
    timestamp: String,
}

/// Represents the result of authentication, indicating whether it was successful or not.
///
/// If successful, it includes the username and a generated key for the session.
pub struct AuthToken {
    pub ok: bool,
    pub detail: String,
    pub username: String,
}


/// Extracts credentials from the authorization header in the following steps
///
/// # Arguments
///
/// * `authorization` - An optional `HeaderValue` containing the authorization header.
///
/// # See Also
/// - Decodes the base64 encoded header
/// - Splits it into 3 parts with first one being the username followed by the signature and timestamp
/// - Converts the username from hex into a string.
///
/// # Returns
///
/// Returns a `Result` containing the extracted `Credentials` or an error message if extraction fails.
fn extract_credentials(authorization: &HeaderValue) -> Result<Credentials, &'static str> {
    let header = authorization.to_str().unwrap().to_string();
    // base64 encoded in JavaScript using inbuilt btoa function
    let b64_decode_response = squire::secure::base64_decode(&header);
    match b64_decode_response {
        Ok(decoded_auth) => {
            if decoded_auth.is_empty() {
                log::warn!("Authorization header was received without a value");
                return Err("No credentials received");
            }
            let vector: Vec<&str> = decoded_auth.split(',').collect();
            Ok(Credentials {
                // Decode hex username into string to retrieve password from config file
                username: squire::secure::hex_decode(vector.first().unwrap()),
                signature: vector.get(1).unwrap().to_string(),
                timestamp: vector.get(2).unwrap().to_string(),
            })
        }
        Err(err) => {
            Err(err)
        }
    }
}

/// Verifies user login based on extracted credentials and configuration settings.
///
/// # Arguments
///
/// * `request` - A reference to the Actix web `HttpRequest` object.
/// * `config` - Configuration data for the application.
/// * `session` - Session struct that holds the `session_mapping` to handle sessions.
///
/// # Returns
///
/// Returns a `Result` containing a `HashMap` with session information if authentication is successful,
/// otherwise returns an error message.
pub fn verify_login(
    request: &HttpRequest,
    config: &web::Data<Arc<squire::settings::Config>>,
    session: &web::Data<Arc<constant::Session>>,
) -> Result<HashMap<&'static str, String>, String> {
    let err_response;
    if let Some(authorization) = request.headers().get("authorization") {
        let extracted_credentials = extract_credentials(authorization);
        match extracted_credentials {
            Ok(credentials) => {
                // Check if the username is present in HashMap as key
                let message = format!("{}{}{}",
                                      squire::secure::hex_encode(&credentials.username),
                                      squire::secure::hex_encode(&config.password),
                                      credentials.timestamp);
                // Create a new signature with hex encoded username and password stored in config file as plain text
                let expected_signature = squire::secure::calculate_hash(message);
                if expected_signature == credentials.signature {
                    let key = squire::secure::keygen();
                    session.mapping.lock().unwrap().insert(credentials.username.to_string(), key.to_string());
                    let mut mapped = HashMap::new();
                    mapped.insert("username", credentials.username.to_string());
                    mapped.insert("key", key.to_string());
                    mapped.insert("timestamp", credentials.timestamp.to_string());
                    return Ok(mapped);
                } else {
                    log::warn!("{} entered bad credentials", credentials.username);
                    err_response = "Incorrect username or password";
                }
            }
            Err(err) => {
                err_response = err;
            }
        }
    } else {
        log::warn!("Authorization header was missing");
        err_response = "No credentials received";
    }
    Err(err_response.to_string())
}

/// Verifies a session token extracted from an HTTP request against stored session mappings and configuration.
///
/// # Arguments
///
/// * `request` - A reference to the Actix web `HttpRequest` object.
/// * `config` - Configuration data for the application.
/// * `fernet` - Fernet object to encrypt the auth payload that will be set as `session_token` cookie.
/// * `session` - Session struct that holds the `session_mapping` to handle sessions.
///
/// # Returns
///
/// Returns an instance of the `AuthToken` struct indicating the result of the token verification.
pub fn verify_token(
    request: &HttpRequest,
    config: &squire::settings::Config,
    fernet: &Fernet,
    session: &constant::Session,
) -> AuthToken {
    if session.mapping.lock().unwrap().is_empty() {
        log::warn!("No stored sessions, no point in validating further");
        return AuthToken {
            ok: false,
            detail: "Server doesn't recognize your session".to_string(),
            username: "NA".to_string(),
        };
    }
    if let Some(cookie) = request.cookie("session_token") {
        if let Ok(decrypted) = fernet.decrypt(cookie.value()) {
            let payload: HashMap<String, String> = serde_json::from_str(&String::from_utf8_lossy(&decrypted)).unwrap();
            let username = payload.get("username").unwrap().to_string();
            let cookie_key = payload.get("key").unwrap().to_string();
            let timestamp = payload.get("timestamp").unwrap().parse::<i64>().unwrap();
            let stored_key = session.mapping.lock().unwrap().get(&username).unwrap().to_string();
            let current_time = Utc::now().timestamp();
            // Max time and expiry for session token is set in the Cookie, but this is a fallback mechanism
            if stored_key != *cookie_key {
                return AuthToken {
                    ok: false,
                    detail: "Invalid session token".to_string(),
                    username,
                };
            }
            if current_time - timestamp > config.session_duration {
                return AuthToken {
                    ok: false,
                    detail: "Session Expired".to_string(),
                    username,
                };
            }
            let time_left = timestamp + config.session_duration - current_time;
            AuthToken {
                ok: true,
                detail: format!("Session valid for {}s", time_left),
                username,
            }
        } else {
            AuthToken {
                ok: false,
                detail: "Invalid session token".to_string(),
                username: "NA".to_string(),
            }
        }
    } else {
        AuthToken {
            ok: false,
            detail: "Session information not found".to_string(),
            username: "NA".to_string(),
        }
    }
}

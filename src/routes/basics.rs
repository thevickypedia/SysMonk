use crate::{constant, squire};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse};
use std::sync::Arc;

/// Handles the health endpoint, returning a JSON response indicating the server is healthy.
///
/// # Returns
///
/// Returns an `HttpResponse` with a status of 200 (OK), content type "application/json",
/// and a JSON body containing the string "Healthy".
#[get("/health")]
pub async fn health() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .json("Healthy")
}

/// Handles the root endpoint, logging the connection and returning an HTML response.
///
/// # Arguments
///
/// * `request` - A reference to the Actix web `HttpRequest` object.
/// * `session` - Session struct that holds the `session_mapping` to handle sessions.
/// * `metadata` - Struct containing metadata of the application.
/// * `template` - Configuration container for the loaded templates.
///
/// # Returns
///
/// Returns an `HttpResponse` with the index page as its body.
#[get("/")]
pub async fn root(request: HttpRequest,
                  metadata: web::Data<Arc<constant::MetaData>>,
                  template: web::Data<Arc<minijinja::Environment<'static>>>) -> HttpResponse {
    squire::custom::log_connection(&request);
    let index = template.get_template("index").unwrap();
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(index.render(minijinja::context!(version => &metadata.pkg_version)).unwrap())
}

use actix_web::{HttpRequest, HttpResponse};
use actix_web::http::StatusCode;
use minijinja::Template;

/// Logs connection information for an incoming HTTP request.
///
/// # Arguments
///
/// * `request` - A reference to the Actix web `HttpRequest` object.
///
/// This function logs the host and user agent information of the incoming connection.
pub fn log_connection(request: &HttpRequest) {
    let host = request.connection_info().host().to_string();
    log::info!("Connection received from {}", host);
    if let Some(user_agent) = request.headers().get("user-agent") {
        log::info!("User agent: {}", user_agent.to_str().unwrap())
    }
}

/// Frames a custom response into an error page.
///
/// # Arguments
///
/// * `title` - Title to be displayed in the error page.
/// * `error` - Jinja template for the error page.
/// * `version` - Application's version in the title tag of the webpage.
/// * `description` - Description to be displayed in the error page.
/// * `status_code` - Status code of the response.
///
/// # Returns
///
/// Returns an HTTPResponse with the appropriate status code formatted as HTML.
#[allow(dead_code)]
pub fn error(title: &str,
             error: Template,
             version: &String,
             description: String,
             status_code: StatusCode) -> HttpResponse {
    HttpResponse::build(status_code)
        .content_type("text/html; charset=utf-8")
        .body(error.render(minijinja::context!(
            version => version,
            title => title,
            description => description,
            help => r"Lost your way?\n\nHit the HOME button to navigate back to home page.",
            button_text => "HOME", button_link => "/home",
            block_navigation => true
        )).unwrap())
}

use std::collections::HashMap;
use crate::{constant, routes, squire, resources};
use actix_web::cookie::{Cookie, SameSite};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse};
use fernet::Fernet;
use std::sync::Arc;

/// Handles the monitor endpoint and rendering the appropriate HTML page.
///
/// # Arguments
///
/// * `request` - A reference to the Actix web `HttpRequest` object.
/// * `fernet` - Fernet object to encrypt the auth payload that will be set as `session_token` cookie.
/// * `session` - Session struct that holds the `session_mapping` to handle sessions.
/// * `metadata` - Struct containing metadata of the application.
/// * `config` - Configuration data for the application.
/// * `template` - Configuration container for the loaded templates.
///
/// # Returns
///
/// Returns an `HTTPResponse` with the cookie for `session_token` reset if available.
#[get("/monitor")]
pub async fn monitor(request: HttpRequest,
                     fernet: web::Data<Arc<Fernet>>,
                     session: web::Data<Arc<constant::Session>>,
                     metadata: web::Data<Arc<constant::MetaData>>,
                     config: web::Data<Arc<squire::settings::Config>>,
                     template: web::Data<Arc<minijinja::Environment<'static>>>) -> HttpResponse {
    let monitor_template = template.get_template("monitor").unwrap();
    let mut response = HttpResponse::build(StatusCode::OK);
    response.content_type("text/html; charset=utf-8");

    let auth_response = squire::authenticator::verify_token(&request, &config, &fernet, &session);
    if !auth_response.ok {
        return routes::auth::failed_auth(auth_response);
    }
    log::debug!("Session Validation Response: {}", auth_response.detail);

    let sys_info_map = resources::info::get_sys_info();
    let sys_info_disks_vec = resources::disks::get_all_disks();

    let sys_info_network = resources::network::get_network_info().await;

    let mut sys_info_basic: HashMap<&str, String> = sys_info_map.get("basic").unwrap().clone();
    let sys_info_mem_storage = sys_info_map.get("mem_storage").unwrap();

    let sys_info_disks = sys_info_disks_vec;

    if let Some(processor_name) = resources::processor::get_name() {
        sys_info_basic.insert("processor", processor_name);
    }
    let rendered = monitor_template.render(minijinja::context!(
        version => metadata.pkg_version,
        logout => "/logout",
        sys_info_basic => sys_info_basic,
        sys_info_mem_storage => sys_info_mem_storage,
        sys_info_network => sys_info_network,
        sys_info_disks => sys_info_disks
    )).unwrap();

    let mut cookie = Cookie::new("session_token", "");
    cookie.set_same_site(SameSite::Strict);
    cookie.make_removal();
    response.cookie(cookie);
    response.body(rendered)
}

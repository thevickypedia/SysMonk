use crate::{constant, legacy, resources, routes, squire};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse};
use fernet::Fernet;
use std::sync::Arc;
use sysinfo::Disks;

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
    let auth_response = squire::authenticator::verify_token(&request, &config, &fernet, &session);
    if !auth_response.ok {
        return routes::auth::failed_auth(auth_response);
    }
    let monitor_template = template.get_template("monitor").unwrap();
    let mut response = HttpResponse::build(StatusCode::OK);
    response.content_type("text/html; charset=utf-8");
    log::debug!("Session Validation Response: {}", auth_response.detail);

    // Refresh all disks during startup and re-use it
    let disks = Disks::new_with_refreshed_list();

    let sys_info_map = resources::info::get_sys_info(&disks);
    let legacy_disk_info = legacy::disks::get_all_disks();

    let sys_info_disks = if legacy_disk_info.is_empty() {
        resources::info::get_disks(&disks)
    } else {
        legacy_disk_info
    };

    let sys_info_network = resources::network::get_network_info().await;

    let sys_info_basic = sys_info_map.get("basic").unwrap();
    let sys_info_mem_storage = sys_info_map.get("mem_storage").unwrap();

    let rendered = monitor_template.render(minijinja::context!(
        version => metadata.pkg_version,
        logout => "/logout",
        sys_info_basic => sys_info_basic,
        sys_info_mem_storage => sys_info_mem_storage,
        sys_info_network => sys_info_network,
        sys_info_disks => sys_info_disks
    )).unwrap();
    response.body(rendered)
}

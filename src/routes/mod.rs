/// Module for `/` and `/health` entrypoint.
pub mod basics;
/// Module for `/login`, `/logout` and `/error` entrypoint.
pub mod auth;
/// Module for `/monitor` entrypoint.
pub mod monitor;
/// Module for `/ws/system` entrypoint.
pub mod websocket;

use actix_web::web;

pub fn configure_websocket(cfg: &mut web::ServiceConfig) {
    cfg.service(websocket::echo);
}

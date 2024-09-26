#![allow(rustdoc::bare_urls)]
#![doc = include_str!("../README.md")]

#[macro_use]
extern crate actix_web;

use std::io;

use actix_web::{middleware, web, App, HttpServer};

/// Module for the structs and functions called during startup.
mod constant;
/// Module for all the API entry points.
mod routes;
/// Module to store all the helper functions.
mod squire;
/// Module to store all the HTML templates rendered using Jinja.
mod templates;
/// Module for functions related to system resources.
mod resources;
/// Module for legacy (but still useful for reference) functions
mod legacy;

/// Contains entrypoint and initializer settings to trigger the asynchronous `HTTPServer`
///
/// # Examples
///
/// ```no_run
/// #[actix_rt::main]
/// async fn main() {
///     match sysmonk::start().await {
///         Ok(_) => {
///             println!("SysMonk session terminated")
///         }
///         Err(err) => {
///             eprintln!("Error starting SysMonk: {}", err)
///         }
///     }
/// }
/// ```
pub async fn start() -> io::Result<()> {
    let metadata = constant::build_info();
    let config = squire::startup::get_config(&metadata);

    squire::startup::init_logger(config.debug, config.utc_logging, &metadata.crate_name);
    println!("{}[v{}] - {}", &metadata.pkg_name, &metadata.pkg_version, &metadata.description);
    squire::ascii_art::random();

    // Create a dedicated clone, since it will be used within closure
    let config_clone = config.clone();
    let host = format!("{}:{}", config.host, config.port);
    log::info!("{} [workers:{}] running on http://{} (Press CTRL+C to quit)",
        &metadata.pkg_name, &config.workers, &host);
    let jinja = templates::environment();
    let fernet = constant::fernet_object();
    let session = constant::session_info();
    /*
        || syntax is creating a closure that serves as the argument to the HttpServer::new() method.
        The closure is defining the configuration for the Actix web server.
        The purpose of the closure is to configure the server before it starts listening for incoming requests.
     */
    let application = move || {
        App::new()  // Creates a new Actix web application
            .app_data(web::Data::new(config_clone.clone()))
            .app_data(web::Data::new(jinja.clone()))
            .app_data(web::Data::new(fernet.clone()))
            .app_data(web::Data::new(session.clone()))
            .app_data(web::Data::new(metadata.clone()))
            .wrap(squire::middleware::get_cors(config_clone.websites.clone()))
            .wrap(middleware::Logger::default())  // Adds a default logger middleware to the application
            .service(routes::basics::health)  // Registers a service for handling requests
            .service(routes::basics::root)
            .service(routes::basics::health)  // Registers a service for handling requests
            .service(routes::auth::login)
            .service(routes::monitor::monitor)
            .service(routes::auth::logout)
            .service(routes::auth::error)
            .configure(routes::configure_websocket)
    };
    let server = HttpServer::new(application)
        .workers(config.workers)
        .max_connections(config.max_connections);
    match server.bind(host) {
        Ok(bound_server) => bound_server.run().await,
        Err(err) => {
            log::error!("Failed to bind server: {}", err);
            Err(err)
        }
    }
}

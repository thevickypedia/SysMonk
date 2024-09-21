#![allow(rustdoc::bare_urls)]
#![doc = include_str!("../README.md")]

#[macro_use]
extern crate actix_web;

use std::io;

use actix_web::{App, HttpServer, middleware, web};

/// Module for the structs and functions called during startup.
mod constant;
/// Module for all the API entry points.
mod routes;
/// Module to store all the helper functions.
mod squire;

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
    /*
        || syntax is creating a closure that serves as the argument to the HttpServer::new() method.
        The closure is defining the configuration for the Actix web server.
        The purpose of the closure is to configure the server before it starts listening for incoming requests.
     */
    let application = move || {
        App::new()  // Creates a new Actix web application
            .app_data(web::Data::new(config_clone.clone()))
            .app_data(web::Data::new(metadata.clone()))
            .wrap(squire::middleware::get_cors(config_clone.websites.clone()))
            .wrap(middleware::Logger::default())  // Adds a default logger middleware to the application
            .service(routes::basics::health)  // Registers a service for handling requests
            // .service(routes::basics::root)
            // .service(routes::auth::login)
            // .service(routes::auth::logout)
            // .service(routes::auth::home)
            // .service(routes::basics::profile)
            // .service(routes::fileio::edit)
            // .service(routes::auth::error)
            // .service(routes::media::track)
            // .service(routes::media::stream)
            // .service(routes::media::streaming_endpoint)
            // .service(routes::upload::upload_files)
            // .service(routes::upload::save_files)
    };
    let server = HttpServer::new(application)
        .workers(config.workers)
        .max_connections(config.max_connections);
    server.bind(host)?
        .run()
        .await
}

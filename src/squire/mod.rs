/// Module for the web data configuration that holds the secrets required by the application.
pub mod settings;
/// Module that initializes the logger and loads the configuration into a dedicated Struct.
pub mod startup;
/// Module for the functions that yield an ASCII art to print during startup.
pub mod ascii_art;
/// Module for the CORS middleware configuration.
pub mod middleware;
/// Module that handles parsing command line arguments.
pub mod parser;
/// Module for custom functions that logs connection information and builds custom error responses.
pub mod custom;
/// Module that handles the authentication and
pub mod authenticator;
/// Module for the functions that handle encryption/encoding and decryption/decoding.
pub mod secure;
/// Module for utility functions.
pub mod util;

use std;
use std::io::Write;

use crate::squire::settings;
use crate::{constant, squire};
use chrono::{DateTime, Local};
use regex::Regex;

/// Initializes the logger based on the provided debug flag and cargo information.
///
/// # Arguments
///
/// * `debug` - A flag indicating whether to enable debug mode for detailed logging.
/// * `crate_name` - Name of the crate loaded during compile time.
pub fn init_logger(debug: bool, utc: bool, crate_name: &String) {
    if debug {
        std::env::set_var("RUST_LOG", format!(
            "actix_web=debug,actix_server=info,{}=debug", crate_name
        ));
        std::env::set_var("RUST_BACKTRACE", "1");
    } else {
        // Set Actix logging to warning mode since it becomes too noisy when streaming
        std::env::set_var("RUST_LOG", format!(
            "actix_web=warn,actix_server=warn,{}=info", crate_name
        ));
        std::env::set_var("RUST_BACKTRACE", "0");
    }
    let timestamp = if utc {
        DateTime::<chrono::Utc>::from(Local::now())
            .format("%Y-%m-%dT%H:%M:%SZ")
            .to_string()
    } else {
        Local::now()
            .format("%Y-%m-%dT%H:%M:%SZ")
            .to_string()
    };
    env_logger::Builder::from_default_env()
        .format(move |buf, record| {
            writeln!(
                buf,
                "[{} {} {}:{}] - {}",
                timestamp,
                record.level(),
                record.target(),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();
}

/// Extracts the mandatory env vars by key and parses it as `HashMap<String, String>` and `PathBuf`
///
/// # Returns
///
/// Returns a tuple of `HashMap<String, String>` and `PathBuf`.
///
/// # Panics
///
/// If the value is missing or if there is an error parsing the `HashMap`
fn mandatory_vars() -> (String, String) {
    let mut errors = "".to_owned();
    let username = match std::env::var("username") {
        Ok(val) => val,
        Err(_) => {
            errors.push_str(
                "\nusername\n\texpected a string, received null [value=missing]\n",
            );
            "".to_string()
        }
    };
    let password = match std::env::var("password") {
        Ok(val) => val,
        Err(_) => {
            errors.push_str(
                "\npassword\n\texpected a string, received null [value=missing]\n",
            );
            "".to_string()
        }
    };
    if !errors.is_empty() {
        panic!("{}", errors);
    }
    (username, password)
}

/// Extracts the env var by key and parses it as a `bool`
///
/// # Arguments
///
/// * `key` - Key for the environment variable.
///
/// # Returns
///
/// Returns an `Option<bool>` if the value is available.
///
/// # Panics
///
/// If the value is present, but it is an invalid data-type.
fn parse_bool(key: &str) -> Option<bool> {
    match std::env::var(key) {
        Ok(val) => match val.parse() {
            Ok(parsed) => Some(parsed),
            Err(_) => {
                panic!("\n{}\n\texpected bool, received '{}' [value=invalid]\n", key, val);
            }
        },
        Err(_) => None,
    }
}

/// Extracts the env var by key and parses it as a `i64`
///
/// # Arguments
///
/// * `key` - Key for the environment variable.
///
/// # Returns
///
/// Returns an `Option<i64>` if the value is available.
///
/// # Panics
///
/// If the value is present, but it is an invalid data-type.
fn parse_i64(key: &str) -> Option<i64> {
    match std::env::var(key) {
        Ok(val) => match val.parse() {
            Ok(parsed) => Some(parsed),
            Err(_) => {
                panic!("\n{}\n\texpected i64, received '{}' [value=invalid]\n", key, val);
            }
        },
        Err(_) => None,
    }
}

/// Extracts the env var by key and parses it as a `u16`
///
/// # Arguments
///
/// * `key` - Key for the environment variable.
///
/// # Returns
///
/// Returns an `Option<u16>` if the value is available.
///
/// # Panics
///
/// If the value is present, but it is an invalid data-type.
fn parse_u16(key: &str) -> Option<u16> {
    match std::env::var(key) {
        Ok(val) => match val.parse() {
            Ok(parsed) => Some(parsed),
            Err(_) => {
                panic!("\n{}\n\texpected u16, received '{}' [value=invalid]\n", key, val);
            }
        },
        Err(_) => None,
    }
}

/// Extracts the env var by key and parses it as a `usize`
///
/// # Arguments
///
/// * `key` - Key for the environment variable.
///
/// # Returns
///
/// Returns an `Option<usize>` if the value is available.
///
/// # Panics
///
/// If the value is present, but it is an invalid data-type.
fn parse_usize(key: &str) -> Option<usize> {
    match std::env::var(key) {
        Ok(val) => match val.parse() {
            Ok(parsed) => Some(parsed),
            Err(_) => {
                panic!("\n{}\n\texpected usize, received '{}' [value=invalid]\n", key, val);
            }
        },
        Err(_) => None,
    }
}

/// Extracts the env var by key and parses it as a `Vec<String>`
///
/// # Arguments
///
/// * `key` - Key for the environment variable.
///
/// # Returns
///
/// Returns an `Option<Vec<String>>` if the value is available.
///
/// # Panics
///
/// If the value is present, but it is an invalid data-type.
fn parse_vec(key: &str) -> Option<Vec<String>> {
    match std::env::var(key) {
        Ok(val) => match serde_json::from_str::<Vec<String>>(&val) {
            Ok(parsed) => Some(parsed),
            Err(_) => {
                panic!("\n{}\n\texpected vec, received '{}' [value=invalid]\n", key, val);
            }
        },
        Err(_) => None,
    }
}

/// Handler that's responsible to parse all the env vars.
///
/// # Returns
///
/// Instantiates the `Config` struct with the required parameters.
fn load_env_vars() -> settings::Config {
    let (username, password) = mandatory_vars();
    let debug = parse_bool("debug").unwrap_or(settings::default_debug());
    let utc_logging = parse_bool("utc_logging").unwrap_or(settings::default_utc_logging());
    let host = std::env::var("host").unwrap_or(settings::default_host());
    let port = parse_u16("port").unwrap_or(settings::default_port());
    let session_duration = parse_i64("session_duration").unwrap_or(settings::default_session_duration());
    let workers = parse_usize("workers").unwrap_or(settings::default_workers());
    let max_connections = parse_usize("max_connections").unwrap_or(settings::default_max_connections());
    let websites = parse_vec("websites").unwrap_or(settings::default_vec());
    let services = parse_vec("services").unwrap_or(settings::default_vec());
    let processes = parse_vec("processes").unwrap_or(settings::default_vec());
    settings::Config {
        username,
        password,
        debug,
        utc_logging,
        host,
        port,
        session_duration,
        workers,
        max_connections,
        websites,
        services,
        processes
    }
}

/// Verifies the strength of a password string.
///
/// A secret is considered strong if it meets the following conditions:
/// - At least 32 characters long
/// - Contains at least one digit
/// - Contains at least one uppercase letter
/// - Contains at least one lowercase letter
/// - Contains at least one special character
///
/// # Arguments
///
/// * `password` - A reference to a string slice (`&str`) that represents the password to check.
///
/// # Returns
///
/// This function returns a `Result<(), String>`.
/// - `Ok(())` is returned if all conditions are met.
/// - `Err(String)` is returned with an error message if any condition fails.
pub fn complexity_checker(password: &str) -> Result<(), String> {
    let mock_password = "*".repeat(password.len());

    // Check minimum length
    if password.len() < 8 {
        return Err(
            format!(
                "\npassword\n\t[{}] password must be at least 8 or more characters [value=invalid]\n", mock_password
            )
        );
    }

    // Check for at least one digit
    let has_digit = Regex::new(r"\d").unwrap();
    if !has_digit.is_match(password) {
        return Err(
            format!(
                "\npassword\n\t[{}] password must include at least one digit [value=invalid]\n", mock_password
            )
        );
    }

    // Check for at least one uppercase letter
    let has_uppercase = Regex::new(r"[A-Z]").unwrap();
    if !has_uppercase.is_match(password) {
        return Err(
            format!(
                "\npassword\n\t[{}] password must include at least one uppercase letter [value=invalid]\n", mock_password
            )
        );
    }

    // Check for at least one lowercase letter
    let has_lowercase = Regex::new(r"[a-z]").unwrap();
    if !has_lowercase.is_match(password) {
        return Err(
            format!(
                "\npassword\n\t[{}] password must include at least one lowercase letter [value=invalid]\n", mock_password
            )
        );
    }

    // Check for at least one special character
    let has_special_char = Regex::new(r###"[ !#$%&'()*+,./:;<=>?@\\^_`{|}~"-]"###).unwrap();
    if !has_special_char.is_match(password) {
        return Err(
            format!(
                "\npassword\n\t[{}] password must contain at least one special character [value=invalid]\n", mock_password
            )
        );
    }
    Ok(())
}

/// Validates all the required environment variables with the required settings.
///
/// # Returns
///
/// Returns the `Config` struct containing the required parameters.
fn validate_vars() -> settings::Config {
    let config = load_env_vars();
    let mut errors = "".to_owned();
    if config.username.len() < 4 {
        let err1 = format!(
            "\nusername\n\t[{}] username should be at least 4 or more characters [value=invalid]\n",
            config.username
        );
        errors.push_str(&err1);
    }
    match complexity_checker(&config.password) {
        Ok(_) => (),
        Err(err) => {
            errors.push_str(&err);
        }
    }
    if !errors.is_empty() {
        panic!("{}", errors);
    }
    config
}

/// Retrieves the environment variables and parses as the data-type specified in Config struct.
///
/// # Arguments
///
/// * `metadata` - Struct containing metadata of the application.
///
/// # Returns
///
/// Converts the config struct into an `Arc` and returns it.
pub fn get_config(metadata: &constant::MetaData) -> std::sync::Arc<settings::Config> {
    let mut env_file = squire::parser::arguments(metadata);
    if env_file.is_empty() {
        env_file = std::env::var("env_file")
            .unwrap_or(std::env::var("ENV_FILE")
                .unwrap_or(".env".to_string()));
    }
    let env_file_path = std::env::current_dir()
        .unwrap_or_default()
        .join(env_file);
    // https://github.com/dotenv-rs/dotenv/issues/94
    let _ = dotenv::from_path(env_file_path.as_path());
    std::sync::Arc::new(validate_vars())
}

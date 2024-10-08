use regex::Regex;
use std::collections::HashMap;
use std::process::Command;

/// Function to retrieve the REGEX object for an IPv4 address format
///
/// # Returns
///
/// A `Regex` object that can be used to match an IPv4 address format
pub fn ip_regex() -> Regex {
    Regex::new(
        r"^(?:(?:25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9][0-9]|[0-9])\.){3}(?:25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9][0-9]|[0-9])$"
    ).unwrap()
}

/// Mapping of URLs to check for public IP addresses
///
/// # Returns
///
/// A `HashMap` containing the URL and a boolean value indicating whether the URL expects text or JSON response
pub fn public_ip_mapping() -> HashMap<String, bool> {
    let mapping: HashMap<String, bool> = vec![
        ("https://checkip.amazonaws.com/".to_string(), true), // expects text
        ("https://api.ipify.org/".to_string(), true),         // expects text
        ("https://ipinfo.io/ip/".to_string(), true),          // expects text
        ("https://v4.ident.me/".to_string(), true),           // expects text
        ("https://httpbin.org/ip".to_string(), false),        // expects JSON
        ("https://myip.dnsomatic.com/".to_string(), true),    // expects text
    ]
        .into_iter()
        .collect();
    mapping
}


/// Function to convert seconds to human-readable format
///
/// # Arguments
///
/// * `seconds` - The number of seconds to convert
///
/// # Returns
///
/// A `String` containing the human-readable format of the seconds
pub fn convert_seconds(seconds: i64) -> String {
    let days = seconds / 86_400; // 86,400 seconds in a day
    let hours = (seconds % 86_400) / 3_600; // 3,600 seconds in an hour
    let minutes = (seconds % 3_600) / 60; // 60 seconds in a minute
    let remaining_seconds = seconds % 60;

    let mut result = Vec::new();

    if days > 0 {
        result.push(format!("{} day{}", days, if days > 1 { "s" } else { "" }));
    }
    if hours > 0 {
        result.push(format!("{} hour{}", hours, if hours > 1 { "s" } else { "" }));
    }
    if minutes > 0 && result.len() < 2 {
        result.push(format!("{} minute{}", minutes, if minutes > 1 { "s" } else { "" }));
    }
    if remaining_seconds > 0 && result.len() < 2 {
        result.push(format!("{} second{}", remaining_seconds, if remaining_seconds > 1 { "s" } else { "" }));
    }
    result.join(" and ")
}

/// Function to convert byte size to human-readable format
///
/// # Arguments
///
/// * `byte_size` - The size in bytes to convert
///
/// # Returns
///
/// A `String` containing the human-readable format of the byte size
pub fn size_converter(byte_size: u64) -> String {
    let size_name = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    let mut index = 0;
    let mut size = byte_size as f64;

    while size >= 1024.0 && index < size_name.len() - 1 {
        size /= 1024.0;
        index += 1;
    }

    format!("{:.2} {}", size, size_name[index])
}

/// Function to run a terminal command.
///
/// # Arguments
///
/// * `command` - Command to run
/// * `log` - Boolean flag to log errors
///
/// # Returns
///
/// A `String` containing the parsed size string.
pub fn run_command(command: &str, args: &[&str], log: bool) -> Result<String, String> {
    match Command::new(command)
        .args(args)
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                let exit_code = output.status.code().unwrap_or(-1);
                if log {
                    log::error!("Command [{}] failed with exit code: {}", command, exit_code);
                    log::error!("Stderr: {}", stderr);
                }
                Err(stderr)
            }
        }
        Err(err) => {
            if log {
                log::error!("Failed to execute command [{}]: {}", command, err);
            }
            Err(err.to_string())
        }
    }
}

/// Function to capitalize the first letter of each word in a string.
///
/// # Arguments
///
/// * `s` - The string to capitalize
/// * `sep` - The separator to split the string
///
/// # Returns
///
/// A `String` containing the capitalized string
pub fn capwords(string: &str, sep: Option<&str>) -> String {
    let separator = sep.unwrap_or(" ");
    let mut result = Vec::new();

    for word in string.split(separator) {
        let mut chars = word.chars();
        if let Some(first) = chars.next() {
            let capitalized_word = first.to_uppercase().collect::<String>() + chars.as_str();
            result.push(capitalized_word);
        } else {
            result.push(String::new());
        }
    }

    result.join(separator)
}

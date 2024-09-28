use std::fs::File;
use std::io::{self, BufRead};

use crate::squire;

/// Function to get processor information.
///
/// # Arguments
///
/// * `lib_path` - The path to the library used to get processor information.
///
/// # Returns
///
/// A `Option` containing the processor information if successful, otherwise `None`.
fn get_processor_info_darwin(lib_path: &str) -> Result<String, String> {
    squire::util::run_command(lib_path, &["-n", "machdep.cpu.brand_string"], true)
}

/// Function to get processor information on Linux.
///
/// # Arguments
///
/// * `lib_path` - The path to the library used to get processor information.
///
/// # Returns
///
/// A `Option` containing the processor information if successful, otherwise `None`.
fn get_processor_info_linux(lib_path: &str) -> Result<String, String> {
    let file = match File::open(lib_path) {
        Ok(file) => file,
        Err(_) => return Err(format!("Failed to open '{}'", lib_path)),
    };
    for line in io::BufReader::new(file).lines() {
        match line {
            Ok(line) => {
                if line.contains("model name") {
                    let parts: Vec<&str> = line.split(':').collect();
                    if parts.len() == 2 {
                        return Ok(parts[1].trim().to_string());
                    }
                }
            }
            Err(_) => return Err(format!("Error reading lines in '{}'", lib_path)),
        }
    }
    Err(format!("Model name not found in '{}'", lib_path))
}

/// Function to get processor information on Windows.
///
/// # Arguments
///
/// * `lib_path` - The path to the library used to get processor information.
///
/// # Returns
///
/// A `Option` containing the processor information if successful, otherwise `None`.
fn get_processor_info_windows(lib_path: &str) -> Result<String, String> {
    let result = squire::util::run_command(lib_path, &["cpu", "get", "name"], true);
    let output = match result {
        Ok(output) => output,
        Err(_) => return Err("Failed to get processor info".to_string()),
    };
    let lines: Vec<&str> = output.trim().split('\n').collect();
    if lines.len() > 1 {
        Ok(lines[1].trim().to_string())
    } else {
        Err("Invalid output from command".to_string())
    }
}

/// OS-agnostic function to get processor name.
///
/// # Returns
///
/// A `Option` containing the processor name if successful, otherwise `None`.
pub fn get_name() -> Option<String> {
    let operating_system = std::env::consts::OS;
    let result = match operating_system {
        "macos" => get_processor_info_darwin("/usr/sbin/sysctl"),
        "linux" => get_processor_info_linux("/proc/cpuinfo"),
        "windows" => get_processor_info_windows("C:\\Windows\\System32\\wbem\\wmic.exe"),
        _ => {
            log::error!("Unsupported operating system: {}", operating_system);
            Err("Unsupported operating system".to_string())
        }
    };
    match result {
        Ok(info) => Some(info),
        Err(err) => {
            log::error!("{}", err);
            None
        }
    }
}

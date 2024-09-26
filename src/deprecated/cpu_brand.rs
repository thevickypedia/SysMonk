use std::fs::File;
use std::io::{self, BufRead};

use crate::deprecated::helper::run_command;

/// Function to get processor information.
///
/// # Arguments
///
/// * `lib_path` - The path to the library used to get processor information.
///
/// # Returns
///
/// A `Option` containing the processor information if successful, otherwise `None`.
fn get_processor_info_darwin(lib_path: &str) -> Result<String, &'static str> {
    let result = run_command(lib_path, &["-n", "machdep.cpu.brand_string"]);
    if result.is_err() {
        return Err("Failed to get processor info");
    }
    Ok(result.unwrap())
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
fn get_processor_info_linux(lib_path: &str) -> Result<String, &'static str> {
    let file = match File::open(lib_path) {
        Ok(file) => file,
        Err(_) => return Err("Failed to open file"),
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
            Err(_) => return Err("Error reading line"),
        }
    }
    Err("Model name not found")
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
fn get_processor_info_windows(lib_path: &str) -> Result<String, &'static str> {
    let result = run_command(lib_path, &["cpu", "get", "name"]);
    let output = match result {
        Ok(output) => output,
        Err(_) => return Err("Failed to get processor info"),
    };
    let lines: Vec<&str> = output.trim().split('\n').collect();
    if lines.len() > 1 {
        Ok(lines[1].trim().to_string())
    } else {
        Err("Invalid output from command")
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
        "darwin" => get_processor_info_darwin("/usr/sbin/sysctl"),
        "linux" => get_processor_info_linux("/proc/cpuinfo"),
        "windows" => get_processor_info_windows("C:\\Windows\\System32\\wbem\\wmic.exe"),
        _ => {
            log::error!("Unsupported operating system: {}", operating_system);
            Err("Unsupported operating system")
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

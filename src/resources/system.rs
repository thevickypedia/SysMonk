use crate::squire;
use serde::{Deserialize, Serialize};
use std::str;

#[derive(Deserialize, Serialize, Debug)]
pub struct OperatingSystem {
    pub name: String,
    pub architecture: String,
}

/// Function to get OS architecture.
///
/// # Returns
///
/// A string with the OS architecture.
fn unamem() -> String {
    // Get architecture using `uname -m` with fallback
    let result = squire::util::run_command("uname", &["-m"], true);
    match result {
        Ok(output) => output.to_lowercase(),
        Err(_) => {
            log::error!("Failed to execute command");
            "".to_string()
        }
    }
}

/// Function to get OS name.
///
/// # Returns
///
/// A string with the OS name.
fn unameu() -> String {
    // Get OS using `uname`
    let result = squire::util::run_command("uname", &[], true);
    match result {
        Ok(output) => output.to_uppercase(),
        Err(_) => {
            log::error!("Failed to execute command");
            std::env::consts::OS.to_uppercase()
        }
    }
}

/// Function to get OS architecture.
///
/// # Returns
///
/// A `OperatingSystem` struct with the OS name and architecture.
pub fn os_arch() -> OperatingSystem {
    let arch = match unamem() {
        arch if arch.contains("aarch64") || arch.contains("arm64") => "arm64",
        arch if arch.contains("64") => "amd64",
        arch if arch.contains("86") => "386",
        arch if arch.contains("armv5") => "armv5",
        arch if arch.contains("armv6") => "armv6",
        arch if arch.contains("armv7") => "armv7",
        _ => "",
    };
    let os = match unameu() {
        os if os.contains("DARWIN") => "darwin",
        os if os.contains("LINUX") => "linux",
        os if os.contains("FREEBSD") => "freebsd",
        os if os.contains("NETBSD") => "netbsd",
        os if os.contains("OPENBSD") => "openbsd",
        os if os.contains("WIN") || os.contains("MSYS") => "windows",
        _ => "",
    };
    OperatingSystem {
        name: os.to_string(),
        architecture: arch.to_string(),
    }
}

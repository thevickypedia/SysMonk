use std::process::Command;
use std::str;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct OperatingSystem {
    pub name: String,
    pub architecture: String,
}

fn unamem() -> String {
    // Get architecture using `uname -m` with fallback
    let uname_m_output = match Command::new("uname")
        .arg("-m")
        .output() {
        Ok(output) => output,
        Err(_) => {
            log::error!("Failed to read uname output");
            return "".to_string();
        },
    };
    let unamem = match str::from_utf8(&uname_m_output.stdout) {
        Ok(output) => output.trim().to_string(),
        Err(_) => {
            log::error!("Failed to read uname output");
            "".to_string()
        },
    };
    unamem.to_string()
}

fn unameu() -> String {
    // Get OS using `uname`
    let uname_output = match Command::new("uname")
        .output() {
        Ok(output) => output,
        Err(_) => {
            log::error!("Failed to read uname output");
            return std::env::consts::OS.to_uppercase();
        },
    };
    let unameu = match str::from_utf8(&uname_output.stdout) {
        Ok(output) => output.trim().to_string(),
        Err(_) => {
            log::error!("Failed to read uname output");
            std::env::consts::OS.to_uppercase()
        },
    };
    unameu.to_uppercase()
}

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

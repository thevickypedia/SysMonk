use crate::squire;
use serde_json::Value;
use std::collections::HashMap;

/// Function to get GPU information for macOS machines.
///
/// # Arguments
///
/// * `lib_path` - The path to the library used to get GPU information.
///
/// # Returns
///
/// A `Vec` of `HashMap` containing GPU(s) information if successful, otherwise an empty `Vec`.
fn get_gpu_info_darwin(lib_path: &str) -> Vec<HashMap<String, String>> {
    let result: Result<String, String> = squire::util::run_command(
        lib_path,
        &["SPDisplaysDataType", "-json"],
        true,
    );
    let displays: Vec<Value>;
    match result {
        Ok(json_str) => {
            match serde_json::from_str::<Value>(&json_str) {
                Ok(json) => {
                    if let Some(d) = json.get("SPDisplaysDataType").and_then(Value::as_array) {
                        displays = d.to_vec();
                    } else {
                        log::error!("Key 'SPDisplaysDataType' not found or is not an array.");
                        return Vec::new();
                    }
                }
                Err(err) => {
                    log::error!("Failed to parse JSON: {}", err);
                    return Vec::new();
                }
            }
        }
        Err(err) => {
            log::error!("Error retrieving result: {}", err);
            return Vec::new();
        }
    }
    let mut gpu_info = Vec::new();
    let na = "N/A".to_string();
    for display in displays {
        if let Some(model) = display.get("sppci_model") {
            let mut info = HashMap::new();
            info.insert(
                "model".to_string(),
                model.as_str()
                    .unwrap_or(na.as_str())
                    .to_string(),
            );

            // Handle cores
            info.insert(
                "cores".to_string(),
                display.get("sppci_cores")
                    .or(display.get("spdisplays_cores"))
                    .and_then(|v| v.as_str())
                    .unwrap_or(&na)
                    .to_string(),
            );

            // Handle memory
            info.insert(
                "memory".to_string(),
                display.get("sppci_vram")
                    .or(display.get("spdisplays_vram"))
                    .and_then(|v| v.as_str())
                    .unwrap_or(&na)
                    .to_string(),
            );

            // Handle vendor
            info.insert(
                "vendor".to_string(),
                display.get("sppci_vendor")
                    .and_then(|v| v.as_str())
                    .unwrap_or(&na)
                    .to_string(),
            );
            gpu_info.push(info);
        }
    }
    gpu_info
}

/// Function to get GPU information for Linux machines.
///
/// # Arguments
///
/// * `lib_path` - The path to the library used to get GPU information.
///
/// # Returns
///
/// A `Vec` of `HashMap` containing GPU(s) information if successful, otherwise an empty `Vec`.
fn get_gpu_info_linux(lib_path: &str) -> Vec<HashMap<String, String>> {
    let result = squire::util::run_command(
        lib_path,
        &[],
        true,
    );
    if result.is_err() {
        return Vec::new();
    }
    let mut gpu_info = Vec::new();
    for line in result.unwrap().lines() {
        if line.contains("VGA") {
            let gpu = line.split(':').last().unwrap().trim();
            let mut info = HashMap::new();
            info.insert("model".to_string(), gpu.to_string());
            gpu_info.push(info);
        }
    }
    gpu_info
}

/// Function to get GPU information for Windows machines.
///
/// # Arguments
///
/// * `lib_path` - The path to the library used to get GPU information.
///
/// # Returns
///
/// A `Vec` of `HashMap` containing GPU(s) information if successful, otherwise an empty `Vec`.
fn get_gpu_info_windows(lib_path: &str) -> Vec<HashMap<String, String>> {
    let result = squire::util::run_command(
        lib_path,
        &["path", "win32_videocontroller", "get", "Name,AdapterCompatibility", "/format:csv"],
        true,
    );
    let output = match result {
        Ok(output) => output.to_uppercase(),
        Err(_) => {
            return Vec::new();
        }
    };
    let gpus_raw: Vec<&str> = output.lines().filter(|line| !line.trim().is_empty()).collect();
    if gpus_raw.is_empty() {
        log::info!("No GPUs found!");
        return vec![];
    }

    let keys: Vec<String> = gpus_raw[0]
        .trim()
        .to_lowercase()
        .replace("adaptercompatibility", "vendor")
        .replace("name", "model")
        .split(',')
        .map(|key| key.to_string())
        .collect();

    let binding = gpus_raw[1..].join("");
    let values: Vec<&str> = binding.trim().split(",").collect();
    let mut gpu_info = Vec::new();
    let key_len = keys.len();
    for chunk in values.chunks(key_len) {
        let mut map = HashMap::new();
        for (key, value) in keys.iter().zip(chunk) {
            map.insert(key.to_string(), value.to_string());
        }
        gpu_info.push(map);
    }
    gpu_info
}

/// OS-agnostic function to get GPU name.
///
/// # Returns
///
/// A `Vec` of `HashMap` containing GPU(s) information if successful, otherwise an empty `Vec`.
pub fn get_gpu_info() -> Vec<HashMap<String, String>> {
    let operating_system = std::env::consts::OS;
    match operating_system {
        "macos" => get_gpu_info_darwin("/usr/sbin/system_profiler"),
        "linux" => get_gpu_info_linux("/usr/bin/lspci"),
        "windows" => get_gpu_info_windows("C:\\Windows\\System32\\wbem\\wmic.exe"),
        _ => {
            log::error!("Unsupported operating system: {}", operating_system);
            Vec::new()
        }
    }
}

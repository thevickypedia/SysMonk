use std::collections::HashMap;
use serde_json::Value;
use crate::squire;

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

fn get_gpu_info_darwin(lib_path: &str) -> Vec<HashMap<String, String>> {
    let result = squire::util::run_command(
        lib_path,
        &["SPDisplaysDataType", "-json"],
        true
    );
    if result.is_err() {
        return Vec::new();
    }
    let json: Value = serde_json::from_str(&result.unwrap()).unwrap();
    
    let displays = json["SPDisplaysDataType"].as_array().unwrap();
    let mut gpu_info = Vec::new();

    let na = "N/A".to_string();
    for display in displays {
        if let Some(model) = display.get("sppci_model") {
            let mut info = HashMap::new();
            info.insert(
                "model".to_string(),
                model.as_str()
                    .unwrap_or(na.as_str())
                    .to_string()
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

fn get_gpu_info_linux(lib_path: &str) -> Vec<HashMap<String, String>> {
    let result = squire::util::run_command(
        lib_path,
        &[],
        true
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

fn get_gpu_info_windows(lib_path: &str) -> Vec<HashMap<String, String>> {
    let result = squire::util::run_command(
        lib_path,
        &["path", "win32_videocontroller", "get", "Name,AdapterCompatibility", "/format:csv"],
        true
    );
    if result.is_err() {
        return Vec::new();
    }
    let stdout = result.unwrap();
    let gpus_raw: Vec<&str> = stdout.lines().filter(|line| !line.trim().is_empty()).collect();    
    if gpus_raw.is_empty() {
        return vec![];
    }

    let keys: Vec<String> = gpus_raw[0]
        .replace("Node", "node")
        .replace("AdapterCompatibility", "vendor")
        .replace("Name", "model")
        .split(',')
        .map(|key| key.to_string())
        .collect();

    let mut gpu_info = Vec::new();

    for values in gpus_raw[1..].chunks(keys.len()) {
        if values.len() == keys.len() {
            let mut info = HashMap::new();
            for (key, value) in keys.iter().zip(values.iter()) {
                info.insert(key.clone(), value.to_string());
            }
            gpu_info.push(info);
        }
    }
    gpu_info
}

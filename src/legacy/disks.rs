use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;
use std::str;

use crate::squire;

/// Function to parse size string for Linux.
///
/// # Arguments
///
/// * `size_str` - The size string to parse
///
/// # Returns
///
/// A `String` containing the parsed size string.
fn parse_size(size_str: &str) -> String {
    let re = Regex::new(r"([\d.]+)([KMGTP]?)").unwrap();
    if let Some(caps) = re.captures(size_str.trim()) {
        let value: f64 = caps[1].parse().unwrap();
        let unit = &caps[2];
        let unit_multipliers = HashMap::from([
            ("K", 2_f64.powi(10)),
            ("M", 2_f64.powi(20)),
            ("G", 2_f64.powi(30)),
            ("T", 2_f64.powi(40)),
            ("P", 2_f64.powi(50)),
        ]);
        let multiplier = unit_multipliers.get(unit).unwrap_or(&1.0);
        return squire::util::size_converter((value * multiplier) as u64);
    }
    size_str.replace("K", " KB")
        .replace("M", " MB")
        .replace("G", " GB")
        .replace("T", " TB")
        .replace("P", " PB")
}

/// Function to check if a disk is physical/virtual for macOS.
///
/// # Arguments
///
/// * `lib_path` - The path to the library
/// * `device_id` - The device ID
///
/// # Returns
///
/// A `bool` indicating if the disk is physical.
fn is_physical_disk(lib_path: &str, device_id: &str) -> bool {
    let result = squire::util::run_command(lib_path, &["info", device_id]);
    let output = match result {
        Ok(output) => output,
        Err(_) => {
            log::error!("Failed to get disk info");
            return false;
        }
    };
    for line in output.split("\n") {
        if line.contains("Virtual:") && line.contains("Yes") {
            return false;
        }
    }
    true
}

/// Function to get disk information on Linux.
///
/// # Arguments
///
/// * `lib_path` - The path to the library used to get disks' information.
///
/// # Returns
///
/// A `Vec` of `HashMap` containing the disk information.
fn linux_disks(lib_path: &str) -> Vec<HashMap<String, String>> {
    let result = squire::util::run_command(lib_path, &["-o", "NAME,SIZE,TYPE,MODEL", "-d"]);
    let output = match result {
        Ok(output) => output,
        Err(_) => {
            log::error!("Failed to get disk info");
            return Vec::new();
        }
    };
    let disks: Vec<&str> = output.lines().collect();
    let filtered_disks: Vec<&str> = disks.into_iter().filter(|&disk| !disk.contains("loop")).collect();
    let keys_raw = filtered_disks[0].to_lowercase()
        .replace("name", "DeviceID")
        .replace("model", "Name")
        .replace("size", "Size");
    let keys: Vec<&str> = keys_raw.split_whitespace().collect();
    let mut disk_info = Vec::new();
    for line in &filtered_disks[1..] {
        let values: Vec<&str> = line.split_whitespace().collect();
        let mut disk_map = HashMap::new();
        for (key, value) in keys.iter().zip(values.iter()) {
            disk_map.insert(key.to_string(), value.to_string());
        }
        disk_map.insert("Size".to_string(), parse_size(disk_map.get("Size").unwrap()));
        disk_map.remove("type");
        disk_info.push(disk_map);
    }
    disk_info
}

/// Function to get disk information on macOS.
///
/// # Arguments
///
/// * `lib_path` - The path to the library used to get disks' information.
///
/// # Returns
///
/// A `Vec` of `HashMap` containing the disk information.
fn darwin_disks(lib_path: &str) -> Vec<HashMap<String, String>> {
    let result = squire::util::run_command(lib_path, &["list"]);
    let output = match result {
        Ok(output) => output,
        Err(_) => {
            log::error!("Failed to get disk info");
            return Vec::new();
        }
    };
    let disks: Vec<&str> = output.lines().collect();
    let disk_lines: Vec<&str> = disks.into_iter().filter(|&line| line.starts_with("/dev/disk")).collect();
    let mut disk_info = Vec::new();
    for line in disk_lines {
        let device_id = line.split_whitespace().next().unwrap();
        if !is_physical_disk(lib_path, device_id) {
            continue;
        }
        let result = squire::util::run_command(lib_path, &["info", device_id]);
        let disk_info_output = match result {
            Ok(output) => output,
            Err(_) => {
                log::error!("Failed to get disk info");
                return Vec::new();
            }
        };
        let info_lines: Vec<&str> = disk_info_output.lines().collect();
        let mut disk_data = HashMap::new();
        for info_line in info_lines {
            if info_line.contains("Device / Media Name:") {
                disk_data.insert("Name".to_string(), info_line.split(":").nth(1).unwrap().trim().to_string());
            }
            if info_line.contains("Disk Size:") {
                let size_info = info_line.split(":").nth(1).unwrap().split("(").next().unwrap().trim().to_string();
                disk_data.insert("Size".to_string(), size_info);
            }
        }
        disk_data.insert("DeviceID".to_string(), device_id.to_string());
        disk_info.push(disk_data);
    }
    disk_info
}

/// Function to reformat disk information on Windows.
///
/// # Arguments
///
/// * `data` - A mutable reference to the disk information.
///
/// # Returns
///
/// A `HashMap` containing the reformatted disk information.
fn reformat_windows(data: &mut HashMap<String, Value>) -> HashMap<String, String> {
    let size = data.get("Size").unwrap().as_f64().unwrap();
    let model = data.get("Model").unwrap().as_str().unwrap().to_string();
    let mut reformatted_data = HashMap::new();
    reformatted_data.insert("Size".to_string(), squire::util::size_converter(size as u64));
    reformatted_data.insert("Name".to_string(), model);
    reformatted_data.insert("DeviceID".to_string(), data.get("DeviceID").unwrap().as_str().unwrap().to_string());
    reformatted_data
}

/// Function to get disk information on Windows.
///
/// # Arguments
///
/// * `lib_path` - The path to the library used to get disks' information.
///
/// # Returns
///
/// A `Vec` of `HashMap` containing the disk information.
fn windows_disks(lib_path: &str) -> Vec<HashMap<String, String>> {
    let ps_command = "Get-CimInstance Win32_DiskDrive | Select-Object Caption, DeviceID, Model, Partitions, Size | ConvertTo-Json";
    let result = squire::util::run_command(lib_path, &["-Command", ps_command]);
    let output = match result {
        Ok(output) => output,
        Err(_) => {
            log::error!("Failed to get disk info");
            return Vec::new();
        }
    };
    let disks_info: Value = serde_json::from_str(&output).unwrap();
    let mut disk_info = Vec::new();
    if let Some(disks) = disks_info.as_array() {
        for disk in disks {
            let mut disk_map: HashMap<String, Value> = serde_json::from_value(disk.clone()).unwrap();
            disk_info.push(reformat_windows(&mut disk_map));
        }
    } else {
        let mut disk_map: HashMap<String, Value> = serde_json::from_value(disks_info).unwrap();
        disk_info.push(reformat_windows(&mut disk_map));
    }
    disk_info
}

/// Function to get all disks' information.
///
/// # Returns
///
/// A `Vec` of `HashMap` containing the disk information.
pub fn get_all_disks() -> Vec<HashMap<String, String>> {
    let operating_system = std::env::consts::OS;
    match operating_system {
        "windows" => windows_disks("C:\\Program Files\\PowerShell\\7\\pwsh.exe"),
        "macos" => darwin_disks("/usr/sbin/diskutil"),
        "linux" => linux_disks("/usr/bin/lsblk"),
        _ => {
            log::error!("Unsupported operating system");
            Vec::new()
        }
    }
}

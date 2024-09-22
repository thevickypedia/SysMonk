use std::collections::HashMap;
use std::process::Command;
use std::str;
use regex::Regex;
use serde_json::Value;

// todo: tested only on macOS

fn format_nos(input: f64) -> f64 {
    if input.fract() == 0.0 {
        input.trunc()
    } else {
        input
    }
}

fn size_converter(byte_size: f64) -> String {
    let size_name = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    let index = (byte_size.log(1024.0)).floor() as usize;
    format!("{:.2} {}", format_nos(byte_size / 1024.0_f64.powi(index as i32)), size_name[index])
}

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
        return size_converter(value * multiplier);
    }
    size_str.replace("K", " KB")
        .replace("M", " MB")
        .replace("G", " GB")
        .replace("T", " TB")
        .replace("P", " PB")
}

fn run_command(command: &str, args: &[&str]) -> String {
    let output = Command::new(command)
        .args(args)
        .output()
        .expect("Failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn is_physical_disk(lib_path: &str, device_id: &str) -> bool {
    let output = run_command(lib_path, &["info", device_id]);
    // !output.contains("Virtual:Yes")
    for line in output.split("\n") {
        if line.contains("Virtual:") && line.contains("Yes") {
            return false;
        }
    }
    true
}

fn linux_disks(lib_path: &str) -> Vec<HashMap<String, String>> {
    let output = run_command(lib_path, &["-o", "NAME,SIZE,TYPE,MODEL", "-d"]);
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

fn darwin_disks(lib_path: &str) -> Vec<HashMap<String, String>> {
    let output = run_command(lib_path, &["list"]);
    let disks: Vec<&str> = output.lines().collect();
    let disk_lines: Vec<&str> = disks.into_iter().filter(|&line| line.starts_with("/dev/disk")).collect();
    let mut disk_info = Vec::new();
    for line in disk_lines {
        let device_id = line.split_whitespace().next().unwrap();
        if !is_physical_disk(lib_path, device_id) {
            continue;
        }
        let disk_info_output = run_command(lib_path, &["info", device_id]);
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

fn reformat_windows(data: &mut HashMap<String, Value>) -> HashMap<String, String> {
    let size = data.get("Size").unwrap().as_f64().unwrap();
    let model = data.get("Model").unwrap().as_str().unwrap().to_string();
    let mut reformatted_data = HashMap::new();
    reformatted_data.insert("Size".to_string(), size_converter(size));
    reformatted_data.insert("Name".to_string(), model);
    reformatted_data.insert("DeviceID".to_string(), data.get("DeviceID").unwrap().as_str().unwrap().to_string());
    reformatted_data
}

fn windows_disks(lib_path: &str) -> Vec<HashMap<String, String>> {
    let ps_command = "Get-CimInstance Win32_DiskDrive | Select-Object Caption, DeviceID, Model, Partitions, Size | ConvertTo-Json";
    let output = run_command(lib_path, &["-Command", ps_command]);
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

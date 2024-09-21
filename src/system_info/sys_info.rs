use chrono::Utc;
use serde::{Deserialize, Serialize};
use sysinfo::{DiskExt, System, SystemExt};

// Helper function to format the duration
fn convert_seconds(seconds: i64) -> String {
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

// Helper function to convert size
fn size_converter(byte_size: u64) -> String {
    let size_name = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    let mut index = 0;
    let mut size = byte_size as f64;

    while size >= 1024.0 && index < size_name.len() - 1 {
        size /= 1024.0;
        index += 1;
    }

    format!("{:.2} {}", size, size_name[index])
}

// Struct to hold system information in a JSON-serializable format
#[derive(Serialize, Deserialize, Debug)]
pub struct SystemInfoBasic {
    node: String,
    system: String,
    architecture: String,
    cpu_cores: String,
    uptime: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemInfoMemStorage {
    memory: String,
    swap: String,
    storage: String,
}


// Function to collect system information and return as a JSON-serializable struct
pub fn get_sys_info() -> (SystemInfoBasic, SystemInfoMemStorage) {
    let mut sys = System::new_all();
    sys.refresh_all();

    // Uptime
    let boot_time = sys.boot_time();
    let uptime_duration = Utc::now().timestamp() - boot_time as i64;
    let uptime = convert_seconds(uptime_duration);

    let total_memory = size_converter(sys.total_memory());  // in bytes
    let total_swap = size_converter(sys.total_swap());  // in bytes
    let total_storage = size_converter(sys.disks().iter().map(|disk| disk.total_space()).sum::<u64>());

    // Basic and Memory/Storage Info
    let basic = SystemInfoBasic {
        node: sys.host_name().unwrap_or_else(|| "Unknown".to_string()),
        system: sys.name().unwrap_or_else(|| "Unknown".to_string()),
        architecture: std::env::consts::ARCH.to_string(),
        uptime,
        cpu_cores: sys.cpus().len().to_string(),
    };
    let mem_storage = SystemInfoMemStorage {
        memory: total_memory,
        swap: total_swap,
        storage: total_storage,
    };
    (basic, mem_storage)
}

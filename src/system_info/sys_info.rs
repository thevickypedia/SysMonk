use chrono::Utc;
use serde::{Deserialize, Serialize};
use sysinfo::{DiskExt, System, SystemExt};
use crate::squire;

/// Struct to hold basic system information
///
/// This struct holds basic system information such as the node name, system name, architecture, CPU cores, and uptime.
///
/// # Fields
///
/// * `node` - The node name of the system
/// * `system` - The system name
/// * `architecture` - The system architecture
/// * `cpu_cores` - The number of CPU cores
/// * `uptime` - The system uptime
#[derive(Serialize, Deserialize, Debug)]
pub struct SystemInfoBasic {
    node: String,
    system: String,
    architecture: String,
    cpu_cores: String,
    uptime: String,
}

/// Struct to hold memory and storage information
///
/// This struct holds memory and storage information such as the total memory, total swap, and total storage.
///
/// # Fields
///
/// * `memory` - The total memory of the system
/// * `swap` - The total swap space of the system
/// * `storage` - The total storage space of the system
#[derive(Serialize, Deserialize, Debug)]
pub struct SystemInfoMemStorage {
    memory: String,
    swap: String,
    storage: String,
}

/// Function to get system information
///
/// This function retrieves system information such as basic system information and memory/storage information.
///
/// # Returns
///
/// A tuple containing the `SystemInfoBasic` and `SystemInfoMemStorage` structs.
pub fn get_sys_info() -> (SystemInfoBasic, SystemInfoMemStorage) {
    let mut sys = System::new_all();
    sys.refresh_all();

    // Uptime
    let boot_time = sys.boot_time();
    let uptime_duration = Utc::now().timestamp() - boot_time as i64;
    let uptime = squire::util::convert_seconds(uptime_duration);

    let total_memory = squire::util::size_converter(sys.total_memory());  // in bytes
    let total_swap = squire::util::size_converter(sys.total_swap());  // in bytes
    let total_storage = squire::util::size_converter(sys.disks().iter().map(|disk| disk.total_space()).sum::<u64>());

    // Basic and Memory/Storage Info
    let basic = SystemInfoBasic {
        node: sys.host_name().unwrap_or("Unknown".to_string()),
        system: sys.name().unwrap_or("Unknown".to_string()),
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

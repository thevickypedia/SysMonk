use std::collections::HashMap;
use chrono::Utc;
use inflector::cases::titlecase::to_title_case;
use sysinfo::{DiskExt, System, SystemExt};
use crate::{squire, resources};

/// Function to get total disk usage.
///
/// # Arguments
///
/// * `system` - A reference to the `System` struct.
///
/// # Returns
///
/// A `u64` value containing the total disk usage.
pub fn get_disk_usage(system: &System) -> u64 {
    let mut disks_space = vec![];
    for disk in system.disks() {
        disks_space.push(disk.total_space());
    }
    disks_space.iter().sum()
}

/// Function to get system information
///
/// This function retrieves system information such as basic system information and memory/storage information.
///
/// # Returns
///
/// A tuple containing the `SystemInfoBasic` and `SystemInfoMemStorage` structs.
pub fn get_sys_info() -> HashMap<&'static str, HashMap<&'static str, String>> {
    let mut sys = System::new_all();
    sys.refresh_all();

    // Uptime
    let boot_time = sys.boot_time();
    let uptime_duration = Utc::now().timestamp() - boot_time as i64;
    let uptime = squire::util::convert_seconds(uptime_duration);

    let total_memory = squire::util::size_converter(sys.total_memory());  // in bytes
    let total_storage = squire::util::size_converter(get_disk_usage(&sys));  // in bytes

    // Basic and Memory/Storage Info
    let os_arch = resources::system::os_arch();
    let basic = HashMap::from_iter(vec![
        ("node", sys.host_name().unwrap_or("Unknown".to_string())),
        ("system", to_title_case(&os_arch.name)),
        ("architecture", os_arch.architecture),
        ("uptime", uptime),
        ("CPU_cores_raw", sys.cpus().len().to_string()
    )]);
    let mut hash_vec = vec![
        ("memory", total_memory),
        ("storage", total_storage)
    ];

    let total_swap = sys.total_swap();  // in bytes
    if total_swap != 0 {
        hash_vec.push(("swap", squire::util::size_converter(total_swap)));
    }
    let mem_storage = HashMap::from_iter(hash_vec);
    HashMap::from_iter(vec![
        ("basic", basic),
        ("mem_storage", mem_storage)
    ])
}

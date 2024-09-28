use crate::{legacy, resources, squire};
use chrono::Utc;
use std::collections::HashMap;
use std::collections::HashSet;
use sysinfo::Disks;
use sysinfo::System;

/// Function to get total disk usage.
///
/// # Returns
///
/// A `u64` value containing the total disk usage.
pub fn get_disk_usage(disks: &Disks) -> u64 {
    let mut disks_space: Vec<u64> = vec![];
    for disk in disks.list() {
        disks_space.push(disk.total_space());
    }
    disks_space.iter().sum()
}

/// Function to get individual disk specs.
///
/// # Returns
///
/// A `vec` of .
pub fn get_disks(disks: &Disks) -> Vec<HashMap<String, String>> {
    let mut disks_info = vec![];
    for disk in disks.list() {
        disks_info.push(
            HashMap::from([
                ("Name".to_string(), disk.name().to_string_lossy().to_string()),
                ("Size".to_string(), squire::util::size_converter(disk.total_space()).to_string()),
                ("Kind".to_string(), disk.file_system().to_string_lossy().to_string()),
                ("Mount Point".to_string(), disk.mount_point().to_string_lossy().to_string()),
            ])
        );
    }
    disks_info
}


fn get_gpu_info() -> Vec<String> {
    let gpu_info = legacy::gpu::get_gpu_info();
    log::debug!("GPUs: {:?}", gpu_info);
    let mut gpus: Vec<String> = vec![];
    for gpu in gpu_info {
        if let Some(gpu_model) = gpu.get("model") {
            gpus.push(gpu_model.to_string());
        }
    }
    gpus
}

/// Function to get CPU brand names as a comma separated string.
///
/// # Arguments
///
/// * `system` - A reference to the `System` struct.
///
/// # Returns
///
/// A `String` with CPU brand names.
fn get_cpu_brand(sys: &System) -> String {
    let mut cpu_brands: HashSet<String> = HashSet::new();
    let cpus = sys.cpus();
    for cpu in cpus {
        cpu_brands.insert(cpu.brand().to_string());
    }
    if cpu_brands.is_empty() {
        let legacy_cpu_brand_name = legacy::cpu::get_name();
        return if let Some(cpu_brand) = legacy_cpu_brand_name {
            log::debug!("Using legacy methods for CPU brand!!");
            cpu_brand
        } else {
            log::error!("Unable to get brand information for all {} CPUs", cpus.len());
            "Unknown".to_string()
        };
    }
    let mut cpu_brand_list: Vec<String> = cpu_brands.into_iter().collect();
    cpu_brand_list.sort_by_key(|brand| brand.len());
    match cpu_brand_list.len() {
        0 => String::new(),
        1 => cpu_brand_list[0].clone(),
        2 => format!("{} and {}", cpu_brand_list[0], cpu_brand_list[1]),
        _ => {
            let last = cpu_brand_list.pop().unwrap(); // Remove and store the last element
            format!("{}, and {}", cpu_brand_list.join(", "), last)
        }
    }
}

/// Function to get system information
///
/// This function retrieves system information such as basic system information and memory/storage information.
///
/// # Returns
///
/// A tuple containing the `SystemInfoBasic` and `SystemInfoMemStorage` structs.
pub fn get_sys_info(disks: &Disks) -> HashMap<&'static str, HashMap<&'static str, String>> {
    let mut sys = System::new_all();
    sys.refresh_all();

    // Uptime
    let boot_time = System::boot_time();
    let uptime_duration = Utc::now().timestamp() - boot_time as i64;
    let uptime = squire::util::convert_seconds(uptime_duration);

    let total_memory = squire::util::size_converter(sys.total_memory());  // in bytes
    let total_storage = squire::util::size_converter(get_disk_usage(disks));  // in bytes

    // Basic and Memory/Storage Info
    let os_arch = resources::system::os_arch();
    let mut basic = HashMap::from_iter(vec![
        ("Hostname", System::host_name().unwrap_or("Unknown".to_string())),
        ("Operating System", squire::util::capwords(&os_arch.name, None)),
        ("Architecture", os_arch.architecture),
        ("Uptime", uptime),
        ("CPU cores", sys.cpus().len().to_string()),
        ("CPU brand", get_cpu_brand(&sys))
    ]);
    let gpu_info = get_gpu_info();
    if !gpu_info.is_empty() {
        let key = if gpu_info.len() == 1 { "GPU" } else { "GPUs" };
        basic.insert(key, gpu_info.join(", "));
    }
    let mut hash_vec = vec![
        ("Memory", total_memory),
        ("Storage", total_storage)
    ];

    let total_swap = sys.total_swap();  // in bytes
    if total_swap != 0 {
        hash_vec.push(("Swap", squire::util::size_converter(total_swap)));
    }
    let mem_storage = HashMap::from_iter(hash_vec);
    HashMap::from_iter(vec![
        ("basic", basic),
        ("mem_storage", mem_storage)
    ])
}

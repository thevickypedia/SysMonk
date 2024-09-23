use std::collections::HashMap;
use sysinfo::{CpuExt, System, SystemExt};


use serde_json;
use crate::squire;

/// Function to get docker stats via commandline.
///
/// # Returns
///
/// A `Result` containing a `Vec` of `serde_json::Value` if successful, otherwise an empty `Vec`.
fn get_docker_stats() -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
    // Check if there are any docker containers running
    // `docker -a` will show all containers including stopped, which will block `docker stats`
    let ps_result = squire::util::run_command("docker", &["ps", "-q"]);
    let stats_result = match ps_result {
        Ok(output) if !output.is_empty() => {
            let stats_result = squire::util::run_command(
                "docker",
                &["stats", "--no-stream", "--format", "{{json .}}"]
            );
            match stats_result {
                Ok(stats) => stats,
                Err(err) => {
                    log::error!("Error running docker stats: {}", err);
                    return Ok(vec![]);
                },
            }
        }
        Ok(_) => {
            log::debug!("No running containers");
            return Ok(vec![]);
        },
        Err(err) => {
            log::error!("Error checking containers: {}", err);
            return Ok(vec![]);
        },
    };
    let stats: Vec<serde_json::Value> = stats_result
        .lines()
        .filter_map(|line| serde_json::from_str(line).ok())
        .collect();
    Ok(stats)
}

/// Function to get CPU usage percentage.
///
/// # Returns
///
/// A `Vec` containing the CPU usage percentage of each core.
fn get_cpu_percent() -> Vec<String> {
    let mut system = System::new_all();
    system.refresh_all();
    let mut cpu_usage = Vec::new();
    for core in system.cpus() {
        cpu_usage.push(format!("{:.2}", core.cpu_usage()));
    }
    cpu_usage
}

/// Function to get system metrics.
///
/// # Returns
///
/// A `HashMap` containing the system metrics with CPU load average, memory and swap usage.
fn get_system_metrics() -> HashMap<String, serde_json::Value> {
    let mut system = System::new_all();
    system.refresh_all();

    let load_avg = system.load_average();

    // used_memory uses "mem_total - mem_free" but memory is set to available instead of free in macOS
    let mut hash_vec = vec![
        (
            "memory_info".to_string(),
            serde_json::json!({
                "total": system.total_memory(),
                "used": system.total_memory() - system.available_memory(),
            }),
        ),
        (
            "load_averages".to_string(),
            serde_json::json!({
                "m1": load_avg.one,
                "m5": load_avg.five,
                "m15": load_avg.fifteen,
            }),
        ),
    ];

    let total_swap = system.total_swap();
    if total_swap != 0 {
        hash_vec.push((
            "swap_info".to_string(),
            serde_json::json!({
                "total": total_swap,
                "used": system.used_swap(),
            }),
        ));
    }
    HashMap::from_iter(hash_vec)
}


/// Function to get the system information.
///
/// # Returns
///
/// A `HashMap` containing the system information with basic system information and memory/storage information.
pub fn system_resources() -> HashMap<String, serde_json::Value> {
    let mut system_metrics = get_system_metrics();
    let cpu_percent = get_cpu_percent();
    let docker_stats = get_docker_stats().unwrap();
    system_metrics.insert("cpu_usage".to_string(), serde_json::json!(cpu_percent));
    system_metrics.insert("docker_stats".to_string(), serde_json::json!(docker_stats));
    system_metrics
}

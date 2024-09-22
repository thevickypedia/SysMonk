use std::collections::HashMap;
use sysinfo::{CpuExt, System, SystemExt};


use serde_json;
use std::process::{Command, Stdio};

fn get_docker_stats() -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
    let process = Command::new("docker")
        .args(&["stats", "--no-stream", "--format", "{{json .}}"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let output = process.wait_with_output().unwrap();
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error: {}", stderr);
        return Ok(vec![]);
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stats: Vec<serde_json::Value> = stdout
        .lines()
        .filter_map(|line| serde_json::from_str(line).ok())
        .collect();
    Ok(stats)
}

fn get_cpu_percent() -> Vec<String> {
    let mut system = System::new_all();
    system.refresh_all();
    let mut cpu_usage = Vec::new();
    for core in system.cpus() {
        cpu_usage.push(format!("{:.2}", core.cpu_usage()));
    }
    cpu_usage
}

fn get_system_metrics() -> HashMap<String, serde_json::Value> {
    let mut system = System::new_all();
    system.refresh_all();

    let load_avg = system.load_average();

    let mut hash_vec = vec![
        (
            "memory_info".to_string(),
            serde_json::json!({
                "total": system.total_memory(),
                "used": system.used_memory(),  // todo: wildly inaccurate (always 99%) on macOS
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
    let info = HashMap::from_iter(hash_vec);
    info
}


pub fn system_resources() -> HashMap<String, serde_json::Value> {
    let mut system_metrics = get_system_metrics();
    let cpu_percent = get_cpu_percent();
    let docker_stats = get_docker_stats().unwrap();
    system_metrics.insert("cpu_usage".to_string(), serde_json::json!(cpu_percent));
    system_metrics.insert("docker_stats".to_string(), serde_json::json!(docker_stats));
    system_metrics
}

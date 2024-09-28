use serde::{Deserialize, Serialize};
use crate::squire;
use sysinfo::{Pid, ProcessesToUpdate, System};

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    name: String,
    pid: u32,
    cpu: String,
    memory: String,
    uptime: String,
    read_io: String,
    write_io: String
}

pub fn process_monitor(system: &mut System, process_names: &Vec<String>) -> Vec<Usage> {
    let mut usages: Vec<Usage> = Vec::new();
    system.refresh_processes(ProcessesToUpdate::All);
    for (pid, process) in system.processes() {
        let process_name = process.name().to_str().unwrap().to_string();
        if process_names.iter().any(|given_name| process_name.contains(given_name)) {
            let cpu_usage = process.cpu_usage();
            let memory_usage = process.memory();
            let cpu = format!("{:.2}%", cpu_usage);
            let memory = squire::util::size_converter(memory_usage);
            let pid_32 = pid.as_u32();

            let uptime = squire::util::convert_seconds(process.run_time() as i64);
            let disk_usage = process.disk_usage();
            let written = squire::util::size_converter(disk_usage.total_written_bytes);
            let written_since = squire::util::size_converter(disk_usage.written_bytes);
            let read = squire::util::size_converter(disk_usage.total_read_bytes);
            let read_since = squire::util::size_converter(disk_usage.read_bytes);

            usages.push(Usage {
                name: process_name,
                pid: pid_32,
                cpu,
                memory,
                uptime,
                read_io: format!("{}/{}", read_since, read),
                write_io: format!("{}/{}", written_since, written)
            });
        }
    }
    usages
}

pub fn service_monitor(system: &mut System, service_names: &Vec<String>) -> Vec<Usage> {
    let mut usages: Vec<Usage> = Vec::new();
    system.refresh_processes(ProcessesToUpdate::All);
    for service_name in service_names {
        match service_monitor_fn(system, &service_name) {
            Ok(usage) => usages.push(usage),
            Err(err) => {
                log::debug!("{}", err);
                usages.push(Usage {
                    name: service_name.to_string(),
                    pid: 0000,
                    memory: "N/A".to_string(),
                    cpu: "N/A".to_string(),
                    uptime: "N/A".to_string(),
                    read_io: "N/A".to_string(),
                    write_io: "N/A".to_string()
                });
            }
        };
    }
    usages
}

fn service_monitor_fn(system: &System, service_name: &String) -> Result<Usage, String> {
    let pid = match get_service_pid(service_name) {
        Some(pid) => pid,
        None => return Err(format!("Failed to get PID for service: {}", service_name)),
    };
    let sys_pid: Pid = Pid::from(pid as usize);
    if let Some(process) = system.process(sys_pid) {
        let cpu_usage = process.cpu_usage();
        let memory_usage = process.memory();
        let cpu = format!("{:.2}%", cpu_usage);
        let memory = squire::util::size_converter(memory_usage);
        let pid_32 = sys_pid.as_u32();

        let uptime = squire::util::convert_seconds(process.run_time() as i64);
        let disk_usage = process.disk_usage();
        let written = squire::util::size_converter(disk_usage.total_written_bytes);
        let written_since = squire::util::size_converter(disk_usage.written_bytes);
        let read = squire::util::size_converter(disk_usage.total_read_bytes);
        let read_since = squire::util::size_converter(disk_usage.read_bytes);

        Ok(Usage {
            name: service_name.to_string(),
            pid: pid_32,
            cpu,
            memory,
            uptime,
            read_io: format!("{}/{}", read_since, read),
            write_io: format!("{}/{}", written_since, written)
        })
    } else {
        Err(format!("Process with PID {} not found", pid))
    }
}

/// Function to get PID of a service (OS-agnostic)
///
/// # See Also
///
/// Service names are case-sensitive, so use the following command to get the right name.
///
/// * macOS: `launchctl list | grep {{ service_name }}`
/// * Linux: `systemctl show {{ service_name }} --property=MainPID`
/// * Windows: `sc query {{ service_name }}`
fn get_service_pid(service_name: &str) -> Option<i32> {
    let operating_system = std::env::consts::OS;
    match operating_system {
        "macos" => get_service_pid_macos(service_name, "/bin/launchctl"),
        "linux" => get_service_pid_linux(service_name, "/usr/bin/systemctl"),
        "windows" => get_service_pid_windows(service_name, "C:\\Windows\\System32\\sc.exe"),
        _ => {
            log::error!("Unsupported operating system: {}", operating_system);
            None
        }
    }
}

// Linux: Use systemctl to get the service PID
fn get_service_pid_linux(service_name: &str, lib_path: &str) -> Option<i32> {
    let result = squire::util::run_command(
        lib_path,
        &["show", service_name, "--property=MainPID"],
        true,
    );
    let output = match result {
        Ok(output) => output,
        Err(_) => return None,
    };
    if let Some(line) = output.lines().find(|line| line.starts_with("MainPID=")) {
        if let Some(pid_str) = line.split('=').nth(1) {
            if let Ok(pid) = pid_str.trim().parse::<i32>() {
                return Some(pid);
            }
        }
    }
    None
}

// macOS: Use launchctl to get the service PID
fn get_service_pid_macos(service_name: &str, lib_path: &str) -> Option<i32> {
    let result = squire::util::run_command(
        lib_path,
        &["list"],
        true,
    );
    let output = match result {
        Ok(output) => output,
        Err(_) => return None,
    };
    for line in output.lines() {
        if line.contains(service_name) {
            // Split the line and extract the PID (usually first column)
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Ok(pid) = parts[0].parse::<i32>() {
                return Some(pid);
            }
        }
    }
    None
}

// Windows: Use sc query or PowerShell to get the service PID
fn get_service_pid_windows(service_name: &str, lib_path: &str) -> Option<i32> {
    let result = squire::util::run_command(
        lib_path,
        &["query", service_name],
        true,
    );
    let output = match result {
        Ok(output) => output,
        Err(_) => return None,
    };
    if let Some(line) = output.lines().find(|line| line.contains("PID")) {
        if let Some(pid_str) = line.split(':').nth(1) {
            if let Ok(pid) = pid_str.trim().parse::<i32>() {
                return Some(pid);
            }
        }
    }
    None
}

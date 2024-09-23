use crate::{resources, squire};
use std::fs::File;
use std::io::{self, BufRead};

fn get_processor_info_darwin(lib_path: &str) -> Result<String, &'static str> {
    let result = squire::util::run_command(lib_path, &["-n", "machdep.cpu.brand_string"]);
    if result.is_err() {
        return Err("Failed to get processor info");
    }
    Ok(result.unwrap())
}

fn get_processor_info_linux(lib_path: &str) -> Result<String, &'static str> {
    let file = match File::open(lib_path) {
        Ok(file) => file,
        Err(_) => return Err("Failed to open file"),
    };
    for line in io::BufReader::new(file).lines() {
        match line {
            Ok(line) => {
                if line.contains("model name") {
                    let parts: Vec<&str> = line.split(':').collect();
                    if parts.len() == 2 {
                        return Ok(parts[1].trim().to_string());
                    }
                }
            }
            Err(_) => return Err("Error reading line"),
        }
    }
    Err("Model name not found")
}
fn get_processor_info_windows(lib_path: &str) -> Result<String, &'static str> {
    let result = squire::util::run_command(lib_path, &["cpu", "get", "name"]);
    let output = match result {
        Ok(output) => output,
        Err(_) => return Err("Failed to get processor info"),
    };
    let lines: Vec<&str> = output.trim().split('\n').collect();
    if lines.len() > 1 {
        Ok(lines[1].trim().to_string())
    } else {
        Err("Invalid output from command")
    }
}

pub fn get_name() -> Option<String> {
    let os = resources::system::os_arch().name;
    let result = match os.as_str() {
        "darwin" => get_processor_info_darwin("/usr/sbin/sysctl"),
        "linux" => get_processor_info_linux("/proc/cpuinfo"),
        "windows" => get_processor_info_windows("C:\\Windows\\System32\\wbem\\wmic.exe"),
        _ => {
            log::error!("Unsupported operating system: {}", os);
            Err("Unsupported operating system")
        },
    };
    match result {
        Ok(info) => Some(info),
        Err(err) => {
            log::error!("{}", err);
            None
        }
    }
}

use std::process::Command;

/// Function to convert byte size to human-readable format
///
/// # Arguments
///
/// * `byte_size` - The size in bytes to convert
///
/// # Returns
///
/// A `String` containing the human-readable format of the byte size
pub fn size_converter(byte_size: u64) -> String {
    let size_name = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    let mut index = 0;
    let mut size = byte_size as f64;

    while size >= 1024.0 && index < size_name.len() - 1 {
        size /= 1024.0;
        index += 1;
    }

    format!("{:.2} {}", size, size_name[index])
}


/// Function to parse size string.
///
/// # Arguments
///
/// * `size_str` - The size string to parse
/// * `unit` - The unit to convert the size to
///
/// # Returns
///
/// A `String` containing the parsed size string.
pub fn run_command(command: &str, args: &[&str]) -> Result<String, String> {
    match Command::new(command)
        .args(args)
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                log::debug!("Command [{}] executed successfully", &command);
                Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                let exit_code = output.status.code().unwrap_or(-1);
                log::error!("Command [{}] failed with exit code: {}", command, exit_code);
                log::error!("Stderr: {}", stderr);
                Err(stderr)
            }
        }
        Err(err) => {
            log::error!("Failed to execute command: {}", err);
            Err(err.to_string())
        }
    }
}

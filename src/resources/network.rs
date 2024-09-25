use crate::squire;
use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::UdpSocket;

/// Function to retrieve the public IP address
///
/// # Returns
///
/// An `Option` containing the public IP address as a `String` if found, otherwise `None`
async fn public_ip_address() -> Option<String> {
    let ip_regex = squire::util::ip_regex();
    let mapping = squire::util::public_ip_mapping();

    for (url, expects_text) in mapping {
        match reqwest::get(&url).await {
            Ok(response) => {
                let extracted_ip = if expects_text {
                    response.text().await.unwrap_or_default().trim().to_string()
                } else {
                    response.json::<serde_json::Value>().await.ok()
                        .and_then(|json| json["origin"].as_str().map(str::to_string))
                        .unwrap_or_default()
                };

                if ip_regex.is_match(&extracted_ip) {
                    return Some(extracted_ip);
                }
            }
            Err(err) => {
                log::error!("Failed to fetch from {}: {}", &url, err);
                continue; // Move on to the next URL
            }
        }
    }

    None
}

/// Function to retrieve the private IP address
///
/// # Returns
///
/// An `Option` containing the private IP address as a `String` if found, otherwise `None`
fn private_ip_address() -> Option<String> {
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(err) => {
            log::error!("Failed to bind to a socket: {}", err);
            return None;
        }
    };
    if socket.connect("8.8.8.8:80").is_err() {
        log::error!("Failed to connect to a socket");
        return None;
    }
    let local_addr = match socket.local_addr() {
        Ok(addr) => addr,
        Err(err) => {
            log::error!("Failed to get local IP address: {}", err);
            return None;
        }
    };
    Some(local_addr.ip().to_string())
}

/// Struct to hold the network information of the system
///
/// This struct holds the private and public IP addresses of the system.
///
/// # Fields
///
/// * `private_ip_address` - The private IP address of the system
/// * `public_ip_address` - The public IP address of the system
#[derive(Serialize, Deserialize, Debug)]
pub struct SystemInfoNetwork {
    private_ip_address_raw: String,
    public_ip_address_raw: String,
}

/// Function to get network information
///
/// This function retrieves the private and public IP addresses of the system.
pub async fn get_network_info() -> HashMap<&'static str, String> {
    let private_ip = private_ip_address().unwrap_or_default();
    let public_ip = public_ip_address().await.unwrap_or_default();
    HashMap::from([
        ("Private IP address", private_ip),
        ("Public IP address", public_ip),
    ])
}

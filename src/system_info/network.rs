use regex::Regex;
use reqwest;
use serde::{Deserialize, Serialize};
use std::net::{SocketAddr, UdpSocket};

// Define the IP regex pattern
fn ip_regex() -> Regex {
    Regex::new(
        r"^(?:(?:25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9][0-9]|[0-9])\.){3}(?:25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9][0-9]|[0-9])$"
    ).unwrap()
}

// Synchronous function to retrieve the public IP address
async fn public_ip_address() -> Option<String> {
    let ip_regex = ip_regex();

    // Mapping URLs to their expected response types
    let mapping: Vec<(&str, bool)> = vec![
        ("https://checkip.amazonaws.com/", true), // expects text
        ("https://api.ipify.org/", true),         // expects text
        ("https://ipinfo.io/ip/", true),          // expects text
        ("https://v4.ident.me/", true),           // expects text
        ("https://httpbin.org/ip", false),        // expects JSON
        ("https://myip.dnsomatic.com/", true),    // expects text
    ];

    for (url, expects_text) in mapping {
        match reqwest::get(url).await {
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
            Err(e) => {
                eprintln!("Failed to fetch from {}: {}", url, e);
                continue; // Move on to the next URL
            }
        }
    }

    None
}

// Function to retrieve the private IP address
fn private_ip_address() -> Option<String> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    let local_addr: SocketAddr = socket.local_addr().ok()?;
    Some(local_addr.ip().to_string())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemInfoNetwork {
    private_ip_address: String,
    public_ip_address: String,
}


pub async fn get_network_info() -> SystemInfoNetwork {
    let private_ip = private_ip_address().unwrap_or_default();
    let public_ip = public_ip_address().await.unwrap_or_default();
    SystemInfoNetwork {
        private_ip_address: private_ip,
        public_ip_address: public_ip,
    }
}

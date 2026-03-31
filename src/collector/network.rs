use sysinfo::{Networks};

use crate::api::routes::ConnectionCore;

pub fn get_network() -> Vec<(String, u64, u64)> {
    let networks = Networks::new_with_refreshed_list();

    networks.iter().map(|(name, data)| {
      let received = data.received();
      let transmitted = data.transmitted();

      (name.to_string(), received, transmitted)  
    }).collect()
}

pub fn get_connections() -> Vec<ConnectionCore> {
    let content = std::fs::read_to_string("/proc/net/tcp").unwrap_or_default();

    content.lines()
        .skip(1)
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();

            let local = parts.get(1)?;
            let mut split = local.split(":");

            let ip_hex = split.next()?;
            let port_hex = split.next()?;

            Some(ConnectionCore {
                local_ip: hex_to_ip(ip_hex),
                local_port: hex_to_port(port_hex),
            })
        })
        .collect()
}

fn hex_to_ip(hex: &str) -> String {
    let bytes = (0..4)
        .map(|i| u8::from_str_radix(&hex[i*2..i*2+2], 16).unwrap())
        .collect::<Vec<u8>>();

    format!("{}.{}.{}.{}", bytes[3], bytes[2], bytes[1], bytes[0])
}

fn hex_to_port(hex: &str) -> u16 {
    u16::from_str_radix(hex, 16).unwrap()
}
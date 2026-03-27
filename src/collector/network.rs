use sysinfo::{Networks};
use std::fs;

pub fn get_network() -> Vec<(String, u64, u64)> {
    let networks = Networks::new_with_refreshed_list();

    networks.iter().map(|(name, data)| {
      let received = data.received();
      let transmitted = data.transmitted();

      (name.to_string(), received, transmitted)  
    }).collect()
}

pub fn get_connections() -> Vec<String> {
    let content = fs::read_to_string("/proc/net/tcp").unwrap_or_default();

    content.lines()
        .skip(1)
        .map(|line| line.to_string())
        .collect()
}
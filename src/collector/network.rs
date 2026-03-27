use sysinfo::{Networks};

pub fn get_network() -> Vec<(String, u64, u64)> {
    let networks = Networks::new_with_refreshed_list();

    networks.iter().map(|(name, data)| {
      let received = data.received();
      let transmitted = data.transmitted();

      (name.to_string(), received, transmitted)  
    }).collect()
}
use sysinfo::{System};

pub fn get_processes() -> Vec<(u32, String, f32, u64)> {
    let mut sys = System::new_all();
    sys.refresh_all();

    sys.processes()
        .iter()
        .map(|(pid, process)| {
            (
                pid.as_u32(),
                process.name().to_string(),
                process.cpu_usage(),
                process.memory(),
            )
        })
        .collect()
}
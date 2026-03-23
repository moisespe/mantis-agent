use sysinfo::System;

pub fn get_metrics() -> (f32, u64, u64) {
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu = sys.global_cpu_info().cpu_usage();
    let used_mem = sys.used_memory();
    let total_mem = sys.total_memory();

    (cpu, used_mem, total_mem)
}
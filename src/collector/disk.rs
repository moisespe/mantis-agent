use sysinfo::Disks;

pub fn get_disks() -> Vec<(String, u64, u64)> {
    let disks = Disks::new_with_refreshed_list();

    disks.iter().map(|disk| {
            let name = disk.name().to_string_lossy().to_string();
            let total = disk.total_space();
            let available = disk.available_space();

        (name, total, available)
    }).collect()
}

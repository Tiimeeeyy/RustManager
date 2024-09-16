use sysinfo::System;
use crate::process_info;
use rayon::prelude::*;

pub(crate) fn fetch_system_info() -> (Vec<process_info::ProcessInfo>, f32, u64) {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut processes: Vec<process_info::ProcessInfo> = sys.processes()
        .iter()
        .par_bridge()
        .map(|(&pid, process)| process_info::ProcessInfo {
            pid,
            name: process.name().to_str().unwrap().parse().unwrap(),
            cpu_usage: process.cpu_usage(),
            memory_usage: process.memory(),
            status: match process.status() {
                sysinfo::ProcessStatus::Run => process_info::ProcessStatus::Running,
                sysinfo::ProcessStatus::Sleep => process_info::ProcessStatus::Sleeping,
                sysinfo::ProcessStatus::Stop => process_info::ProcessStatus::Stopped,
                sysinfo::ProcessStatus::Zombie => process_info::ProcessStatus::Zombie,
                _ => process_info::ProcessStatus::Running,
            },
        })
        .collect();
    processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());
    let overall_cpu_usage = sys.global_cpu_usage();
    let used_mem = sys.used_memory();


    (processes, overall_cpu_usage, used_mem)
}
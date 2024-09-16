pub(crate) struct ProcessInfo {
    pub(crate) pid: sysinfo::Pid,
    pub(crate) name: String,
    pub(crate) cpu_usage: f32,
    pub(crate) memory_usage: u64,
    pub(crate) status: ProcessStatus
}

#[derive(Debug)]
pub enum ProcessStatus {
    Running,
    Sleeping,
    Stopped,
    Zombie,
}
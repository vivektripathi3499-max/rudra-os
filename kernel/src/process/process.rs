use alloc::vec::Vec;

pub struct Process {
    pub pid: u64,
    pub threads: Vec<u64>,
    pub name: &'static str,
}

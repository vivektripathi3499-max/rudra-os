pub fn check_system(cpu: u32, memory: u32) -> bool {

    if cpu > 90 {
        return false;
    }

    if memory > 90 {
        return false;
    }

    true
}

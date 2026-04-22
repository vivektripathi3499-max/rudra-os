use crate::scheduler::task;

/// Returns the current system tick counter
pub fn ticks() -> u64 {
    task::ticks()
}

/// Puts the current task to sleep for N ticks
pub fn sleep(duration: u64) {
    task::sleep(duration);
}

/// Busy wait delay (only for very short waits)
pub fn delay(duration: u64) {
    let start = ticks();

    while ticks() - start < duration {
        core::hint::spin_loop();
    }
}

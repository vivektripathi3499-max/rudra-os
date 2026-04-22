pub mod task;
pub mod switch;
pub mod run_queue;
pub mod priority;

use core::sync::atomic::{AtomicBool, Ordering};

use run_queue::{add_task, get_next_task};
use task::Task;

pub static REQUEST_SCHEDULE: AtomicBool = AtomicBool::new(false);

pub fn init() {
    // Scheduler initialization
}

pub fn tick() {
    unsafe {
        task::SYSTEM_TICKS += 1;
    }

    // request scheduling
    REQUEST_SCHEDULE.store(true, Ordering::SeqCst);

    schedule();
}

pub fn schedule() {
    REQUEST_SCHEDULE.store(false, Ordering::SeqCst);

    unsafe {
        task::run_tasks();
    }
}

   

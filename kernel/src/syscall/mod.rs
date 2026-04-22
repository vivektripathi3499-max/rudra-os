// kernel/src/syscall/mod.rs

use core::fmt::Write; // 🔥 This fixes the E0599 error!

pub mod entry;
pub mod numbers; 

pub use entry::init;
use numbers::*; 

pub fn handle_syscall(num: u64, arg1: u64, _arg2: u64, _arg3: u64) -> u64 {
    match num {
        SYS_WRITE => sys_write(arg1),
        SYS_EXIT => sys_exit(),
        SYS_TICKS => sys_ticks(),
        SYS_SLEEP => sys_sleep(arg1),
        _ => {
            if let Some(console) = crate::console::CONSOLE.lock().as_mut() {
                let _ = console.write_str("[syscall] unknown syscall executed\n");
            }
            0
        }
    }
}

fn sys_write(value: u64) -> u64 {
    if let Some(console) = crate::console::CONSOLE.lock().as_mut() {
        let _ = console.write_str("[user] ");
        // Temporarily converting small numbers to characters safely
        console.write_char((b'0' + (value as u8)) as char);
        console.newline();
    }
    0
}

fn sys_exit() -> u64 {
    if let Some(console) = crate::console::CONSOLE.lock().as_mut() {
        let _ = console.write_str("[syscall] process exited\n");
    }
    0
}

fn sys_ticks() -> u64 {
    0 // Placeholder for now
}

fn sys_sleep(_time: u64) -> u64 {
    // We will hook this up to your timer next!
    0
}

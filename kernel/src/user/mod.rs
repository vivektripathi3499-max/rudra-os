// kernel/src/user/mod.rs
use core::fmt::Write;
use crate::syscall;

pub fn run(program: &str) {
    match program {
        "hello" => hello(),
        "counter" => counter(),
        _ => {
            if let Some(console) = crate::console::CONSOLE.lock().as_mut() {
                let _ = console.write_str("Unknown user program\n");
            }
        }
    }
}

fn hello() {
    // 1 is SYS_WRITE
    syscall::handle_syscall(1, 100, 0, 0);
}

fn counter() {
    for i in 1..=5 {
        // 1 is SYS_WRITE, 4 is SYS_SLEEP
        syscall::handle_syscall(1, i, 0, 0);
        syscall::handle_syscall(4, 10, 0, 0);
    }
}

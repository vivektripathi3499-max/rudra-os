extern crate alloc;

use alloc::vec::Vec;
use spin::Mutex;
use crate::{print, println};

static BUFFER: Mutex<[u8; 256]> = Mutex::new([0; 256]);
static INDEX: Mutex<usize> = Mutex::new(0);

pub fn init() {
    print!("Rudra> ");
}

pub fn handle_key(c: char) {

    match c {

        '\n' => {
            execute_command();
            reset();
            print!("Rudra> ");
        }

        '\u{8}' => {
            let mut idx = INDEX.lock();
            if *idx > 0 {
                *idx -= 1;
            }
        }

        _ => {

            let mut buffer = BUFFER.lock();
            let mut idx = INDEX.lock();

            if *idx < buffer.len() {

                buffer[*idx] = c as u8;
                *idx += 1;

                print!("{}", c);
            }
        }
    }
}

fn execute_command() {


    let buffer = BUFFER.lock();
    let idx = INDEX.lock();

    let command = core::str::from_utf8(&buffer[..*idx]).unwrap_or("").trim();


    println!();

    if command.is_empty() {
        return;
    }

else if command.starts_with("spawn ") {

    let program = command.trim_start_matches("spawn ").trim();

    crate::process::run(program);

}

    // Dynamic touch command
    if command.starts_with("touch ") {

        let name = &command[6..];
        crate::fs::create_file(name);

        return;
    }

// write command
if command.starts_with("write ") {

    let parts: Vec<&str> = command.splitn(3, ' ').collect();

    if parts.len() == 3 {

        let filename = parts[1];
        let content = parts[2];

        crate::fs::write_file(filename, content);

    } else {

        println!("Usage: write <file> <text>");
    }

    return;
}

// cat command
if command.starts_with("cat ") {

    let filename = &command[4..];

    crate::fs::read_file(filename);

    return;
}

// run command
if command.starts_with("run ") {

    let program = &command[4..];

    crate::process::run(program);

    return;
}

if command.starts_with("kill ") {

    let pid_str = &command[5..];

    if let Ok(pid) = pid_str.parse::<u64>() {
        crate::process::kill(pid);
    } else {
        println!("Invalid PID");
    }

    return;
}

if command.starts_with("spawn ") {

    let program = command.trim_start_matches("spawn ").trim();

    crate::process::run(program);

    return;
}

if command.starts_with("kill ") {

    let pid_str = command.trim_start_matches("kill ").trim();

    if let Ok(pid) = pid_str.parse::<u64>() {
        crate::process::kill(pid);
    }

    return;
}

if command.starts_with("touch ") {

    let filename = command.trim_start_matches("touch ").trim();

    crate::fs::create_file(filename);

    return;
}

match command {

    "help" => {
        println!("Available commands:");
        println!("help     - show commands");
        println!("clear    - clear screen");
        println!("whoami   - show current user");
        println!("version  - show OS version");
        println!("sysinfo  - system information");
        println!("reboot   - restart system");
        println!("ls       - list files");
        println!("touch <file> - create file");
        println!("spawn <program> - create process");
        println!("ps       - list processes");
        println!("kill <pid> - terminate process");
    }

    "ps" => {
        crate::process::list_processes();
    }

    "whoami" => println!("root"),

    "version" => println!("RudraOS Kernel v0.1"),

    "sysinfo" => {
        println!("System: RudraOS");
        println!("Architecture: x86_64");
        println!("Scheduler: active");
        println!("Security: enabled");
    }

    "ls" => {
        crate::fs::list_files();
    }


"send" => {
    crate::ipc::send(0, 1, "hello");
}

"recv" => {
    crate::ipc::receive(1);
}

"run hello" => {
    crate::user::run("hello");
}

"run counter" => {
    crate::user::run("counter");
}


"dmesg" => {
    crate::debug::log_buffer::dump();
}

"syscall" => {
    println!("Invoking syscall...");
    unsafe {
        core::arch::asm!("int 0x80");
    }
}


        "clear" | "clean" => {
            if let Some(console) = crate::console::CONSOLE.lock().as_mut() {
                console.clear();
            }
        }

        "reboot" => {
            unsafe {
                crate::port::outb(0x64, 0xFE);
            }
        }

        _ => println!("Unknown command: {}", command),
    }
}

fn reset() {
    let mut idx = INDEX.lock();
    *idx = 0;
}



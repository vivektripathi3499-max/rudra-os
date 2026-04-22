extern crate alloc;

use alloc::vec::Vec;
use alloc::string::String;
use spin::Mutex;
use crate::println;

pub mod switch_to_user;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ProcessState {
    Ready,
    Running,
    Waiting,
    Terminated,
}

pub struct Process {
    pub pid: u64,
    pub name: String,
    pub state: ProcessState,
}

static PROCESS_TABLE: Mutex<Vec<Process>> = Mutex::new(Vec::new());

static mut NEXT_PID: u64 = 1;

fn generate_pid() -> u64 {
    unsafe {
        let pid = NEXT_PID;
        NEXT_PID += 1;
        pid
    }
}

pub fn init() {
    println!("Process manager ready");
}

pub fn run(program: &str) {
    match program {
        "hello" => spawn("hello"),
        _ => println!("Program not found"),
    }
}

pub fn spawn(name: &str) {

    let pid = generate_pid();

    let process = Process {
        pid,
        name: name.into(),
        state: ProcessState::Ready,
    };

    PROCESS_TABLE.lock().push(process);

    println!("[process] spawned {} (pid={})", name, pid);

    match name {

"hello" => {

    crate::scheduler::task::add_task(
        crate::scheduler::task::Task {
            id: pid as usize,
            func: hello_program,
            context: Default::default(),
            stack: [0; 4096],
            state: crate::scheduler::task::TaskState::Ready,
            sleep_until: 0,
            priority: crate::scheduler::priority::Priority::Normal,
        }
    );

    // 🔴 force scheduler to run
    crate::scheduler::REQUEST_SCHEDULE.store(
        true,
        core::sync::atomic::Ordering::SeqCst
    );

}


        _ => println!("Unknown program"),
    }
}

pub fn list_processes() {

    let table = PROCESS_TABLE.lock();

    for p in table.iter() {
        println!("PID {} - {} [{:?}]", p.pid, p.name, p.state);
    }
}

pub fn kill(pid: u64) {

    let mut table = PROCESS_TABLE.lock();

    for p in table.iter_mut() {

        if p.pid == pid {

            p.state = ProcessState::Terminated;

            println!("[process] killed pid {}", pid);

            return;
        }
    }

    println!("Process {} not found", pid);
}

pub fn set_running(pid: u64) {

    let mut table = PROCESS_TABLE.lock();

    for p in table.iter_mut() {

        if p.pid == pid {

            p.state = ProcessState::Running;
            return;

        }

    }
}

fn hello_program() {

    for _ in 0..5 {
        println!("[hello] process running");
        crate::time::sleep(30);
    }

    println!("[hello] process finished");

    // mark this process terminated
    let mut table = PROCESS_TABLE.lock();

    for p in table.iter_mut() {
        if p.name == "hello" {
            p.state = ProcessState::Terminated;
        }
    }

    // stop the scheduler from re-running this task
    loop {
        x86_64::instructions::hlt();
    }
}

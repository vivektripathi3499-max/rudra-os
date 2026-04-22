use crate::println;
use super::switch::context_switch;
use super::priority::Priority;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Context {
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub rbx: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub rip: u64,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TaskState {
    Ready,
    Running,
    Sleeping,
    Waiting,
    Blocked,
    Zombie,
    Terminated,
}


#[derive(Copy, Clone)]
pub struct Task {
    pub id: usize,
    pub func: fn(),
    pub context: Context,
    pub stack: [u8; 4096],
    pub state: TaskState,
    pub sleep_until: u64,
    pub priority: Priority,
}




static mut TASKS: [Option<Task>; 16] = [None; 16];
static mut TASK_COUNT: usize = 0;
static mut CURRENT_TASK: usize = 0;

static mut LAST_SWITCH: u64 = 0;
const TIME_SLICE: u64 = 10; // ticks

pub static mut SYSTEM_TICKS: u64 = 0;
pub fn ticks() -> u64 {
    unsafe { SYSTEM_TICKS }
}

extern "C" fn task_entry() {
    unsafe {
        if let Some(task) = &TASKS[CURRENT_TASK] {
            (task.func)();
        }
    }

    // if the task function ever returns
    loop {
        yield_task();
    }
}


pub fn add_task(mut task: Task) {
    unsafe {

        if TASK_COUNT >= TASKS.len() {
            return;
        }

        // Set task entry point
        task.id = TASK_COUNT;
task.context.rip = task_entry as u64;

        // Clear registers
        task.context.r15 = 0;
        task.context.r14 = 0;
        task.context.r13 = 0;
        task.context.r12 = 0;
        task.context.rbx = 0;
        task.context.rbp = 0;

        // Setup stack pointer (top of stack)
        let stack_top = task.stack.as_ptr() as u64 + task.stack.len() as u64;

        // Align stack to 16 bytes (important for x86_64 ABI)
        task.context.rsp = stack_top & !0xF;

        // Default task settings
        task.priority = Priority::Normal;
        task.state = TaskState::Ready;
        task.sleep_until = 0;

        // Add task to scheduler
        TASKS[TASK_COUNT] = Some(task);
        TASK_COUNT += 1;

    }
}

pub fn sleep(ticks: u64) {
    unsafe {
        if let Some(task) = &mut TASKS[CURRENT_TASK] {
            task.state = TaskState::Sleeping;
            task.sleep_until = SYSTEM_TICKS + ticks;
        }
    }

    // Force scheduler immediately
    run_tasks();
}

pub fn yield_task() {

    unsafe {
        if let Some(task) = &mut TASKS[CURRENT_TASK] {
            task.state = TaskState::Ready;
        }
    }

    // Immediately run scheduler
    run_tasks();
}

pub fn run_tasks() {
    unsafe {

        if TASK_COUNT == 0 {
            return;
        }

        // ⏱️ time slicing
        if SYSTEM_TICKS - LAST_SWITCH < TIME_SLICE {
            return;
        }
        LAST_SWITCH = SYSTEM_TICKS;

        // 🔄 wake sleeping tasks
        for i in 0..TASK_COUNT {
            if let Some(task) = &mut TASKS[i] {
                if task.state == TaskState::Sleeping &&
                   SYSTEM_TICKS >= task.sleep_until {
                    task.state = TaskState::Ready;
                }
            }
        }

        // 🧠 priority-based selection
        let mut selected_task: Option<usize> = None;
        let mut best_priority = Priority::Low;

        for i in 0..TASK_COUNT {
            if let Some(task) = &TASKS[i] {
                if task.state == TaskState::Ready &&
                   task.priority >= best_priority {
                    best_priority = task.priority;
                    selected_task = Some(i);
                }
            }
        }

        // 🔁 switch task
        if let Some(i) = selected_task {

            let prev = CURRENT_TASK;

            crate::process::set_running(i as u64);

            if let (Some(prev_task), Some(next_task)) =
                (&mut TASKS[prev], &TASKS[i])
            {
                let prev_context = &mut prev_task.context;
                let next_context = &next_task.context;

                CURRENT_TASK = i;

                if prev != i {
                    context_switch(prev_context, next_context);
                }
            }
        }
    }
}

fn task1() {
    loop {
        println!("Task 1");
    }
}

fn task2() {
    loop {
        println!("Task 2");
    }
}

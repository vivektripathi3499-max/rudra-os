use alloc::collections::VecDeque;
use spin::Mutex;

use super::task::Task;

pub struct RunQueue {
    queue: VecDeque<Task>,
}

impl RunQueue {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    pub fn push(&mut self, task: Task) {
        self.queue.push_back(task);
    }

    pub fn pop(&mut self) -> Option<Task> {
        self.queue.pop_front()
    }
}

static RUN_QUEUE: Mutex<Option<RunQueue>> = Mutex::new(None);

pub fn init_run_queue() {
    *RUN_QUEUE.lock() = Some(RunQueue::new());
}

pub fn add_task(task: Task) {
    if let Some(ref mut rq) = *RUN_QUEUE.lock() {
        rq.push(task);
    }
}

pub fn get_next_task() -> Option<Task> {
    if let Some(ref mut rq) = *RUN_QUEUE.lock() {
        rq.pop()
    } else {
        None
    }
}

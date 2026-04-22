use crate::println;

pub fn log(level: &str, msg: &str) {
    println!("[{}] {}", level, msg);
}

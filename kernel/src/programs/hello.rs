use crate::println;
use crate::time;

pub fn run() {

    for _ in 0..5 {
        println!("[hello] process running");
        time::sleep(1000);
    }

    println!("[hello] finished");
}

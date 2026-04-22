use crate::println;

pub mod hello;
pub mod registry;

pub fn hello_program() {

    println!("Hello from RudraOS program Developed by Vivek Tripathi!");

    loop {
        x86_64::instructions::hlt();
    }

}

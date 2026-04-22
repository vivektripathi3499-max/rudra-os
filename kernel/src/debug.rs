use core::panic::PanicInfo;
use crate::serial_println;
use crate::println;
use x86_64::instructions::hlt;

pub mod log_buffer;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {

    serial_println!("");
    serial_println!("==============================");
    serial_println!("        KERNEL PANIC");
    serial_println!("==============================");

    if let Some(location) = info.location() {
        serial_println!("File : {}", location.file());
        serial_println!("Line : {}", location.line());
        serial_println!("Column : {}", location.column());
    }

    serial_println!("Message : {}", info);

    serial_println!("System halted.");

    loop {
        hlt();
    }
}

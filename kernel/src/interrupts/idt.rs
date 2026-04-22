use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::interrupts::{timer, keyboard, mouse};
use crate::println;

// =============================
// HANDLERS
// =============================

extern "x86-interrupt" fn breakpoint_handler(
    _stack_frame: InterruptStackFrame,
) {
    println!("🔥 EXCEPTION: BREAKPOINT");
}

// =============================
// IDT (STATIC + SAFE)
// =============================

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint.set_handler_fn(breakpoint_handler);

        timer::init(&mut idt);
        keyboard::init(&mut idt);
        mouse::init(&mut idt);

        idt
    };
}

// =============================
// INIT
// =============================

pub fn init() {
    crate::serial_println!("Loading IDT...");
    IDT.load();
    crate::serial_println!("IDT LOADED SUCCESSFULLY 🔥");
}

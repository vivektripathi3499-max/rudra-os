use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::interrupts::pic::{PICS, PIC_1_OFFSET};
use crate::port::outb;

pub const TIMER_INTERRUPT_ID: u8 = PIC_1_OFFSET;

use crate::scheduler;

extern "x86-interrupt" fn timer_interrupt_handler(
    _stack_frame: InterruptStackFrame,
) {
    // 🔥 trigger scheduler every tick
   // scheduler::tick();

    unsafe {
        PICS.lock().notify_end_of_interrupt(TIMER_INTERRUPT_ID);
    }
}

pub fn init(idt: &mut InterruptDescriptorTable) {
    idt[TIMER_INTERRUPT_ID as usize]
        .set_handler_fn(timer_interrupt_handler);
}

pub fn init_timer() {

    // PIT frequency (~1000 Hz)
    let frequency: u16 = 1193;

    unsafe {
        outb(0x43, 0x36);
        outb(0x40, (frequency & 0xFF) as u8);
        outb(0x40, (frequency >> 8) as u8);
    }
}

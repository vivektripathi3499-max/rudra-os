use crate::port::inb;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::interrupts::pic::{PICS, PIC_1_OFFSET};

use pc_keyboard::{Keyboard, ScancodeSet1, layouts, HandleControl, DecodedKey};
use spin::Mutex;
use lazy_static::lazy_static;

pub const KEYBOARD_INTERRUPT: u8 = PIC_1_OFFSET + 1;

// =============================
// GLOBAL BUFFER (INTERRUPT SAFE)
// =============================

use core::sync::atomic::{AtomicBool, AtomicU8, Ordering};

pub static LAST_SCANCODE: AtomicU8 = AtomicU8::new(0);
pub static HAS_KEY: AtomicBool = AtomicBool::new(false);

// =============================
// KEYBOARD DECODER (FIXED)
// =============================

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
        Mutex::new(Keyboard::new(
            ScancodeSet1::new(),   // ✅ CORRECT ORDER
            layouts::Us104Key,
            HandleControl::Ignore
        ));
}

// =============================
// INTERRUPT HANDLER
// =============================

extern "x86-interrupt" fn keyboard_interrupt_handler(
    _stack_frame: InterruptStackFrame
) {
    let scancode = unsafe { inb(0x60) };

    // ✅ STORE ONLY (SAFE)
   
 LAST_SCANCODE.store(scancode, Ordering::Relaxed);
HAS_KEY.store(true, Ordering::Relaxed);
    // optional debug
   // crate::serial_println!("SCANCODE: {}", scancode);

    // ✅ SEND EOI
    unsafe {
        PICS.lock().notify_end_of_interrupt(KEYBOARD_INTERRUPT);
    }
}

// =============================
// INIT
// =============================

pub fn init(idt: &mut InterruptDescriptorTable) {
    idt[KEYBOARD_INTERRUPT as usize]
        .set_handler_fn(keyboard_interrupt_handler);
}

// =============================
// SCANCODE → KEY DECODER
// =============================

pub fn decode_scancode(scancode: u8) -> Option<DecodedKey> {
    let mut keyboard = KEYBOARD.lock();

    if let Ok(Some(event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(event) {
            return Some(key);
        }
    }

    None
}

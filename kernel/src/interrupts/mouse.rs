use core::sync::atomic::{AtomicI32, AtomicBool, Ordering};

pub static MOUSE_X_ATOMIC: AtomicI32 = AtomicI32::new(400);
pub static MOUSE_Y_ATOMIC: AtomicI32 = AtomicI32::new(300);
pub static MOUSE_EVENT: AtomicBool = AtomicBool::new(false);
pub static MOUSE_CLICK: AtomicBool = AtomicBool::new(false);


use x86_64::structures::idt::InterruptStackFrame;
use crate::interrupts::pic::PICS;
use crate::interrupts::InterruptIndex;
use x86_64::structures::idt::InterruptDescriptorTable;


use spin::Mutex;
use crate::port::{inb, outb};

static mut PACKET: [u8; 3] = [0; 3];
static mut INDEX: usize = 0;


// 🔥 Fix: track button state (IMPORTANT)
static LAST_LEFT: Mutex<bool> = Mutex::new(false);

/* =========================
   INIT MOUSE (PS/2)
========================= */

pub fn init_hardware() {
    unsafe {
        // Enable auxiliary device (mouse)
        outb(0x64, 0xA8);

        // Enable interrupts
        outb(0x64, 0x20);
        let status = inb(0x60) | 2;
        outb(0x64, 0x60);
        outb(0x60, status);

        // Default settings
        write_mouse(0xF6);
        read_mouse();

        // Enable streaming
        write_mouse(0xF4);
        read_mouse();
    }
}

/* =========================
   LOW LEVEL IO
========================= */

fn write_mouse(value: u8) {
    unsafe {
        outb(0x64, 0xD4);
        outb(0x60, value);
    }
}

fn read_mouse() -> u8 {
    unsafe { inb(0x60) }
}

/* =========================
   INTERRUPT HANDLER ENTRY
========================= */

pub fn handle_interrupt(data: u8) {
    unsafe {
        // 🔥 STEP 1: SYNC FIRST BYTE (CRITICAL FIX)
        if INDEX == 0 && (data & 0x08) == 0 {
            return;
        }

        PACKET[INDEX] = data;
        INDEX += 1;

        if INDEX < 3 {
            return;
        }

        INDEX = 0;

        let flags = PACKET[0];

        // 🔥 STEP 2: IGNORE OVERFLOW
        if (flags & 0x40) != 0 || (flags & 0x80) != 0 {
            return;
        }

        let dx = PACKET[1] as i8 as i32;
        let dy = PACKET[2] as i8 as i32;

        let mut x = MOUSE_X_ATOMIC.load(Ordering::Relaxed);
        let mut y = MOUSE_Y_ATOMIC.load(Ordering::Relaxed);

        // 🔥 STEP 3: TRY RAW FIRST (IMPORTANT)
        x += dx;
        y -= dy;

        // 🔥 STEP 4: CLAMP (MATCH YOUR SCREEN 1280x800)
        if x < 0 { x = 0; }
        if y < 0 { y = 0; }
        if x > 1279 { x = 1279; }
        if y > 799 { y = 799; }

        MOUSE_X_ATOMIC.store(x, Ordering::Relaxed);
        MOUSE_Y_ATOMIC.store(y, Ordering::Relaxed);

        // 🔥 STEP 5: CLICK
        let mut last = LAST_LEFT.lock();
        let current = (flags & 1) != 0;

        if current && !*last {
            MOUSE_CLICK.store(true, Ordering::Relaxed);
        }

        *last = current;

        // 🔥 STEP 6: SIGNAL
        MOUSE_EVENT.store(true, Ordering::Relaxed);
    }
}


extern "x86-interrupt" fn mouse_interrupt_handler(
    _stack_frame: InterruptStackFrame
) {
    let data = unsafe { inb(0x60) };

    handle_interrupt(data);

    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Mouse.as_u8());
    }
}

pub fn init(idt: &mut InterruptDescriptorTable) {
    idt[InterruptIndex::Mouse.as_usize()]
        .set_handler_fn(mouse_interrupt_handler);
}

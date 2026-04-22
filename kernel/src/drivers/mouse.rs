use spin::Mutex;
use crate::port::{inb, outb};

static mut PACKET: [u8; 3] = [0; 3];
static mut INDEX: usize = 0;

static MOUSE_X: Mutex<i32> = Mutex::new(400);
static MOUSE_Y: Mutex<i32> = Mutex::new(300);

// track click state (IMPORTANT)
static LAST_LEFT: Mutex<bool> = Mutex::new(false);

/* =========================
   INIT MOUSE (PS/2)
========================= */

pub fn init() {
    unsafe {
        // Enable auxiliary device
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

pub fn handle_interrupt(byte: u8) {
    unsafe {
        // Sync packet (important)
        if INDEX == 0 && (byte & 0x08) == 0 {
            return;
        }

        PACKET[INDEX] = byte;
        INDEX += 1;

        if INDEX == 3 {
            INDEX = 0;

            let buttons = PACKET[0];
            let dx = PACKET[1] as i8 as i32;
            let dy = PACKET[2] as i8 as i32;

            let mut x = MOUSE_X.lock();
            let mut y = MOUSE_Y.lock();

            // update position
            *x += dx;
            *y -= dy; // invert Y

            // clamp (adjust if needed)
            if *x < 0 { *x = 0; }
            if *y < 0 { *y = 0; }
            if *x > 1024 { *x = 1024; }
            if *y > 768 { *y = 768; }

            // =========================
            // WINDOW SYSTEM
            // =========================
            crate::window::mouse_move(*x, *y);

            // =========================
            // CLICK HANDLING (EDGE TRIGGER)
            // =========================
            let mut last = LAST_LEFT.lock();
            let current = (buttons & 1) != 0;

            if current && !*last {
                crate::window::mouse_down(*x, *y);
                crate::ui::handle_click(*x, *y);
            }

            if !current && *last {
                crate::window::mouse_up();
            }

            *last = current;

            // =========================
            // REDRAW SYSTEM
            // =========================
            crate::window::redraw();

            // =========================
            // CURSOR (LAST)
            // =========================
            crate::drivers::cursor::update_cursor(*x, *y);
        }
    }
}

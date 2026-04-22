use crate::console::CONSOLE;
use core::fmt::Write;

static mut PROGRESS: u8 = 0;

pub fn draw_boot_screen() {
    if let Some(console) = crate::console::CONSOLE.lock().as_mut() {

        let width = console.info.width;
        let height = console.info.height;

        // Background
        for y in 0..height {
            for x in 0..width {
                console.put_pixel(x as i32, y as i32, 0x030308);
            }
        }

        // Title
        console.set_cursor(200, 100);
        let _ = write!(console, "RUDRA OS");

        console.set_cursor(200, 140);
        let _ = write!(console, "Booting...");
    }
}

pub fn set_progress(p: u8) {
    unsafe { PROGRESS = p; }

    if let Some(console) = CONSOLE.lock().as_mut() {

        let bar_x = 100;
        let bar_y = 300;
        let bar_width = 400;
        let bar_height = 20;

        // background bar
        for y in 0..bar_height {
            for x in 0..bar_width {
                console.put_pixel(
                    (bar_x + x) as i32,
                    (bar_y + y) as i32,
                    0x222222,
                );
            }
        }

        // filled bar
        let filled = (bar_width * p as usize) / 100;

        for y in 0..bar_height {
            for x in 0..filled {
                console.put_pixel(
                    (bar_x + x) as i32,
                    (bar_y + y) as i32,
                    0x00FFD5,
                );
            }
        }

        // text
        console.set_cursor(100, 330);
        let _ = write!(console, "Loading: {}%", p);
    }
}

// ✅ MOVE THIS OUTSIDE
pub fn log(msg: &'static str) {
    if let Some(console) = CONSOLE.lock().as_mut() {

        static mut Y: usize = 360;

        unsafe {
            console.set_cursor(100, Y);
            let _ = write!(console, "{}", msg);
            Y += 20;
        }
    }
}

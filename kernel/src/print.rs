use core::fmt;
use core::sync::atomic::{AtomicBool, Ordering};

use crate::MODE;

pub static BOOT_MODE: AtomicBool = AtomicBool::new(true);

pub fn _print(args: fmt::Arguments) {
    // 🚫 Disable printing during boot
    if BOOT_MODE.load(Ordering::Relaxed) {
        return;
    }

    // 🚫 Disable printing in GUI mode
    if MODE.load(Ordering::Relaxed) == 1 {
        return;
    }

    x86_64::instructions::interrupts::without_interrupts(|| {
        if let Some(console) = crate::console::CONSOLE.lock().as_mut() {
            use core::fmt::Write;
            let _ = console.write_fmt(args);
        }
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::print::_print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($fmt:expr) => ($crate::print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn backspace() {
    // 🚫 Disable in GUI mode
    if MODE.load(Ordering::Relaxed) == 1 {
        return;
    }

    x86_64::instructions::interrupts::without_interrupts(|| {
        if let Some(console) = crate::console::CONSOLE.lock().as_mut() {
            console.backspace();
        }
    });
}

use pic8259::ChainedPics;
use spin::Mutex;
use x86_64::instructions::port::Port;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

// =========================
// SMALL IO WAIT (OPTIONAL)
// =========================
fn io_wait() {
    unsafe {
        let mut port = Port::<u8>::new(0x80);
        port.write(0);
    }
}

// =========================
// INIT PIC + ENABLE INPUT
// =========================
pub fn init() {
    unsafe {
        // 🔥 INIT PIC
        PICS.lock().initialize();
        io_wait();

        // =========================
        // 🔥 UNMASK KEYBOARD (IRQ1)
        // =========================
        let mut pic1 = Port::<u8>::new(0x21);
        let mask1 = pic1.read();
        pic1.write(mask1 & !(1 << 1)); // enable IRQ1
        io_wait();

        // =========================
        // 🔥 UNMASK MOUSE (IRQ12)
        // =========================
        let mut pic2 = Port::<u8>::new(0xA1);
        let mask2 = pic2.read();
        pic2.write(mask2 & !(1 << 4)); // enable IRQ12
        io_wait();
    }
}

// =========================
// END OF INTERRUPT
// =========================
pub fn eoi(irq: u8) {
    unsafe {
        PICS.lock().notify_end_of_interrupt(irq);
    }
}

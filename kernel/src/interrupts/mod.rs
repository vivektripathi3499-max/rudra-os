pub mod idt;
pub mod pic;
pub mod timer;
pub mod keyboard;
pub mod mouse;
pub mod syscall;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = pic::PIC_1_OFFSET,
    Keyboard,
    Cascade,
    Com2,
    Com1,
    Lpt2,
    Floppy,
    Lpt1,
    Rtc,
    Free1,
    Free2,
    Free3,
    Mouse,
}

impl InterruptIndex {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

pub fn init() {
    // 1. Load IDT FIRST
    idt::init();

    // 2. THEN init PIC
    pic::init();

    // 3. THEN timer
    timer::init_timer();
}

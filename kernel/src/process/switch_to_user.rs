use core::arch::asm;

pub unsafe fn enter_user_mode(entry: u64, stack: u64) -> ! {
    asm!(
        "mov ax, 0x23",
        "mov ds, ax",
        "mov es, ax",
        "mov fs, ax",
        "mov gs, ax",

        "push 0x23",
        "push {stack}",

        "pushfq",

        "push 0x1B",
        "push {entry}",

        "iretq",

        stack = in(reg) stack,
        entry = in(reg) entry,

        options(noreturn)
    );
}

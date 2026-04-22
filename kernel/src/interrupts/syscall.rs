use crate::syscall::handle_syscall;
use x86_64::structures::idt::InterruptStackFrame;

pub extern "x86-interrupt" fn syscall_handler(
    _stack_frame: InterruptStackFrame
) {

    let syscall_num: u64;
    let arg1: u64;
    let arg2: u64;
    let arg3: u64;

    unsafe {
        core::arch::asm!(
            "mov {}, rax",
            "mov {}, rdi",
            "mov {}, rsi",
            "mov {}, rdx",
            out(reg) syscall_num,
            out(reg) arg1,
            out(reg) arg2,
            out(reg) arg3,
        );
    }

    handle_syscall(syscall_num, arg1, arg2, arg3);
}

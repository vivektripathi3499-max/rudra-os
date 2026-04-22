#![allow(dead_code)]

use x86_64::registers::model_specific::{Msr, Efer, EferFlags};

const IA32_STAR: u32 = 0xC0000081;
const IA32_LSTAR: u32 = 0xC0000082;
const IA32_FMASK: u32 = 0xC0000084;

/* =========================
   INIT SYSCALL SYSTEM
========================= */

pub fn init() {
    unsafe {
        // Enable SYSCALL/SYSRET
        Efer::update(|efer| {
            efer.insert(EferFlags::SYSTEM_CALL_EXTENSIONS);
        });

        // Set syscall entry point (FIXED CAST)
        Msr::new(IA32_LSTAR).write(syscall_entry as *const () as u64);

        // Segment selectors
        // Kernel CS = 0x08
        // User CS = 0x1B (must match your GDT!)
        Msr::new(IA32_STAR).write(
            ((0x08u64) << 32) | ((0x1Bu64) << 48)
        );

        // Clear flags mask
        Msr::new(IA32_FMASK).write(0);
    }
}

/* =========================
   SYSCALL ENTRY (ASM)
========================= */

#[unsafe(naked)]
pub extern "C" fn syscall_entry() -> ! {
    unsafe {
        core::arch::naked_asm!(
            "
            // Save registers
            push rdi
            push rsi
            push rdx
            push rcx
            push r8
            push r9

            // syscall number is in rax
            mov rdi, rax

            // Call Rust handler
            call syscall_dispatch

            // Restore registers
            pop r9
            pop r8
            pop rcx
            pop rdx
            pop rsi
            pop rdi

            // Return to user mode
            sysretq
            "
        );
    }
}

/* =========================
   SYSCALL DISPATCHER
========================= */

#[no_mangle]
pub extern "C" fn syscall_dispatch(syscall_number: u64) {
    match syscall_number {
        1 => {
            crate::serial_println!("SYSCALL WORKED 🚀");
        }
        _ => {
            crate::serial_println!("Unknown syscall: {}", syscall_number);
        }
    }
}

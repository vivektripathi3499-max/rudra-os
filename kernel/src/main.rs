#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;

use bootloader_api::{entry_point, BootInfo};
use bootloader_api::config::Mapping;
use pc_keyboard::DecodedKey;
use core::fmt::Write;

use x86_64::{
    VirtAddr,
    structures::paging::{
        Page,
        Size4KiB,
        FrameAllocator,
        Mapper,
        PageTableFlags,
    },
};



// =============================
// GLOBAL STATE
// =============================




use core::sync::atomic::{AtomicU8, Ordering};

static MODE: AtomicU8 = AtomicU8::new(1);
// 0 = Console
// 1 = GUI

const TEXT_X: usize = 10;
const TEXT_Y: usize = 10;

pub static BOOTLOADER_CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();

    // keep this
    config.mappings.physical_memory = Some(Mapping::Dynamic);

    // 🔥 ADD THIS LINE (IMPORTANT)
    

    config
};

// =============================
// MODULE DECLARATIONS
// =============================

// Low Level Hardware Access
mod port;
mod hal;
pub mod serial;

mod gdt;

// Memory Management
pub mod memory;

// CPU / Interrupts / Timer
mod interrupts;
mod time;

// Process + Scheduler
mod process;
mod scheduler;

// System Call + Security
mod syscall;
mod security;

// Device Drivers + IPC
mod drivers;
mod ipc;

// Filesystem + Execution
mod fs;
mod elf;
mod exec;

// User Space + Programs
mod user;
mod programs;

// System Interface
mod console;
mod print;
mod shell;

// GUI System
mod window;
mod ui;
mod splash;
//mod boot_ui;

// Debugging
mod debug;

// Graphics
mod graphics;



use console::Console;

// =============================
// KERNEL TASKS
// =============================

fn task_ai() {
    static mut INITIALIZED: bool = false;
    static mut COUNTER: u64 = 0;

    unsafe {
        if !INITIALIZED {
           
            INITIALIZED = true;
        }

        COUNTER += 1;

        if COUNTER % 20 == 0 {
            let cpu_usage = 30;
            let memory_usage = 40;

            if !crate::security::check_system(cpu_usage, memory_usage) {
                println!("AI detected anomaly!");
            }
        }
    }
}

fn task_system() {
    static mut COUNTER: u64 = 0;

    unsafe {
        COUNTER += 1;

        if COUNTER % 20 == 0 {
            if !security::check_system(30, 40) {
                println!("Security warning!");
            }
        }
    }

    crate::time::sleep(10);
}

fn task1() {
    loop {
        
        crate::time::sleep(50);
    }
}

fn task2() {
    loop {
       
        crate::time::sleep(80);
    }
}

extern "C" fn user_test() -> ! {
    unsafe {
        core::arch::asm!(
            // syscall number
            "mov rax, 1",

            // return RIP
            "lea rcx, [rip + 2f]",

            // flags → r11
            "pushfq",
            "pop r11",

            // syscall
            "syscall",

            // return point
            "2:",

            // safe infinite halt loop
            "hlt",
            "jmp 2b",

            options(noreturn)
        );
    }
}
fn delay() {
    for _ in 0..200_000 {
        core::hint::spin_loop();
    }
}

// =============================
// ENTRY POINT
// =============================

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

use crate::process::switch_to_user::enter_user_mode;

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    serial_println!("Kernel entry reached!");
    
   

    // =========================
    // INIT HAL + MEMORY ONLY
    // =========================
    serial_println!("Initializing HAL");
    hal::init();
    
   // boot_ui::log("[ OK ] HAL initialized");
//boot_ui::set_progress(10);
    
    serial_println!("Initializing GDT");
gdt::init();

//boot_ui::log("[ OK ] GDT loaded");
//boot_ui::set_progress(15);

    serial_println!("Initializing memory");
    memory::init();
    
   // boot_ui::log("[ OK ] Memory initialized");
//boot_ui::set_progress(25);

    // =========================
    // PAGING
    // =========================
    serial_println!("Entering paging initialization");

    let phys_mem_offset = VirtAddr::new(
        boot_info
            .physical_memory_offset
            .into_option()
            .expect("physical_memory_offset missing"),
    );

    unsafe {
        memory::paging::init(phys_mem_offset);
    }

    serial_println!("Paging initialized");
    
    //boot_ui::log("[ OK ] Paging ready");
//boot_ui::set_progress(35);

    // =========================
    // MAP FRAMEBUFFER
    // =========================
    let fb = boot_info
        .framebuffer
        .as_ref()
        .expect("Framebuffer missing");

    let fb_addr = fb.buffer().as_ptr() as u64;
    let fb_size = fb.buffer().len();

    serial_println!("Using bootloader framebuffer mapping");

    // =========================
    // RECREATE CONSOLE AFTER PAGING
    // =========================
    let framebuffer = boot_info
        .framebuffer
        .as_mut()
        .expect("Framebuffer not provided");

   let info = framebuffer.info();

let buffer = unsafe {
    core::slice::from_raw_parts_mut(
        framebuffer.buffer().as_ptr() as *mut u8,
        framebuffer.buffer().len(),
    )
};
    let console = Console::new(buffer, info);

   {
    let mut lock = console::CONSOLE.lock();
    *lock = Some(console);
}

// 🔥 HARD CLEAR WHOLE FRAMEBUFFER (CRITICAL FIX)
if let Some(console) = crate::console::CONSOLE.lock().as_mut() {
    let width = console.info.width;
    let height = console.info.height;

    for y in 0..height {
        for x in 0..width {
            console.put_pixel(x as i32, y as i32, 0x000000);
        }
    }
}
    if let Some(console) = crate::console::CONSOLE.lock().as_mut() {
    console.clear();
}

    serial_println!("Console initialized");
    
 

//crate::boot_ui::draw_boot_screen();
    



//boot_ui::log("[ OK ] Graphics initialized");
//boot_ui::set_progress(5);

    // =========================
    // NOW INIT INTERRUPTS
    // =========================
    serial_println!("Initializing interrupts AFTER paging");
    interrupts::init();
    crate::interrupts::mouse::init_hardware(); 
    x86_64::instructions::interrupts::enable();
    if x86_64::instructions::interrupts::are_enabled() {
    serial_println!("INTERRUPTS ARE ENABLED ✅");
} else {
    serial_println!("INTERRUPTS NOT ENABLED ❌");
}
    x86_64::instructions::interrupts::int3();
   // boot_ui::log("[ OK ] Interrupts ready");
//boot_ui::set_progress(50);
    
   
    
  

    println!("Interrupts initialized");

crate::syscall::init();

//boot_ui::log("[ OK ] Syscalls ready");
//boot_ui::set_progress(60);

    serial_println!("Hardware Syscalls Initialized");

    // =========================
    // FRAME ALLOCATOR
    // =========================
    let _frame_allocator = unsafe {
        memory::frame_allocator::BootInfoFrameAllocator::init(&boot_info.memory_regions)
    };

    println!("Frame allocator ready");
    
    crate::window::create_window(200, 150, 300, 200);

    // =========================
    // SYSTEMS
    // =========================
    process::init();
    scheduler::init();
    fs::init();
    
   // boot_ui::log("[ OK ] Core systems ready");
//boot_ui::set_progress(75);
    
    use crate::scheduler::task::{Task, TaskState};
use crate::scheduler::priority::Priority;
use crate::scheduler::task::add_task;

// 🔥 ADD TASKS TO SCHEDULER

add_task(Task {
    id: 0,
    func: task1,
    context: Default::default(),
    stack: [0; 4096],
    state: TaskState::Ready,
    sleep_until: 0,
    priority: Priority::Normal,
});

add_task(Task {
    id: 1,
    func: task2,
    context: Default::default(),
    stack: [0; 4096],
    state: TaskState::Ready,
    sleep_until: 0,
    priority: Priority::High,
});

// Optional: your AI system
add_task(Task {
    id: 2,
    func: task_ai,
    context: Default::default(),
    stack: [0; 4096],
    state: TaskState::Ready,
    sleep_until: 0,
    priority: Priority::Low,
});

//boot_ui::log("[ OK ] Scheduler tasks loaded");
//boot_ui::set_progress(85);


 

    // =========================
    // ENABLE INTERRUPTS LAST
    // =========================

    println!("Enabling interrupts...");
   
    
    

let user_stack_top = crate::memory::paging::create_user_stack().as_u64();

unsafe {
    enter_user_mode(
       user_test as *const () as u64,
        user_stack_top
    );
}





  
    
    serial_println!("Interrupts enabled");

    print::BOOT_MODE.store(false, core::sync::atomic::Ordering::Relaxed);
    serial_println!("System ready");
  crate::print::BOOT_MODE.store(false, Ordering::Relaxed);
    // =========================
    // SHELL
    // =========================
   
  // boot_ui::log("[ OK ] Preparing shell");
//boot_ui::set_progress(95);
   
    crate::shell::init();
    
    
   // boot_ui::log("[ DONE ] Boot complete");
//boot_ui::set_progress(100);

   

    // =========================
    // MAIN LOOP
    // =========================
   // =========================
    // MAIN LOOP
    // =========================
    // =========================
    // MAIN LOOP
    // =========================
  // =========================
    // UPDATED DEADLOCK-FREE MAIN LOOP
    // =========================
loop {
    let mut did_work = false;

    // =========================
    // KEYBOARD + MOUSE (UNSAFE)
    // =========================
    unsafe {
        // =========================
        // KEYBOARD HANDLING
        // =========================
        if crate::interrupts::keyboard::HAS_KEY.swap(false, Ordering::Relaxed) {
            let sc = crate::interrupts::keyboard::LAST_SCANCODE.load(Ordering::Relaxed);
            
            if sc == 0x0F {
    let new = 1 - MODE.load(Ordering::Relaxed);
    MODE.store(new, Ordering::Relaxed);

    if let Some(c) = crate::console::CONSOLE.lock().as_mut() {
        c.clear();
    }

   
}

          if let Some(key) = crate::interrupts::keyboard::decode_scancode(sc) {

// 🔥 DIRECT SCANCODE CHECK FOR TAB
if sc == 0x0F {   // TAB key scancode
    let new = 1 - MODE.load(Ordering::Relaxed);
    MODE.store(new, Ordering::Relaxed);

    if let Some(c) = crate::console::CONSOLE.lock().as_mut() {
        c.clear();
    }

    println!("MODE: {}", new);
}

    match key {

        // =========================
        // TEXT INPUT
        // =========================
        DecodedKey::Unicode(c) => {

            match c {

                // TAB → switch mode
                '\t' => {
                    let new = 1 - MODE.load(Ordering::Relaxed);
                    MODE.store(new, Ordering::Relaxed);


                }

                // ALL INPUT → SHELL
                _ => {
                    if MODE.load(Ordering::Relaxed) == 0 {
                        crate::shell::handle_key(c);
                    }
                }
            }
        }

        // =========================
        // IGNORE RAW KEYS
        // =========================
        DecodedKey::RawKey(_) => {}
    }

    did_work = true;
}
}
        // =========================
        // MOUSE HANDLING
        // =========================
        let x = crate::interrupts::mouse::MOUSE_X_ATOMIC.load(Ordering::Relaxed);
        let y = crate::interrupts::mouse::MOUSE_Y_ATOMIC.load(Ordering::Relaxed);
        let mode = MODE.load(Ordering::Relaxed);

       // ALWAYS process movement (not only on event)
if mode == 1 {
    crate::window::mouse_move(x, y);
}

// handle click separately
if crate::interrupts::mouse::MOUSE_CLICK.swap(false, Ordering::Relaxed) {
    if mode == 1 {
        crate::window::mouse_down(x, y);
    }
}

did_work = true;

        // always update cursor
        crate::drivers::cursor::update_cursor(x, y);
     if MODE.load(Ordering::Relaxed) == 1 {
    crate::drivers::cursor::draw_cursor(x, y);
}
    }
// =========================
// GUI DRAW
// =========================
// 🔥 ALWAYS DRAW GUI (FOR NOW)
crate::window::redraw();
crate::ui::draw_ui();
crate::ui::render_files();
    
}

// DO NOT SLEEP IN GUI MODE

}


use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::instructions::segmentation::{CS, DS, Segment};
use x86_64::instructions::tables::load_tss;
use x86_64::VirtAddr;

// bring println macro
use crate::println;

// =============================
// KERNEL STACK (VERY IMPORTANT)
// =============================
static mut STACK: [u8; 4096 * 5] = [0; 4096 * 5];

// =============================
// TSS
// =============================
static mut TSS: Option<TaskStateSegment> = None;

// =============================
// SELECTORS
// =============================
pub struct Selectors {
    pub kernel_code: SegmentSelector,
    pub kernel_data: SegmentSelector,
    pub user_code: SegmentSelector,
    pub user_data: SegmentSelector,
    pub tss: SegmentSelector,
}

// =============================
// GLOBAL GDT
// =============================
static mut GDT: Option<(GlobalDescriptorTable, Selectors)> = None;

// =============================
// INIT
// =============================
pub fn init() {
    // =============================
    // CREATE TSS
    // =============================
    let mut tss = TaskStateSegment::new();

    unsafe {
        let stack_start = VirtAddr::from_ptr(&STACK);
        let stack_end = stack_start + STACK.len();

        // 🔥 THIS FIXES YOUR CRASH
        tss.privilege_stack_table[0] = stack_end;
    }

    // =============================
    // CREATE GDT
    // =============================
    let mut gdt = GlobalDescriptorTable::new();

    let kernel_code = gdt.add_entry(Descriptor::kernel_code_segment());
    let kernel_data = gdt.add_entry(Descriptor::kernel_data_segment());

    let user_data = gdt.add_entry(Descriptor::user_data_segment());
    let user_code = gdt.add_entry(Descriptor::user_code_segment());

    let tss_selector = unsafe {
        TSS = Some(tss);
        gdt.add_entry(Descriptor::tss_segment(TSS.as_ref().unwrap()))
    };

    let selectors = Selectors {
        kernel_code,
        kernel_data,
        user_code,
        user_data,
        tss: tss_selector,
    };

    // =============================
    // LOAD GDT + SEGMENTS
    // =============================
    unsafe {
        GDT = Some((gdt, selectors));

        let (ref gdt, ref selectors) = GDT.as_ref().unwrap();

        gdt.load();

        CS::set_reg(selectors.kernel_code);
        DS::set_reg(selectors.kernel_data);

        // 🔥 LOAD TSS (CRITICAL)
        load_tss(selectors.tss);
    }

    println!("GDT initialized (with TSS + user mode)");
}

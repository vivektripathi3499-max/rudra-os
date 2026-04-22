use x86_64::structures::paging::Size4KiB;
use x86_64::{
    VirtAddr,
    PhysAddr,
    structures::paging::{
        OffsetPageTable,
        PageTable,
        Page,
        PhysFrame,
        Mapper,
        PageTableFlags,
    },
    registers::control::Cr3,
};

use spin::Mutex;

use crate::serial_println;

/* =========================
   GLOBAL MAPPER (IMPORTANT)
========================= */

static mut MAPPER: Option<Mutex<OffsetPageTable<'static>>> = None;

/* =========================
   INIT PAGING
========================= */

pub unsafe fn init(physical_memory_offset: VirtAddr) {

    serial_println!("paging::init -> start");

    let level_4_table = active_level_4_table(physical_memory_offset);

    serial_println!("paging::init -> L4 table loaded");

    let mapper = OffsetPageTable::new(level_4_table, physical_memory_offset);

    MAPPER = Some(Mutex::new(mapper));
}

/* =========================
   GET ACTIVE L4 TABLE
========================= */

unsafe fn active_level_4_table(
    physical_memory_offset: VirtAddr,
) -> &'static mut PageTable {

    serial_println!("paging::active_level_4_table -> reading CR3");

    let (level_4_frame, _) = Cr3::read();

    serial_println!("paging::active_level_4_table -> CR3 read OK");

    let phys = level_4_frame.start_address();

    serial_println!("L4 physical address: {:?}", phys);

    let virt = physical_memory_offset + phys.as_u64();

    serial_println!("L4 virtual address: {:?}", virt);

    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    serial_println!("Page table pointer created");

    let table = &mut *page_table_ptr;

    serial_println!("Page table dereferenced successfully");

    table
}

/* =========================
   FRAMEBUFFER MAPPING (FIX)
========================= */

pub unsafe fn map_framebuffer(
    phys_addr: u64,
    size: usize,
) {
    let mapper = MAPPER.as_ref().expect("Paging not initialized");
    let mut mapper = mapper.lock();

    let start = phys_addr;
    let end = phys_addr + size as u64;

    let mut addr = start;

    while addr < end {

        let frame: PhysFrame<Size4KiB> =
    PhysFrame::containing_address(PhysAddr::new(addr));
        let page: Page<Size4KiB> =
    Page::containing_address(VirtAddr::new(addr));

        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

        mapper
            .map_to(page, frame, flags, &mut DummyFrameAllocator)
            .expect("framebuffer map failed")
            .flush();

        addr += 4096;
    }
}

/* =========================
   SIMPLE FRAME ALLOCATOR
========================= */

struct DummyFrameAllocator;

unsafe impl x86_64::structures::paging::FrameAllocator<x86_64::structures::paging::Size4KiB>
    for DummyFrameAllocator
{
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        None
    }
}

/* =========================
   USER FLAGS
========================= */

pub fn user_page_flags() -> PageTableFlags {
    PageTableFlags::PRESENT
        | PageTableFlags::WRITABLE
        | PageTableFlags::USER_ACCESSIBLE
}

/* =========================
   USER STACK
========================= */
pub fn create_user_stack() -> VirtAddr {
    // TEMP: no real mapping yet
    VirtAddr::new(0x70000000 + 4096)
}

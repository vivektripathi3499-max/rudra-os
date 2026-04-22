use x86_64::{

    PhysAddr,
    structures::paging::{
        Mapper,
        Page,
        PhysFrame,
        Size4KiB,
        FrameAllocator,
        PageTableFlags,
    }
};

pub const KERNEL_START: u64 = 0xFFFF_8000_0000_0000;
pub const USER_START: u64   = 0x0000_0000_0000_0000;

pub fn create_example_mapping(
    page: Page,
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {

    let frame = PhysFrame::containing_address(
        PhysAddr::new(0xb8000)
    );

let flags = PageTableFlags::PRESENT
    | PageTableFlags::WRITABLE
    | PageTableFlags::NO_EXECUTE
    | PageTableFlags::GLOBAL; // 🔥 important

    let map_to_result =
        unsafe { mapper.map_to(page, frame, flags, frame_allocator) };

    map_to_result.expect("map_to failed").flush();
}

pub fn map_user_page(
    page: Page,
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    let frame = frame_allocator
        .allocate_frame()
        .expect("no frames available");

   let flags = PageTableFlags::PRESENT
    | PageTableFlags::WRITABLE
    | PageTableFlags::USER_ACCESSIBLE
    | PageTableFlags::NO_EXECUTE;
    
    unsafe {
        mapper.map_to(page, frame, flags, frame_allocator)
            .expect("map_to failed")
            .flush();
    }
}

pub fn map_user_code_page(
    page: Page,
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    let frame = frame_allocator
        .allocate_frame()
        .expect("no frames available");

    let flags = PageTableFlags::PRESENT
        | PageTableFlags::USER_ACCESSIBLE; // 🔥 executable (no NX)

    unsafe {
        mapper.map_to(page, frame, flags, frame_allocator)
            .expect("map_to failed")
            .flush();
    }
}

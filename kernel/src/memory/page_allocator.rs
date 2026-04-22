use x86_64::{
    VirtAddr,
    structures::paging::{
        Mapper,
        Page,
        PageTableFlags,
        Size4KiB,
        FrameAllocator,
    }
};

pub fn map_page(
    page: Page<Size4KiB>,
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {

    let frame = frame_allocator
        .allocate_frame()
        .expect("Failed to allocate frame");

    let flags =
        PageTableFlags::PRESENT |
        PageTableFlags::WRITABLE;

    unsafe {
        mapper.map_to(page, frame, flags, frame_allocator)
            .expect("map_to failed")
            .flush();
    }
}

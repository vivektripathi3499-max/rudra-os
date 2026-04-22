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

static mut NEXT_FREE: u64 = 0x4444_0000;

pub fn alloc_page(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> VirtAddr {

    let addr = unsafe { NEXT_FREE };

    unsafe {
        NEXT_FREE += 4096;
    }

    let page = Page::containing_address(VirtAddr::new(addr));

    let frame = frame_allocator
        .allocate_frame()
        .expect("No frames available");

    let flags =
        PageTableFlags::PRESENT |
        PageTableFlags::WRITABLE;

    unsafe {
        mapper
            .map_to(page, frame, flags, frame_allocator)
            .expect("map_to failed")
            .flush();
    }

    VirtAddr::new(addr)
}

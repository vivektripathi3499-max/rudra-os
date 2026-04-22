pub mod heap;
pub mod frame_allocator;
pub mod paging;
pub mod map;
pub mod page_allocator;
pub mod virt_allocator;

pub fn init() {
    heap::init_heap();
}

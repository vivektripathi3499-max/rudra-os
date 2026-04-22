use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub const HEAP_SIZE: usize = 1024 * 1024; // 1MB heap

static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

pub fn init_heap() {
    unsafe {
        ALLOCATOR.lock().init(
            HEAP.as_mut_ptr(),
            HEAP_SIZE,
        );
    }
}

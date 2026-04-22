use core::alloc::Layout;

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    panic!("allocation error");
}

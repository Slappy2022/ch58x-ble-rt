use core::alloc::GlobalAlloc;
use core::mem::MaybeUninit;
use embedded_alloc::Heap;

#[global_allocator]
static mut HEAP: Heap = Heap::empty();

#[export_name = "main"]
pub unsafe extern "C" fn _main() -> ! {
    extern "Rust" {
        fn rust_main() -> !;
    }
    {
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }
    rust_main()
}

#[no_mangle]
unsafe extern "C" fn rust_alloc(len: usize, align: usize) -> *mut u8 {
    match core::alloc::Layout::from_size_align(len, align) {
        Ok(layout) => HEAP.alloc(layout),
        Err(_) => core::ptr::null_mut(),
    }
}

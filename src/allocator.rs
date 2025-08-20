use core::{
    alloc::GlobalAlloc,
    sync::atomic::{AtomicUsize, Ordering},
};

#[repr(align(0x1000))]
struct Buf([u8; 0x1000_0000]);

static mut BUF: Buf = Buf([0; 0x1000_0000]);

struct Heap;

static INDEX: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for Heap {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let current_index = INDEX.fetch_add(layout.size(), Ordering::Relaxed);
        unsafe { &mut BUF.0[current_index] as *mut u8 }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {}
}

#[global_allocator]
static HEAP: Heap = Heap;

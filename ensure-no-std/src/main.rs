#![no_std]
#![no_main]
#![allow(dead_code, clippy::from_over_into)]

extern crate alloc;

use alloc::alloc::{GlobalAlloc, Layout};

struct NoopAllocator;

#[global_allocator]
static ALLOCATOR: NoopAllocator = NoopAllocator;

unsafe impl Sync for NoopAllocator {}

unsafe impl GlobalAlloc for NoopAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        unimplemented!()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        unimplemented!()
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

mod compat_test;

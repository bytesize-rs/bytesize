#![no_std]
#![no_main]
#![allow(dead_code, clippy::from_over_into)]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

mod compat_test;

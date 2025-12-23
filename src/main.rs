#![feature(step_trait)]
#![feature(new_range_api)]
#![no_std]
#![no_main]

mod memory;

use core::panic::PanicInfo;

core::arch::global_asm!(include_str!("start.s"));

unsafe extern "C" {
    static CORE_MASK: u8;
    static __bss_start: usize;
    static __bss_end: usize;
}

#[unsafe(no_mangle)]
pub extern "C" fn kinit(dtb: *const ()) {
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}

#![feature(step_trait)]
#![feature(new_range_api)]
#![no_std]
#![no_main]

mod memory;

use core::panic::PanicInfo;

core::arch::global_asm!(
    include_str!("start.s"),
    CORE_MASK = const 0b11,
    BOOT_CORE = const 0,
);

#[unsafe(no_mangle)]
pub extern "C" fn kinit() {
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}

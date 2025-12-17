#![feature(step_trait)]
#![feature(new_range_api)]
#![no_std]
#![no_main]

pub mod memory;

// TODO : remove
mod qemu;
mod uart;

use core::panic::PanicInfo;

use crate::memory::frame::SimpleFrameAllocator;

core::arch::global_asm!(include_str!("start.s"));

#[unsafe(no_mangle)]
pub extern "C" fn kmain() {
    let frame_allocator = unsafe {
        SimpleFrameAllocator::<4096>::new(qemu::memregs()).expect("enough memory for meta")
    };
    uart::out(format_args!("Memory: {:?}", frame_allocator.allocated()));

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    uart::out(format_args!("kernel panic: {}", info.message()));
    loop {}
}

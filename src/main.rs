#![feature(step_trait)]
#![feature(allocator_api)]
#![no_std]
#![no_main]

mod arch;
mod console;
mod global;
mod logger;
mod memory;
mod panic;
mod sync;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    log::set_max_level(log::LevelFilter::Trace);
    log::set_logger(&logger::GlobalConsoleLogger).expect("log may be initialized once");

    // TODO: For debug purposes
    // FIXME: re-setup when IDT is done, do logging via IRQ-s
    unsafe {
        *global::CONSOLE.lock() = console::serial::initialize();
    }

    // SAFETY: set in linker
    let (kernel_start, kernel_end) = unsafe { memory::kernel_bounds() };
    log::debug!("Kernel location: {kernel_start}-{kernel_end}");

    panic!("I'm done");
}

#![no_std]
#![no_main]

use core::{fmt::Write, panic::PanicInfo};

mod arch;
mod console;
mod global;
mod logger;
mod sync;

/// Panic is copying the loggger logic to prevent a loop in case log's write gives an error
/// (probably caused by incorrect args formatting)
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let console = &mut *global::CONSOLE.lock();

    // Skip an error to prevent panic loop
    let _ = if let Some(location) = info.location() {
        writeln!(
            console,
            "kernel panic at {} line {}:",
            location.file(),
            location.line(),
        )
    } else {
        writeln!(console, "kernel panic somewhere:")
    };
    let _ = writeln!(console, "{}", info.message());

    // TODO : generalize halt by arch
    unsafe { arch::x86_64::halt() }
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    log::set_max_level(log::LevelFilter::Trace);
    log::set_logger(&logger::GlobalConsoleLogger).unwrap();

    // TODO: For debug purposes
    // FIXME: re-setup when IDT is done, do logging via IRQ-s
    unsafe {
        *global::CONSOLE.lock() = console::serial::initialize();
    }

    log::warn!("I'm gonna panic rn");
    panic!("Hello World!");
}

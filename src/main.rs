#![no_std]
#![no_main]

use multiboot2::{BootInformation, BootInformationHeader};

mod arch;
mod console;
mod global;
mod logger;
mod panic;
mod sync;

#[unsafe(no_mangle)]
pub extern "C" fn _start(magic: u32, ptr: u32) -> ! {
    log::set_max_level(log::LevelFilter::Trace);
    log::set_logger(&logger::GlobalConsoleLogger).expect("log may be initialized once");

    // TODO: For debug purposes
    // FIXME: re-setup when IDT is done, do logging via IRQ-s
    unsafe {
        *global::CONSOLE.lock() = console::serial::initialize();
    }

    assert_eq!(
        magic,
        multiboot2::MAGIC,
        "Multiboot2 magic differs from the passed one"
    );

    // Safety: up to bootloader, verification ain't possible
    let boot_info = unsafe { BootInformation::load(ptr as *const BootInformationHeader).unwrap() };
    let mem_info = boot_info
        .memory_map_tag()
        .expect("mmap info not present in multiboot2");

    mem_info.memory_areas().iter().for_each(|memreg| {
        log::debug!(
            "Memory region with type {:?}: [{:#?}; {:#?}]",
            memreg.typ(),
            memreg.start_address(),
            memreg.end_address()
        );
    });

    panic!("I'm done");
}

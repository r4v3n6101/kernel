#![feature(step_trait)]
#![feature(ptr_as_uninit)]
#![feature(maybe_uninit_fill)]
#![no_std]
#![no_main]

use multiboot2::MemoryAreaType;

use crate::memory::{
    addr::PhysAddr,
    allocator::{early, frame},
};

mod arch;
mod console;
mod global;
mod logger;
mod memory;
mod panic;
mod sync;

#[unsafe(no_mangle)]
pub extern "C" fn _start(magic: u32, multiboot_addr: u32) -> ! {
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

    assert_eq!(
        magic,
        multiboot2::MAGIC,
        "Invalid multiboot2 magic: {magic:#x}",
    );

    // SAFETY: bootloader must be correct
    let multiboot2_hdr = unsafe {
        multiboot2::BootInformation::load(
            multiboot_addr as *const multiboot2::BootInformationHeader,
        )
        .expect("multiboot2 valid header")
    };
    let memmap = multiboot2_hdr
        .memory_map_tag()
        .expect("non-empty memory map tag");

    memmap.memory_areas().iter().for_each(|memreg| {
        log::info!(
            "[{:?}] Memory region: {}-{}",
            MemoryAreaType::from(memreg.typ()),
            memreg.start_address(),
            memreg.end_address()
        );
    });

    // TODO : change pagesize to const
    // SAFETY: there must be some memory right after kernel
    let mut early_alloc = unsafe { early::Bump::<4096>::new(kernel_end) };

    let mem_regions = memmap.memory_areas().iter().map(|memreg| {
        let start = PhysAddr(memreg.start_address() as usize);
        let end = PhysAddr(memreg.end_address() as usize);
        let typ = MemoryAreaType::from(memreg.typ());
        frame::Region {
            range: start..end,
            available: matches!(typ, MemoryAreaType::Available),
        }
    });

    let frame_alloc =
        unsafe { frame::BitvecAllocator::from_regions(&mut early_alloc, mem_regions) };

    log::info!(
        "Frames: available {}, occupied {}",
        frame_alloc.available_frames(),
        frame_alloc.occupied_frames()
    );

    panic!("I'm done");
}

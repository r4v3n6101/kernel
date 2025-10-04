// TODO : docs and x86 flags

use core::cell::LazyCell;

use crate::{arch::x86_64::port, console::Console};

const COM1: u16 = 0x3F8;

pub static mut SERIAL: LazyCell<Serial> = LazyCell::new(|| unsafe { Serial::initialize() });

pub struct Serial(());

impl Serial {
    // TODO : note about safety (can be initialized twice, however kinda fine)
    unsafe fn initialize() -> Self {
        unsafe {
            // Disable interrupts
            port::outb(COM1 + 1, 0x00);
            // Enable DLAB
            port::outb(COM1 + 3, 0x80);
            // Set baud (115200 / 3 = 38400)
            port::outb(COM1, 0x03);
            port::outb(COM1 + 1, 0x00);
            // 8 bits, no parity, one stop
            port::outb(COM1 + 3, 0x03);
            // Enable FIFO
            port::outb(COM1 + 2, 0xC7);
            // Modem control (RTS/DSR set)
            port::outb(COM1 + 4, 0x0B);
        }

        Self(())
    }

    // TODO : mut?
    unsafe fn serial_write(&self, byte: u8) {
        unsafe {
            // TODO : it may afterwards be replaced with IRQ
            while (port::inb(COM1 + 5) & 0x20) == 0 {}
            port::outb(COM1, byte);
        }
    }
}

impl Console for Serial {
    fn put(&self, b: u8) {
        // TODO : write about safety
        unsafe {
            self.serial_write(b);
        }
    }
}

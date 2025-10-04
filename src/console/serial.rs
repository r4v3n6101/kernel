// TODO : rewrite it

use core::ptr;

use crate::{
    arch::x86_64::port,
    console::{Console, ConsoleVTable},
};

pub unsafe fn initialize() -> Console {
    const COM1: u16 = 0x3F8;

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

    Console {
        data: ptr::null_mut(),
        vtable: &ConsoleVTable {
            put: |_, b| unsafe {
                while (port::inb(COM1 + 5) & 0x20) == 0 {}
                port::outb(COM1, b);
            },
        },
    }
}

use core::fmt::{self, Write};

struct UartWriter;

impl fmt::Write for UartWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        const UART0: *mut u8 = 0x0900_0000 as *mut u8;
        for b in s.bytes() {
            unsafe {
                core::ptr::write_volatile(UART0, b);
            }
        }
        Ok(())
    }
}

pub fn out(args: fmt::Arguments<'_>) {
    let _ = writeln!(&mut UartWriter, "{}", args);
}

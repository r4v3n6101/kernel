use core::fmt::{self, Write};

/// No-op console that just skips calls
pub mod noop;
pub mod serial;

pub trait Console {
    fn put(&self, b: u8);
}

impl Write for &dyn Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        s.bytes().for_each(|b| self.put(b));
        Ok(())
    }
}

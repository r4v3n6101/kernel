use core::fmt::{self, Write};

/// No-op console that just skips calls
pub mod noop;
pub mod serial;

pub struct Console {
    /// Owned data somewhere (may be nullptr in case not needed)
    pub data: *mut (),
    /// VTable as usual
    pub vtable: &'static ConsoleVTable,
}

/// Safety: the guarantee must be upheld by implementor.
unsafe impl Send for Console {}

#[repr(C)]
pub struct ConsoleVTable {
    pub put: unsafe fn(*mut (), u8),
}

impl Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // Safety: guaranties relies on the implementation and correct deref of the data arg
        s.bytes().for_each(|b| unsafe {
            (self.vtable.put)(self.data, b);
        });
        Ok(())
    }
}

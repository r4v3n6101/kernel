use core::iter;

use crate::memory::{addr::PhysSpace, region::MemRegion};

unsafe extern "C" {
    static __kernel_start: u8;
    static __kernel_end: u8;
}

pub fn memregs() -> impl Iterator<Item = (MemRegion<PhysSpace>, bool)> + Clone {
    unsafe {
        iter::once((
            MemRegion {
                // range: (&__kernel_end as *const u8).addr().into()..0x7FFF_FFFF.into(),
                range: 0x5000_0000.into()..0x7FFF_FFFF.into(),
            },
            true,
        ))
    }
}

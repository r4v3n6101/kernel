use core::ops::Range;

use crate::memory::addr::{PhysAddr, VirtAddr};

/// Page size
/// 4 KiB, classic
pub const DEFAULT_PAGE_SIZE: usize = 4096;

// TODO : docs
pub trait Paging {
    type PageFlags: Copy;

    fn map(addr: PhysAddr, vaddr: VirtAddr, flags: Self::PageFlags);
    fn unmap(addr: VirtAddr);

    fn map_range(addrs: Range<PhysAddr>, vaddr: VirtAddr, flags: Self::PageFlags) {
        addrs
            .enumerate()
            .for_each(|(i, addr)| Self::map(addr, vaddr + VirtAddr::from(i), flags));
    }

    fn unmap_range(vaddrs: Range<VirtAddr>) {
        vaddrs.for_each(|vaddr| Self::unmap(vaddr));
    }
}

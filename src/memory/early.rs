use core::mem;

use crate::memory::addr::Align;

use super::{DEFAULT_PAGE_SIZE, addr::PhysAddr};

pub type DefaultPageBump = Bump<DEFAULT_PAGE_SIZE>;

/// Simple bump allocator that just advance pointer and give next address.
/// All data is aligned to `Page` size to prevent unaligned reads after paging set up.
pub struct Bump<const PAGE: usize> {
    ptr: PhysAddr,
}

impl<const PAGE: usize> Bump<PAGE> {
    /// Create new bump allocator.
    /// It is up to you to check that there's enough memory
    pub fn new(ptr: PhysAddr) -> Bump<PAGE> {
        Self { ptr }
    }

    /// # Safety
    /// Up to user to be sure about memory validity to access.
    /// Early allocator shouldn't be used to allocate big chunks of memory.
    /// Internal pointer then shifts off by page size, so big allocation is better.
    pub unsafe fn alloc<T>(&mut self) -> PhysAddr {
        let new_addr = PhysAddr::from(self.ptr.as_mut_ptr::<T>().wrapping_add(1)).align_up(PAGE);
        mem::replace(&mut self.ptr, new_addr)
    }
}

use core::{
    alloc::{AllocError, Allocator, Layout},
    ops::Range,
    ptr::{self, NonNull},
};

use crate::sync::spin::SpinLock;

use super::{addr::PhysAddr, paging::DEFAULT_PAGE_SIZE};

pub type DefaultBump = SpinLock<Bump<DEFAULT_PAGE_SIZE>>;

/// Simple bump allocator that just advance pointer and give next address.
/// All data is aligned to `Page` size to prevent unaligned reads after paging set up.
pub struct Bump<const PAGE: usize> {
    start: PhysAddr,
    ptr: PhysAddr,
}

impl<const PAGE: usize> Bump<PAGE> {
    /// Create a new bump allocator setting up 2 pointer into start position.
    ///
    /// # Safety
    /// It's up to caller to be sure about memory access and overflow behaviour;
    pub unsafe fn new(start: PhysAddr) -> Self {
        Self { start, ptr: start }
    }

    /// Get a contiguous memory region of allocated memory
    pub fn region(&self) -> Range<PhysAddr> {
        self.start..self.ptr
    }
}

unsafe impl<const PAGE: usize> Allocator for SpinLock<Bump<PAGE>> {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let mut this = self.lock();

        let page_aligned_size = layout
            .align_to(PAGE)
            .map_err(|_| AllocError)?
            .pad_to_align()
            .size();

        // SAFETY: caller's job
        let allocated = unsafe {
            NonNull::new_unchecked(ptr::slice_from_raw_parts_mut(
                this.ptr.as_mut_ptr(),
                page_aligned_size,
            ))
        };

        this.ptr += PhysAddr::from(page_aligned_size);

        Ok(allocated)
    }

    unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {
        // no-op, i am bump
    }
}

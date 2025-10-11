use core::{mem, ops::Range, ptr};

use crate::memory::addr::{PhysAddr, align_up};

/// Simple bump allocator that just advance pointer and give next address.
/// Allocated data is aligned to `PAGE` size to prevent unaligned reads after paging set up.
pub struct Bump<const PAGE: usize> {
    start: PhysAddr,
    ptr: *mut u8,
}

impl<const PAGE: usize> Bump<PAGE> {
    /// Create a new bump allocator setting up 2 pointer into start position.
    ///
    /// # Safety
    /// It's up to caller to be sure about memory access and overflow behaviour;
    pub unsafe fn new(start: PhysAddr) -> Self {
        Self {
            start,
            ptr: start.as_mut_ptr(),
        }
    }

    /// Advance pointer giving page aligned pointer to memory.
    ///
    /// # Safety
    /// It is up to you to be sure about memory validity and uniqueness.
    pub unsafe fn bump(&mut self, size: usize) -> *mut [u8] {
        let page_aligned_size = align_up(size, PAGE);

        // SAFETY: upheld to caller
        let new_ptr = unsafe { self.ptr.byte_add(page_aligned_size) };
        let old_ptr = mem::replace(&mut self.ptr, new_ptr);

        ptr::slice_from_raw_parts_mut(old_ptr, page_aligned_size)
    }

    /// Get a contiguous memory region of allocated memory
    pub fn region(&self) -> Range<PhysAddr> {
        self.start..PhysAddr(self.ptr.addr())
    }
}

use core::ops::Range;

use bitvec::{order::Lsb0, slice::BitSlice};

use crate::memory::{
    addr::{PhysAddr, align_down, align_up},
    allocator::early,
};

/// TODO : make trait
/// Allocator of frames that tracks occupied ones by flipping bit in bitvec
pub struct BitvecAllocator<const PAGE: usize> {
    occupied: &'static mut BitSlice<u8, Lsb0>,
    frames: Range<usize>,
}

/// Memory region with flag of availability.
/// Just a shortcut for passing into [`BitvecAllocator`] initialization.
pub struct Region {
    pub range: Range<PhysAddr>,
    pub available: bool,
}

impl<const PAGE: usize> BitvecAllocator<PAGE> {
    /// Split available memory by frames with size of page.
    /// Store bitvec of occupied regions.
    ///
    /// # Safety
    /// Early allocator's memory must be valid to allocate by bitvec.
    pub unsafe fn from_regions<'a, I>(early: &mut early::Bump<PAGE>, iter: I) -> Self
    where
        I: IntoIterator<Item = Region> + Clone + 'a,
    {
        let (start, end) = iter
            .clone()
            .into_iter()
            .filter(|region| region.available)
            .fold((usize::MAX, usize::MIN), |(min, max), region| {
                (
                    min.min(region.range.start.into()),
                    max.max(region.range.end.into()),
                )
            });

        // Shrink available mem to be page size aligned
        let (framed_mem_start, framed_mem_end) =
            (align_up(start, PAGE) / PAGE, align_down(end, PAGE) / PAGE);

        // SAFETY: early allocator is fully up to you
        let bytes = unsafe {
            early
                .bump((framed_mem_end - framed_mem_start).div_ceil(u8::BITS as usize))
                .as_uninit_slice_mut()
                .unwrap_unchecked()
        };
        let bytes = bytes.write_filled(u8::MAX);
        let occupied = BitSlice::from_slice_mut(bytes);

        iter.into_iter()
            .filter(|region| region.available)
            .for_each(|region| {
                let (start, end) = (
                    usize::from(align_up(region.range.start, PAGE)) / PAGE,
                    usize::from(align_down(region.range.end, PAGE)) / PAGE,
                );

                let (start_idx, end_idx) = (start - framed_mem_start, end - framed_mem_start);
                occupied
                    .get_mut(start_idx..end_idx)
                    .expect("regions must fit in memory")
                    .fill(false);
            });

        Self {
            occupied,
            frames: framed_mem_start..framed_mem_end,
        }
    }

    pub fn metadata_size(&self) -> usize {
        self.occupied.len() / 8
    }

    pub fn available_frames(&self) -> usize {
        self.occupied.count_zeros()
    }

    pub fn occupied_frames(&self) -> usize {
        self.occupied.count_ones()
    }
}

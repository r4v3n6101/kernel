use core::{mem, ptr};

use super::addr::PhysAddr;

use super::{addr::PhysSpace, region::MemRegion};

pub struct FrameMeta {
    pub refcount: u64,
}

pub struct SimpleFrameAllocator<const FRAME_SIZE: usize> {
    first_frame_addr: PhysAddr,
    meta_ptr: *mut [FrameMeta],
}

impl<const FRAME_SIZE: usize> SimpleFrameAllocator<FRAME_SIZE> {
    pub unsafe fn new(
        regions: impl Iterator<Item = (MemRegion<PhysSpace>, bool)> + Clone,
    ) -> Option<Self> {
        // 1. Get all free memory bounds
        let mut begin = usize::MAX.into();
        let mut end = 0.into();
        for (reg, free) in regions.clone() {
            if free {
                begin = reg.range.start.min(begin);
                end = reg.range.end.max(end);
            }
        }
        if begin >= end {
            return None;
        }

        // 2. Calculate meta length
        let mem_reg = MemRegion { range: begin..end }.align_narrow(FRAME_SIZE);
        let first_frame_addr = mem_reg.range.start;
        let frames = {
            let addrs = mem_reg.len();
            debug_assert!(addrs % FRAME_SIZE == 0);

            addrs / FRAME_SIZE
        };
        let meta_bytes = frames * mem::size_of::<FrameMeta>();

        // 3. Find start and region for metadata
        let meta_start = regions.clone().find_map(|(reg, free)| {
            if free {
                let reg_aligned = reg.align_narrow(FRAME_SIZE);
                if reg_aligned.at_least(meta_bytes) {
                    return Some(reg_aligned.range.start);
                }
            }

            None
        })?;

        let meta_reg = MemRegion {
            range: meta_start..(meta_start + meta_bytes.into()).align_up(FRAME_SIZE),
        };
        let meta_ptr = meta_start.as_mut_ptr().cast::<FrameMeta>();

        // 4. Mark everything as allocated
        for i in 0..frames {
            // SAFETY: you should be sure about availability of regions
            unsafe {
                meta_ptr.add(i).write(FrameMeta { refcount: 1 });
            }
        }

        // 5. Free eveything except meta
        for (reg, free) in regions.clone() {
            if free {
                let reg_aligned = reg.align_narrow(FRAME_SIZE);
                for addr in reg_aligned.range.step_by(FRAME_SIZE) {
                    // Do not free meta
                    if !meta_reg.range.contains(&addr) {
                        let frame = (addr - first_frame_addr).addr() / FRAME_SIZE;
                        unsafe {
                            (&mut *meta_ptr.add(frame)).refcount = 0;
                        }
                    }
                }
            }
        }

        Some(Self {
            first_frame_addr,
            meta_ptr: ptr::slice_from_raw_parts_mut(meta_ptr, frames),
        })
    }

    pub fn allocated(&self) -> (usize, usize) {
        // SAFETY: data must be valid until allocator is alive
        let free = unsafe {
            (&*self.meta_ptr)
                .iter()
                .filter(|meta| meta.refcount == 0)
                .count()
        };
        let all = self.meta_ptr.len();

        (free, all)
    }
}

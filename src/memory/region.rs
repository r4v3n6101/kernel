use core::ops::Range;

use super::addr::TypedAddr;

#[derive(derive_more::with_trait::Debug)]
#[debug(bounds())]
pub struct MemRegion<S> {
    pub range: Range<TypedAddr<S>>,
}

impl<S> Clone for MemRegion<S> {
    fn clone(&self) -> Self {
        Self {
            range: self.range.clone(),
        }
    }
}

impl<S> MemRegion<S> {
    pub fn align_narrow(&self, align: usize) -> Self {
        Self {
            range: self.range.start.align_up(align)..self.range.end.align_down(align),
        }
    }

    pub fn at_least(&self, need: usize) -> bool {
        (self.range.end - self.range.start).addr() > need
    }

    pub fn len(&self) -> usize {
        self.range.clone().count()
    }
}

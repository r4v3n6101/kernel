use core::ops::{Add, Range};

#[derive(
    Debug,
    PartialEq,
    Copy,
    Clone,
    derive_more::Display,
    derive_more::From,
    derive_more::Add,
    derive_more::AddAssign,
    derive_more::Deref,
    derive_more::DerefMut,
)]
#[display("{_0:#x}")]
#[repr(transparent)]
pub struct PhysAddr(usize);

impl PhysAddr {
    pub fn as_ptr<T>(self) -> *const T {
        self.0 as *const T
    }

    pub fn as_mut_ptr<T>(self) -> *mut T {
        self.0 as *mut T
    }
}

impl From<PhysAddr> for usize {
    fn from(val: PhysAddr) -> Self {
        val.0
    }
}

impl<T> From<*const T> for PhysAddr {
    fn from(value: *const T) -> Self {
        Self(value.addr())
    }
}

impl<T> From<*mut T> for PhysAddr {
    fn from(value: *mut T) -> Self {
        Self(value.addr())
    }
}

pub trait Overlap: Sized {
    /// Check whether 2 regions are overlapping
    fn overlaps(&self, other: &Self) -> bool;

    fn is_contiguous_with(&self, other: &Self) -> bool;

    /// Tries to merge into 1 contiguous region
    fn merge(&self, other: &Self) -> Option<Self>;
}

pub trait Align
where
    Self: Sized,
    Self: Into<usize>,
    Self: From<usize>,
{
    fn align_up(self, align: usize) -> Self {
        debug_assert!(align > 1);
        ((self.into() + align - 1) & !(align - 1)).into()
    }

    fn align_down(self, align: usize) -> Self {
        (self.into() & !(align - 1)).into()
    }
}

impl Align for PhysAddr {}

impl<T> Overlap for Range<T>
where
    T: Copy + Ord + Add<Output = T> + From<usize>,
{
    fn overlaps(&self, other: &Self) -> bool {
        self.start.max(other.start) <= self.end.min(other.end)
    }

    fn is_contiguous_with(&self, other: &Self) -> bool {
        self.end + 1.into() == other.start || self.start == other.end + 1.into()
    }

    fn merge(&self, other: &Self) -> Option<Self> {
        if self.overlaps(other) || self.is_contiguous_with(other) {
            let start = self.start.min(other.start);
            let end = self.end.max(other.end);
            Some(start..end)
        } else {
            None
        }
    }
}

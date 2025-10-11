use core::iter::Step;

macro_rules! addr_def {
    ($name: ident) => {
        #[derive(
            Debug,
            PartialEq,
            PartialOrd,
            Ord,
            Eq,
            Copy,
            Clone,
            derive_more::Display,
            derive_more::From,
            derive_more::Add,
            derive_more::AddAssign,
        )]
        #[display("{_0:#x}")]
        #[repr(transparent)]
        pub struct $name(pub usize);

        impl $name {
            pub fn as_ptr<T>(self) -> *const T {
                self.0 as *const T
            }

            pub fn as_mut_ptr<T>(self) -> *mut T {
                self.0 as *mut T
            }
        }

        impl From<$name> for usize {
            fn from(val: $name) -> Self {
                val.0
            }
        }

        impl Step for $name {
            fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
                ((*start).into(), Some((*end).into()))
            }

            fn forward_checked(start: Self, count: usize) -> Option<Self> {
                usize::from(start).checked_add(count).map(Into::into)
            }

            fn backward_checked(start: Self, count: usize) -> Option<Self> {
                usize::from(start).checked_sub(count).map(Into::into)
            }
        }
    };
}

addr_def!(PhysAddr);
addr_def!(VirtAddr);

pub fn align_up<T>(addr: T, align: usize) -> T
where
    T: From<usize>,
    T: Into<usize>,
{
    debug_assert!(align > 1);
    ((addr.into() + align - 1) & !(align - 1)).into()
}

pub fn align_down<T>(addr: T, align: usize) -> T
where
    T: From<usize>,
    T: Into<usize>,
{
    debug_assert!(align > 1);
    (addr.into() & !(align - 1)).into()
}

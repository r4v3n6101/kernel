use core::{cmp::Ordering, marker::PhantomData, range::Step};

pub type PhysAddr = TypedAddr<PhysSpace>;
pub type VirtAddr = TypedAddr<VirtSpace>;

pub struct PhysSpace;
pub struct VirtSpace;

#[derive(
    derive_more::with_trait::Into,
    derive_more::with_trait::Debug,
    derive_more::with_trait::Display,
    derive_more::with_trait::Add,
    derive_more::with_trait::Sub,
    derive_more::with_trait::AddAssign,
    derive_more::with_trait::SubAssign,
    derive_more::with_trait::Eq,
    derive_more::with_trait::PartialEq,
)]
#[debug("{_0:#x}")]
#[display("{_0}")]
pub struct TypedAddr<S>(
    usize,
    #[into(skip)]
    #[sub(skip)]
    #[sub_assign(skip)]
    #[add(skip)]
    #[add_assign(skip)]
    PhantomData<S>,
);

impl<S> From<usize> for TypedAddr<S> {
    fn from(value: usize) -> Self {
        Self(value, PhantomData)
    }
}

impl<S> Clone for TypedAddr<S> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<S> Copy for TypedAddr<S> {}

impl<S> PartialOrd for TypedAddr<S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<S> Ord for TypedAddr<S> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<S> Step for TypedAddr<S> {
    fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
        Step::steps_between(&start.0, &end.0)
    }

    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        Step::forward_checked(start.0, count).map(Into::into)
    }

    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        Step::backward_checked(start.0, count).map(Into::into)
    }
}

impl<S> TypedAddr<S> {
    pub fn addr(&self) -> usize {
        self.0
    }

    /// Aligns a number **up** to the next boundary.
    ///
    /// # Examples
    ///
    /// ```
    /// use kernel::memory::addr::TypedAddr;
    ///
    /// let addr: TypedAddr<()> = 0.into();
    /// let align: usize = 4096;
    /// assert_eq!(addr.align_up(align).addr(), 0);
    ///
    /// let addr: TypedAddr<()> = 4095.into();
    /// let align: usize = 4096;
    /// assert_eq!(addr.align_up(align).addr(), 4096);
    /// ```
    pub fn align_up(self, align: usize) -> Self {
        Self((self.0 + align - 1) & !(align - 1), PhantomData)
    }

    /// Aligns a number **down** to the nearest boundary.
    ///
    /// # Examples
    ///
    /// ```
    /// use kernel::memory::addr::TypedAddr;
    ///
    /// let addr: TypedAddr<()> = 0.into();
    /// let align: usize = 4096;
    /// assert_eq!(addr.align_down(align).addr(), 0);
    ///
    /// let addr: TypedAddr<()> = 5000.into();
    /// let align: usize = 4096;
    /// assert_eq!(addr.align_down(align).addr(), 4096);
    /// ```
    pub fn align_down(self, align: usize) -> Self {
        Self(self.0 & !(align - 1), PhantomData)
    }

    pub fn as_ptr(self) -> *const () {
        self.addr() as _
    }

    pub fn as_mut_ptr(self) -> *mut () {
        self.addr() as _
    }
}

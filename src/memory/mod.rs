pub mod addr;
pub mod early;

/// Page size
/// 4 KiB, classic
pub const DEFAULT_PAGE_SIZE: usize = 4096;

/// Retrieve kernel's start and end addresses.
///
/// # Safety
/// Usually linker sets up it, but it may be absent.
/// For example, in ARM where it can't be mapped as in x86.
/// So it may be up to caller to set `_kernel_start` and `_kernel_end`
pub unsafe fn kernel_bounds() -> (addr::PhysAddr, addr::PhysAddr) {
    unsafe extern "C" {
        static _kernel_start: u8;
        static _kernel_end: u8;
    }

    (
        addr::PhysAddr::from(&raw const _kernel_start),
        addr::PhysAddr::from(&raw const _kernel_end),
    )
}

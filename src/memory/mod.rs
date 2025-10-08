pub mod addr;
pub mod early;
pub mod paging;

/// Retrieve kernel's start and end addresses.
///
/// # Safety
/// Caller must be sure about variables set up.
/// Won't work after paging if start/end aren't mapped linearly.
pub unsafe fn kernel_bounds() -> (addr::PhysAddr, addr::PhysAddr) {
    unsafe extern "C" {
        static _kernel_start: u8;
        static _kernel_end: u8;
    }

    let start_ptr = &raw const _kernel_start;
    let end_ptr = &raw const _kernel_end;

    (
        addr::PhysAddr::from(start_ptr.addr()),
        addr::PhysAddr::from(end_ptr.addr()),
    )
}

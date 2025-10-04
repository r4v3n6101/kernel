pub mod port;

#[inline]
pub unsafe fn halt() {
    unsafe {
        core::arch::asm!("hlt", options(nomem, nostack, preserves_flags),);
    }
}

#[inline]
pub unsafe fn outb(port: u16, val: u8) {
    unsafe {
        core::arch::asm!(
            "out dx, al",
            in("dx") port,
            in("al") val,
            options(nomem, nostack, preserves_flags),
        );
    }
}

#[inline]
pub unsafe fn inb(port: u16) -> u8 {
    unsafe {
        let ret: u8;
        core::arch::asm!(
            "in al, dx",
            in("dx") port,
            out("al") ret,
            options(nomem, nostack, preserves_flags),
        );
        ret
    }
}

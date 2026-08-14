use core::arch::asm;

/// Reads one byte from an x86 I/O port.
///
/// # Safety
///
/// The caller must ensure that `port` is a valid I/O port for the
/// current hardware configuration.
#[inline]
pub unsafe fn inb(port: u16) -> u8 {
    let value: u8;

    unsafe {
        asm!(
            "in al, dx",
            in("dx") port,
            out("al") value,
            options(nomem, nostack, preserves_flags),
        );
    }

    value
}

/// Writes one byte to an x86 I/O port.
///
/// # Safety
///
/// The caller must ensure that `port` is a valid I/O port for the
/// current hardware configuration.
#[inline]
pub unsafe fn outb(port: u16, value: u8) {
    unsafe {
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") value,
            options(nomem, nostack, preserves_flags),
        );
    }
}
use core::arch::asm;

const DATA_PORT: u16 = 0x60;
const STATUS_PORT: u16 = 0x64;

#[inline(always)]
fn read_port(port: u16) -> u8 {
    let value: u8;

    unsafe {
        asm!(
            "in al, dx",
            in("dx") port,
            out("al") value,
            options(nomem, nostack, preserves_flags)
        );
    }

    value
}

pub fn init() {
    // PS/2 controller initialization is intentionally minimal
    // for QEMU/legacy PS/2 keyboard support.
}

#[inline(always)]
pub fn status() -> u8 {
    read_port(STATUS_PORT)
}

#[inline(always)]
pub fn read_scancode() -> Option<u8> {
    // Output buffer full?
    if read_port(STATUS_PORT) & 0x01 == 0 {
        return None;
    }

    Some(read_port(DATA_PORT))
}

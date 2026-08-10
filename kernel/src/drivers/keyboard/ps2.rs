use core::arch::asm;

const DATA_PORT: u16 = 0x60;
const STATUS_PORT: u16 = 0x64;

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

pub fn init() {}

pub fn status() -> u8 {
    read_port(STATUS_PORT)
}

pub fn read_scancode() -> Option<u8> {
    let status = read_port(STATUS_PORT);

    if status & 1 == 0 {
        return None;
    }

    Some(read_port(DATA_PORT))
}

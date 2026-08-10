use crate::drivers::keyboard::ps2;

pub fn init() {
    ps2::init();
}

pub fn status() -> u8 {
    ps2::status()
}

pub fn poll_scancode() -> Option<u8> {
    ps2::read_scancode()
}

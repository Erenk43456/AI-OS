pub fn is_key_release(scancode: u8) -> bool {
    scancode & 0x80 != 0
}

pub fn strip_release_bit(scancode: u8) -> u8 {
    scancode & 0x7F
}

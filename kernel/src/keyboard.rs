use core::arch::asm;
use core::cell::UnsafeCell;

use pc_keyboard::{
layouts,
DecodedKey,
HandleControl,
Keyboard,
ScancodeSet1,
};

struct KeyboardState {
inner: UnsafeCell<Option<Keyboard<layouts::Us104Key, ScancodeSet1>>>,
}

unsafe impl Sync for KeyboardState {}

static KEYBOARD: KeyboardState = KeyboardState {
inner: UnsafeCell::new(None),
};

pub fn init() {
unsafe {
*KEYBOARD.inner.get() = Some(Keyboard::new(
ScancodeSet1::new(),
layouts::Us104Key,
HandleControl::Ignore,
));
}
}

fn keyboard_available() -> bool {
let status: u8;

unsafe {
    asm!(
        "in al, dx",
        in("dx") 0x64u16,
        out("al") status,
        options(nomem, nostack, preserves_flags)
    );
}

status & 1 != 0

}

fn read_scancode() -> u8 {
let scancode: u8;

unsafe {
    asm!(
        "in al, dx",
        in("dx") 0x60u16,
        out("al") scancode,
        options(nomem, nostack, preserves_flags)
    );
}

scancode

}

pub fn read_char() -> Option<u8> {
if !keyboard_available() {
return None;
}

let scancode = read_scancode();

unsafe {
    let keyboard = &mut *KEYBOARD.inner.get();

    let keyboard = keyboard.as_mut()?;

    let key_event = keyboard.add_byte(scancode).ok()??;

    let decoded_key = keyboard.process_keyevent(key_event)?;

    match decoded_key {
        DecodedKey::Unicode(character) => {
            if character.is_ascii() {
                Some(character as u8)
            } else {
                None
            }
        }

        DecodedKey::RawKey(_) => None,
    }
}

}
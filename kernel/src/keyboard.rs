use crate::drivers::keyboard::ps2;

use pc_keyboard::{
    layouts,
    DecodedKey,
    HandleControl,
    Keyboard,
    ScancodeSet1,
};

use spin::Mutex;

const INPUT_BUFFER_SIZE: usize = 256;
const DEBUG_BUFFER_SIZE: usize = 64;

struct InputBuffer {
    data: [u8; INPUT_BUFFER_SIZE],
    read: usize,
    write: usize,
}

impl InputBuffer {
    const fn new() -> Self {
        Self {
            data: [0; INPUT_BUFFER_SIZE],
            read: 0,
            write: 0,
        }
    }

    fn push(&mut self, byte: u8) {
        let next = (self.write + 1) % INPUT_BUFFER_SIZE;

        if next == self.read {
            return;
        }

        self.data[self.write] = byte;
        self.write = next;
    }

    fn pop(&mut self) -> Option<u8> {
        if self.read == self.write {
            return None;
        }

        let byte = self.data[self.read];
        self.read = (self.read + 1) % INPUT_BUFFER_SIZE;

        Some(byte)
    }
}

struct DebugBuffer {
    data: [u8; DEBUG_BUFFER_SIZE],
    len: usize,
}

impl DebugBuffer {
    const fn new() -> Self {
        Self {
            data: [0; DEBUG_BUFFER_SIZE],
            len: 0,
        }
    }

    fn set(&mut self, data: &[u8]) {
        self.len = 0;

        let mut i = 0;

        while i < data.len() && i < DEBUG_BUFFER_SIZE {
            self.data[i] = data[i];
            self.len += 1;
            i += 1;
        }
    }

    fn read(&mut self) -> Option<u8> {
        if self.len == 0 {
            return None;
        }

        let byte = self.data[0];

        let mut i = 1;

        while i < self.len {
            self.data[i - 1] = self.data[i];
            i += 1;
        }

        self.len -= 1;

        Some(byte)
    }
}

static KEYBOARD: Mutex<Option<Keyboard<layouts::Us104Key, ScancodeSet1>>> =
    Mutex::new(None);

static INPUT: Mutex<InputBuffer> =
    Mutex::new(InputBuffer::new());

static DEBUG: Mutex<DebugBuffer> =
    Mutex::new(DebugBuffer::new());

pub fn init() {
    ps2::init();

    let keyboard = Keyboard::new(
        ScancodeSet1::new(),
        layouts::Us104Key,
        HandleControl::Ignore,
    );

    *KEYBOARD.lock() = Some(keyboard);
}

fn process_scancode(scancode: u8) {
    // ============================================================
    // SCANCODE DEBUG
    // ============================================================

    let mut debug = [0u8; 16];

    debug[0] = b'S';
    debug[1] = b'C';
    debug[2] = b':';
    debug[3] = b' ';

    debug[4] = b'0';
    debug[5] = b'x';

    let hex = b"0123456789ABCDEF";

    debug[6] = hex[(scancode >> 4) as usize];
    debug[7] = hex[(scancode & 0x0F) as usize];

    debug[8] = b' ';
    debug[9] = b' ';

    DEBUG.lock().set(&debug[..10]);

    // ============================================================
    // NORMAL KEYBOARD DECODING
    // ============================================================

    let mut keyboard_guard = KEYBOARD.lock();

    let Some(keyboard) = keyboard_guard.as_mut() else {
        return;
    };

    let Ok(Some(event)) = keyboard.add_byte(scancode) else {
        return;
    };

    let Some(key) = keyboard.process_keyevent(event) else {
        return;
    };

    match key {
        DecodedKey::Unicode(character) => {
            if character.is_ascii() {
                INPUT.lock().push(character as u8);
            }
        }

        DecodedKey::RawKey(_) => {}
    }
}

pub fn poll() {
    while let Some(scancode) = ps2::read_scancode() {
        process_scancode(scancode);
    }
}

pub fn read() -> Option<u8> {
    INPUT.lock().pop()
}

pub fn read_debug() -> Option<u8> {
    DEBUG.lock().read()
}

pub fn handle_interrupt() {
    poll();
}
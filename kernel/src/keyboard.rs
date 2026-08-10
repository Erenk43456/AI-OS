use crate::drivers::keyboard::ps2;

use pc_keyboard::{DecodedKey, HandleControl, Keyboard, ScancodeSet1, layouts};

use spin::Mutex;

const INPUT_BUFFER_SIZE: usize = 256;
const MAP_BUFFER_SIZE: usize = 64;

// ============================================================
// INPUT BUFFER
// ============================================================

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

// ============================================================
// SCANCODE DEBUG BUFFER
// ============================================================

struct MapBuffer {
    data: [u8; MAP_BUFFER_SIZE],
    len: usize,
}

impl MapBuffer {
    const fn new() -> Self {
        Self {
            data: [0; MAP_BUFFER_SIZE],
            len: 0,
        }
    }

    fn push(&mut self, byte: u8) {
        if self.len < MAP_BUFFER_SIZE {
            self.data[self.len] = byte;
            self.len += 1;
        }
    }

    fn pop(&mut self) -> Option<u8> {
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

// ============================================================
// KEYBOARD STATE
// ============================================================

struct KeyboardState {
    shift_left: bool,
    shift_right: bool,
    caps_lock: bool,
}

impl KeyboardState {
    const fn new() -> Self {
        Self {
            shift_left: false,
            shift_right: false,
            caps_lock: false,
        }
    }

    fn shift(&self) -> bool {
        self.shift_left || self.shift_right
    }
}

// ============================================================
// GLOBALS
// ============================================================

static KEYBOARD: Mutex<Option<Keyboard<layouts::Us104Key, ScancodeSet1>>> = Mutex::new(None);

static INPUT: Mutex<InputBuffer> = Mutex::new(InputBuffer::new());

static MAP_OUTPUT: Mutex<MapBuffer> = Mutex::new(MapBuffer::new());

static KEYBOARD_STATE: Mutex<KeyboardState> = Mutex::new(KeyboardState::new());

// ============================================================
// INIT
// ============================================================

pub fn init() {
    ps2::init();

    let keyboard = Keyboard::new(
        ScancodeSet1::new(),
        layouts::Us104Key,
        HandleControl::Ignore,
    );

    *KEYBOARD.lock() = Some(keyboard);
}

// ============================================================
// HEX
// ============================================================

fn hex_digit(value: u8) -> u8 {
    match value {
        0..=9 => b'0' + value,
        _ => b'A' + (value - 10),
    }
}

// ============================================================
// DEBUG SCANCODE
// ============================================================

fn push_map_scancode(scancode: u8) {
    let mut output = MAP_OUTPUT.lock();

    output.push(b'S');
    output.push(b'C');
    output.push(b':');
    output.push(b'0');
    output.push(b'x');

    output.push(hex_digit(scancode >> 4));
    output.push(hex_digit(scancode & 0x0F));

    output.push(b'\n');
}

// ============================================================
// TÜRKÇE Q KLAVYE
// ============================================================
//
// Senin cihazındaki gözleme göre:
//
// 0x1A -> Ğ / ğ
// 0x1B -> Ü / ü
// 0x27 -> Ş / ş
// 0x28 -> İ / i
// 0x26 -> I / ı
// 0x33 -> Ö / ö
// 0x34 -> Ç / ç
//
// ============================================================

fn turkish_character(scancode: u8, uppercase: bool) -> Option<u8> {
    match scancode {
        // ====================================================
        // Ğ / ğ
        // ====================================================
        0x1A => {
            if uppercase {
                Some(0x86) // Ğ
            } else {
                Some(0x80) // ğ
            }
        }

        // ====================================================
        // Ü / ü
        // ====================================================
        0x1B => {
            if uppercase {
                Some(0x87) // Ü
            } else {
                Some(0x81) // ü
            }
        }

        // ====================================================
        // Ş / ş
        // ====================================================
        0x27 => {
            if uppercase {
                Some(0x88) // Ş
            } else {
                Some(0x82) // ş
            }
        }

        // ====================================================
        // İ / i
        // ====================================================
        0x28 => {
            if uppercase {
                Some(0x89) // İ
            } else {
                Some(0x83) // i
            }
        }

        // ====================================================
        // I / ı
        // ====================================================
        0x26 => {
            if uppercase {
                Some(0x8D) // I
            } else {
                Some(0x8C) // ı
            }
        }

        // ====================================================
        // Ö / ö
        // ====================================================
        0x33 => {
            if uppercase {
                Some(0x8A) // Ö
            } else {
                Some(0x84) // ö
            }
        }

        // ====================================================
        // Ç / ç
        // ====================================================
        0x34 => {
            if uppercase {
                Some(0x8B) // Ç
            } else {
                Some(0x85) // ç
            }
        }

        _ => None,
    }
}

// ============================================================
// PROCESS SCANCODE
// ============================================================

fn process_scancode(scancode: u8) {
    // ========================================================
    // DEBUG
    // ========================================================

    push_map_scancode(scancode);

    // ========================================================
    // SHIFT STATE
    // ========================================================

    {
        let mut state = KEYBOARD_STATE.lock();

        match scancode {
            // Left Shift press
            0x2A => {
                state.shift_left = true;
            }

            // Left Shift release
            0xAA => {
                state.shift_left = false;
            }

            // Right Shift press
            0x36 => {
                state.shift_right = true;
            }

            // Right Shift release
            0xB6 => {
                state.shift_right = false;
            }

            _ => {}
        }
    }

    // ========================================================
    // CAPS LOCK
    // ========================================================

    if scancode == 0x3A {
        let mut state = KEYBOARD_STATE.lock();

        state.caps_lock = !state.caps_lock;
    }

    // ========================================================
    // RELEASE
    // ========================================================

    if scancode & 0x80 != 0 {
        let mut keyboard_guard = KEYBOARD.lock();

        let Some(keyboard) = keyboard_guard.as_mut() else {
            return;
        };

        let _ = keyboard.add_byte(scancode);

        return;
    }

    // ========================================================
    // STATE
    // ========================================================

    let (shift, caps_lock);

    {
        let state = KEYBOARD_STATE.lock();

        shift = state.shift();
        caps_lock = state.caps_lock;
    }

    // Shift XOR Caps Lock
    let uppercase = shift ^ caps_lock;

    // ========================================================
    // TÜRKÇE KARAKTER
    // ========================================================

    if let Some(character) = turkish_character(scancode, uppercase) {
        INPUT.lock().push(character);
        return;
    }

    // ========================================================
    // NORMAL PC KEYBOARD
    // ========================================================

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

// ============================================================
// POLL
// ============================================================

pub fn poll() {
    while let Some(scancode) = ps2::read_scancode() {
        process_scancode(scancode);
    }
}

// ============================================================
// READ
// ============================================================

pub fn read() -> Option<u8> {
    INPUT.lock().pop()
}

// ============================================================
// DEBUG OUTPUT
// ============================================================

pub fn read_map_output() -> Option<u8> {
    MAP_OUTPUT.lock().pop()
}

// ============================================================
// INTERRUPT
// ============================================================

pub fn handle_interrupt() {
    poll();
}

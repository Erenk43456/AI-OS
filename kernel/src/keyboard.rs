use crate::drivers::keyboard::ps2;
use crate::input::event::InputEvent;
use crate::input::queue;

use pc_keyboard::{
    DecodedKey,
    HandleControl,
    Keyboard,
    KeyCode,
    ScancodeSet1,
};

use spin::Mutex;

const MAP_BUFFER_SIZE: usize = 64;

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
    left_shift: bool,
    right_shift: bool,
    caps_lock: bool,
}

impl KeyboardState {
    const fn new() -> Self {
        Self {
            left_shift: false,
            right_shift: false,
            caps_lock: false,
        }
    }

    fn shift(&self) -> bool {
        self.left_shift || self.right_shift
    }
}

// ============================================================
// GLOBALS
// ============================================================

static KEYBOARD: Mutex<Option<Keyboard<layouts::Us104Key, ScancodeSet1>>> =
    Mutex::new(None);

static MAP_OUTPUT: Mutex<MapBuffer> =
    Mutex::new(MapBuffer::new());

static KEYBOARD_STATE: Mutex<KeyboardState> =
    Mutex::new(KeyboardState::new());

// ============================================================
// INIT
// ============================================================

pub fn init() {
    ps2::init();

    let keyboard = Keyboard::new(
        ScancodeSet1::new(),
        crate::keyboard_layout::TurkishQ,
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
// TÜRKÇE Q
// ============================================================
//
// Set 1 scancode'ları fiziksel tuşlara göre ele alıyoruz.
//
// 0x1A = Ğ / ğ
// 0x1B = Ü / ü
// 0x27 = Ş / ş
// 0x28 = İ / i
// 0x33 = Ö / ö
// 0x34 = Ç / ç
//
// uppercase:
//     false = küçük
//     true  = büyük
//
// ============================================================

fn turkish_character(
    scancode: u8,
    uppercase: bool,
) -> Option<u8> {
    match scancode {

        // ====================================================
        // I / ı
        // ====================================================

        0x16 => {
            if uppercase {
                Some(0x8C)
            } else {
                Some(0x8D)
            }
        }

        // ====================================================
        // Ğ / ğ
        // ====================================================

        0x1A => {
            if uppercase {
                Some(0x86)
            } else {
                Some(0x80)
            }
        }

        // ====================================================
        // Ü / ü
        // ====================================================

        0x1B => {
            if uppercase {
                Some(0x87)
            } else {
                Some(0x81)
            }
        }

        // ====================================================
        // Ş / ş
        // ====================================================

        0x27 => {
            if uppercase {
                Some(0x88)
            } else {
                Some(0x82)
            }
        }

        // ====================================================
        // İ / i
        // ====================================================

        0x28 => {
            if uppercase {
                Some(0x89)
            } else {
                Some(0x83)
            }
        }

        // ====================================================
        // Ö / ö
        // ====================================================

        0x33 => {
            if uppercase {
                Some(0x8A)
            } else {
                Some(0x84)
            }
        }

        // ====================================================
        // Ç / ç
        // ====================================================

        0x34 => {
            if uppercase {
                Some(0x8B)
            } else {
                Some(0x85)
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
    // SHIFT
    // ========================================================

    {
        let mut state = KEYBOARD_STATE.lock();

        match scancode {

            // Left Shift press
            0x2A => {
                state.left_shift = true;
                return;
            }

            // Left Shift release
            0xAA => {
                state.left_shift = false;
                return;
            }

            // Right Shift press
            0x36 => {
                state.right_shift = true;
                return;
            }

            // Right Shift release
            0xB6 => {
                state.right_shift = false;
                return;
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

        return;
    }

    // ========================================================
    // RELEASE
    // ========================================================

    if scancode & 0x80 != 0 {
        return;
    }

    // ========================================================
    // DIRECT ARROW MAPPING
    // ========================================================
    //
    // Set 1:
    //
    // 0x48 = Up
    // 0x50 = Down
    // 0x4B = Left
    // 0x4D = Right
    //
    // Şimdilik doğrudan event olarak ele alıyoruz.
    //
    // Böylece bunların:
    //
    //     8
    //     2
    //     4
    //     6
    //
    // olarak sisteme düşmesini engelliyoruz.
    //
    // ========================================================

    match scancode {

        // Up
        0x48 => {
            queue::push(InputEvent::ArrowUp);
            return;
        }

        // Down
        0x50 => {
            queue::push(InputEvent::ArrowDown);
            return;
        }

        // Left
        0x4B => {
            queue::push(InputEvent::ArrowLeft);
            return;
        }

        // Right
        0x4D => {
            queue::push(InputEvent::ArrowRight);
            return;
        }

        _ => {}
    }

    // ========================================================
    // GET KEYBOARD STATE
    // ========================================================

    let (shift, caps_lock);

    {
        let state = KEYBOARD_STATE.lock();

        shift = state.shift();
        caps_lock = state.caps_lock;
    }

    // ========================================================
    // LETTER CASE
    // ========================================================

    let uppercase = shift ^ caps_lock;

    // ========================================================
    // TÜRKÇE KARAKTERLER
    // ========================================================

    if let Some(character) =
        turkish_character(scancode, uppercase)
    {
        queue::push(
            InputEvent::KeyPress(character)
        );

        return;
    }

    // ========================================================
    // NORMAL KEYBOARD
    // ========================================================

    let mut keyboard_guard = KEYBOARD.lock();

    let Some(keyboard) = keyboard_guard.as_mut() else {
        return;
    };

    let Ok(Some(event)) =
        keyboard.add_byte(scancode)
    else {
        return;
    };

    let Some(key) =
        keyboard.process_keyevent(event)
    else {
        return;
    };

    // ========================================================
    // DECODED KEY
    // ========================================================

    match key {

        // ====================================================
        // UNICODE / ASCII
        // ====================================================

        DecodedKey::Unicode(character) => {

            if character.is_ascii() {

                let mut output =
                    character as u8;

                // ============================================
                // ASCII LETTER CASE
                // ============================================

                if output >= b'a'
                    && output <= b'z'
                {
                    if uppercase {
                        output -= 32;
                    }
                }
                else if output >= b'A'
                    && output <= b'Z'
                {
                    if !uppercase {
                        output += 32;
                    }
                }

                queue::push(
                    InputEvent::KeyPress(output)
                );
            }
        }

        // ====================================================
        // SPECIAL KEYS
        // ====================================================

        DecodedKey::RawKey(keycode) => {

            match keycode {

                // ============================================
                // ARROWS
                // ============================================

                KeyCode::ArrowUp => {
                    queue::push(
                        InputEvent::ArrowUp
                    );
                }

                KeyCode::ArrowDown => {
                    queue::push(
                        InputEvent::ArrowDown
                    );
                }

                KeyCode::ArrowLeft => {
                    queue::push(
                        InputEvent::ArrowLeft
                    );
                }

                KeyCode::ArrowRight => {
                    queue::push(
                        InputEvent::ArrowRight
                    );
                }

                // ============================================
                // BACKSPACE
                // ============================================

                KeyCode::Backspace => {
                    queue::push(
                        InputEvent::Backspace
                    );
                }

                // ============================================
                // ENTER
                // ============================================

                KeyCode::Return => {
                    queue::push(
                        InputEvent::Enter
                    );
                }

                // ============================================
                // TAB
                // ============================================

                KeyCode::Tab => {
                    queue::push(
                        InputEvent::Tab
                    );
                }

                // ============================================
                // OTHER
                // ============================================

                _ => {}
            }
        }
    }
}

// ============================================================
// POLL
// ============================================================

pub fn poll() {
    while let Some(scancode) =
        ps2::read_scancode()
    {
        process_scancode(scancode);
    }
}

// ============================================================
// READ
// ============================================================
//
// Yeni input sistemi queue üzerinden çalışıyor.
// Bu fonksiyon şimdilik uyumluluk amacıyla tutuluyor.
//

pub fn read() -> Option<InputEvent> {
    queue::pop()
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
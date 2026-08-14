pub mod layout;

use crate::drivers::keyboard::ps2::{
    controller::Controller,
    keyboard::Ps2Keyboard,
};

use crate::input::{
    event::InputEvent,
    queue,
};

use spin::Mutex;

// ============================================================
// KEYBOARD STATE
// ============================================================

static PS2_KEYBOARD: Mutex<Option<Ps2Keyboard>> =
    Mutex::new(None);

static EXTENDED_SCANCODE: Mutex<bool> =
    Mutex::new(false);

static SHIFT_PRESSED: Mutex<bool> =
    Mutex::new(false);

static CAPS_LOCK: Mutex<bool> =
    Mutex::new(false);

// ============================================================
// INIT
// ============================================================

pub fn init() {
    let controller = Controller::new();

    let keyboard = Ps2Keyboard::new(controller);

    if keyboard.init().is_err() {
        return;
    }

    *PS2_KEYBOARD.lock() = Some(keyboard);
}

// ============================================================
// INTERRUPT
// ============================================================

pub fn handle_interrupt() {
    let keyboard = PS2_KEYBOARD.lock();

    let Some(keyboard) = keyboard.as_ref() else {
        return;
    };

    let Ok(Some(scancode)) =
        keyboard.read_scancode()
    else {
        return;
    };

    // ========================================================
    // EXTENDED SCANCODE PREFIX
    // ========================================================

    if scancode == 0xE0 {
        *EXTENDED_SCANCODE.lock() = true;
        return;
    }

    let extended = {
        let mut state =
            EXTENDED_SCANCODE.lock();

        let value = *state;

        *state = false;

        value
    };

    // ========================================================
    // SHIFT
    // ========================================================

    if !extended {
        match scancode {
            // Left Shift press
            0x2A => {
                *SHIFT_PRESSED.lock() = true;
                return;
            }

            // Right Shift press
            0x36 => {
                *SHIFT_PRESSED.lock() = true;
                return;
            }

            // Left Shift release
            0xAA => {
                *SHIFT_PRESSED.lock() = false;
                return;
            }

            // Right Shift release
            0xB6 => {
                *SHIFT_PRESSED.lock() = false;
                return;
            }

            _ => {}
        }
    }

    // ========================================================
    // CAPS LOCK
    // ========================================================

    if !extended {
        match scancode {
            // Caps Lock press
            0x3A => {
                let mut caps =
                    CAPS_LOCK.lock();

                *caps = !*caps;

                return;
            }

            // Caps Lock release
            0xBA => {
                return;
            }

            _ => {}
        }
    }

    // ========================================================
    // DECODE
    // ========================================================

    if let Some(event) =
        decode_scancode(scancode, extended)
    {
        queue::push(event);
    }
}

// ============================================================
// SCANCODE DECODER
// ============================================================

fn decode_scancode(
    scancode: u8,
    extended: bool,
) -> Option<InputEvent> {
    let released =
        scancode & 0x80 != 0;

    let code =
        scancode & 0x7F;

    // ========================================================
    // EXTENDED KEYS
    // ========================================================

    if extended {
        if released {
            return None;
        }

        return match code {
            // Arrow Up
            0x48 =>
                Some(InputEvent::ArrowUp),

            // Arrow Down
            0x50 =>
                Some(InputEvent::ArrowDown),

            // Arrow Left
            0x4B =>
                Some(InputEvent::ArrowLeft),

            // Arrow Right
            0x4D =>
                Some(InputEvent::ArrowRight),

            _ => None,
        };
    }

    // ========================================================
    // KEY RELEASE
    // ========================================================

    if released {
        return None;
    }

    // ========================================================
    // SPECIAL KEYS
    // ========================================================

    match code {
        // Backspace
        0x0E => {
            return Some(
                InputEvent::Backspace
            );
        }

        // Enter
        0x1C => {
            return Some(
                InputEvent::Enter
            );
        }

        // Tab
        0x0F => {
            return Some(
                InputEvent::Tab
            );
        }

        _ => {}
    }

    // ========================================================
    // KEYBOARD STATE
    // ========================================================

    let shift =
        *SHIFT_PRESSED.lock();

    let caps_lock =
        *CAPS_LOCK.lock();

    // ========================================================
    // CHARACTER
    // ========================================================

    let character =
        layout::translate_with_shift(
            code,
            shift,
        )?;

    // ========================================================
    // CAPS LOCK
    // ========================================================

    let character =
        apply_caps_lock(
            character,
            caps_lock,
            shift,
        );

    Some(
        InputEvent::KeyPress(character)
    )
}

// ============================================================
// CAPS LOCK CHARACTER TRANSFORMATION
// ============================================================
//
// Turkish keyboard rules:
//
// Normal:
//     i -> i
//     ı -> ı
//
// Shift:
//     i -> İ
//     ı -> I
//
// Caps Lock:
//     i -> İ
//     ı -> I
//
// Caps Lock + Shift:
//     i -> i
//     ı -> ı
//
// The same inverse behavior applies to the other
// Turkish characters.
//
// Symbols and numbers are NOT affected by Caps Lock.
// ============================================================

fn apply_caps_lock(
    character: char,
    caps_lock: bool,
    shift: bool,
) -> char {
    if !caps_lock {
        return character;
    }

    match (character, shift) {
        // ====================================================
        // TURKISH CHARACTERS
        // ====================================================

        // i <-> İ
        ('i', false) => 'İ',
        ('İ', true) => 'i',

        // ı <-> I
        ('ı', false) => 'I',
        ('I', true) => 'ı',

        // ğ <-> Ğ
        ('ğ', false) => 'Ğ',
        ('Ğ', true) => 'ğ',

        // ü <-> Ü
        ('ü', false) => 'Ü',
        ('Ü', true) => 'ü',

        // ş <-> Ş
        ('ş', false) => 'Ş',
        ('Ş', true) => 'ş',

        // ö <-> Ö
        ('ö', false) => 'Ö',
        ('Ö', true) => 'ö',

        // ç <-> Ç
        ('ç', false) => 'Ç',
        ('Ç', true) => 'ç',

        // ====================================================
        // ENGLISH CHARACTERS
        // ====================================================

        // Lowercase -> Uppercase
        ('a', false) => 'A',
        ('b', false) => 'B',
        ('c', false) => 'C',
        ('d', false) => 'D',
        ('e', false) => 'E',
        ('f', false) => 'F',
        ('g', false) => 'G',
        ('h', false) => 'H',
        ('j', false) => 'J',
        ('k', false) => 'K',
        ('l', false) => 'L',
        ('m', false) => 'M',
        ('n', false) => 'N',
        ('o', false) => 'O',
        ('p', false) => 'P',
        ('q', false) => 'Q',
        ('r', false) => 'R',
        ('s', false) => 'S',
        ('t', false) => 'T',
        ('u', false) => 'U',
        ('v', false) => 'V',
        ('w', false) => 'W',
        ('x', false) => 'X',
        ('y', false) => 'Y',
        ('z', false) => 'Z',

        // Uppercase -> Lowercase when Shift is held
        ('A', true) => 'a',
        ('B', true) => 'b',
        ('C', true) => 'c',
        ('D', true) => 'd',
        ('E', true) => 'e',
        ('F', true) => 'f',
        ('G', true) => 'g',
        ('H', true) => 'h',
        ('J', true) => 'j',
        ('K', true) => 'k',
        ('L', true) => 'l',
        ('M', true) => 'm',
        ('N', true) => 'n',
        ('O', true) => 'o',
        ('P', true) => 'p',
        ('Q', true) => 'q',
        ('R', true) => 'r',
        ('S', true) => 's',
        ('T', true) => 't',
        ('U', true) => 'u',
        ('V', true) => 'v',
        ('W', true) => 'w',
        ('X', true) => 'x',
        ('Y', true) => 'y',
        ('Z', true) => 'z',

        // ====================================================
        // EVERYTHING ELSE
        // ====================================================
        //
        // Numbers and symbols remain unchanged.
        //
        _ => character,
    }
}
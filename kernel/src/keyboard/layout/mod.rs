use super::keycode::KeyCode;

#[derive(Clone, Copy, Debug, Default)]
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub alt_gr: bool,
}

pub fn translate_keycode(
    key: KeyCode,
    modifiers: Modifiers,
) -> Option<char> {
    let shift = modifiers.shift;

    if modifiers.alt_gr {
        return translate_alt_gr(key, shift);
    }

    match key {
        // ====================================================
        // NUMBER ROW
        // ====================================================

        KeyCode::Num1 =>
            Some(if shift { '!' } else { '1' }),

        KeyCode::Num2 =>
            Some(if shift { '"' } else { '2' }),

        KeyCode::Num3 =>
            Some(if shift { '^' } else { '3' }),

        KeyCode::Num4 =>
            Some(if shift { '+' } else { '4' }),

        KeyCode::Num5 =>
            Some(if shift { '%' } else { '5' }),

        KeyCode::Num6 =>
            Some(if shift { '&' } else { '6' }),

        KeyCode::Num7 =>
            Some(if shift { '/' } else { '7' }),

        KeyCode::Num8 =>
            Some(if shift { '(' } else { '8' }),

        KeyCode::Num9 =>
            Some(if shift { ')' } else { '9' }),

        KeyCode::Num0 =>
            Some(if shift { '=' } else { '0' }),

        KeyCode::StarQuestion =>
            Some(if shift { '?' } else { '*' }),

        KeyCode::Minus =>
            Some(if shift { '_' } else { '-' }),

        // ====================================================
        // Q ROW
        // ====================================================

        KeyCode::Q =>
            Some(if shift { 'Q' } else { 'q' }),

        KeyCode::W =>
            Some(if shift { 'W' } else { 'w' }),

        KeyCode::E =>
            Some(if shift { 'E' } else { 'e' }),

        KeyCode::R =>
            Some(if shift { 'R' } else { 'r' }),

        KeyCode::T =>
            Some(if shift { 'T' } else { 't' }),

        KeyCode::Y =>
            Some(if shift { 'Y' } else { 'y' }),

        KeyCode::U =>
            Some(if shift { 'U' } else { 'u' }),

        KeyCode::I =>
            Some(if shift { 'I' } else { 'ı' }),

        KeyCode::O =>
            Some(if shift { 'O' } else { 'o' }),

        KeyCode::P =>
            Some(if shift { 'P' } else { 'p' }),

        KeyCode::LeftBracket =>
            Some(if shift { 'Ğ' } else { 'ğ' }),

        KeyCode::RightBracket =>
            Some(if shift { 'Ü' } else { 'ü' }),

        // ====================================================
        // HOME ROW
        // ====================================================

        KeyCode::A =>
            Some(if shift { 'A' } else { 'a' }),

        KeyCode::S =>
            Some(if shift { 'S' } else { 's' }),

        KeyCode::D =>
            Some(if shift { 'D' } else { 'd' }),

        KeyCode::F =>
            Some(if shift { 'F' } else { 'f' }),

        KeyCode::G =>
            Some(if shift { 'G' } else { 'g' }),

        KeyCode::H =>
            Some(if shift { 'H' } else { 'h' }),

        KeyCode::J =>
            Some(if shift { 'J' } else { 'j' }),

        KeyCode::K =>
            Some(if shift { 'K' } else { 'k' }),

        KeyCode::L =>
            Some(if shift { 'L' } else { 'l' }),

        KeyCode::Semicolon =>
            Some(if shift { 'Ş' } else { 'ş' }),

        KeyCode::Apostrophe =>
            Some(if shift { 'İ' } else { 'i' }),

        KeyCode::Grave =>
            Some(if shift { 'é' } else { '"' }),

        // ====================================================
        // ISO EXTRA KEY
        // ====================================================

        KeyCode::Oem102 =>
            Some(if shift { '>' } else { '<' }),

        // ====================================================
        // BOTTOM ROW
        // ====================================================

        KeyCode::Z =>
            Some(if shift { 'Z' } else { 'z' }),

        KeyCode::X =>
            Some(if shift { 'X' } else { 'x' }),

        KeyCode::C =>
            Some(if shift { 'C' } else { 'c' }),

        KeyCode::V =>
            Some(if shift { 'V' } else { 'v' }),

        KeyCode::B =>
            Some(if shift { 'B' } else { 'b' }),

        KeyCode::N =>
            Some(if shift { 'N' } else { 'n' }),

        KeyCode::M =>
            Some(if shift { 'M' } else { 'm' }),

        // Türkçe Q:
        // Ö
        KeyCode::Comma =>
            Some(if shift { 'Ö' } else { 'ö' }),

        // Ç
        KeyCode::Oem2 =>
            Some(if shift { 'Ç' } else { 'ç' }),

        // :
        KeyCode::Oem5 =>
            Some(if shift { ':' } else { '.' }),

        // ====================================================
        // SPACE
        // ====================================================

        KeyCode::Space => Some(' '),

        _ => None,
    }
}

// ============================================================
// ALT GR
// ============================================================

fn translate_alt_gr(
    key: KeyCode,
    shift: bool,
) -> Option<char> {
    match (key, shift) {
        // ----------------------------------------------------
        // Number row
        // ----------------------------------------------------

        (KeyCode::Num3, false) => Some('#'),
        (KeyCode::Num3, true) => Some('£'),

        (KeyCode::Num4, false) => Some('$'),
        (KeyCode::Num4, true) => Some('+'),

        (KeyCode::Num6, false) => Some('½'),

        (KeyCode::Num7, false) => Some('{'),
        (KeyCode::Num7, true) => Some('['),

        (KeyCode::Num8, false) => Some('['),
        (KeyCode::Num8, true) => Some(']'),

        (KeyCode::Num9, false) => Some(']'),
        (KeyCode::Num9, true) => Some('}'),

        (KeyCode::Num0, false) => Some('}'),

        (KeyCode::StarQuestion, false) => Some('\\'),
        (KeyCode::StarQuestion, true) => Some('|'),

        // ----------------------------------------------------
        // Q row
        // ----------------------------------------------------

        (KeyCode::Q, false) => Some('@'),

        (KeyCode::E, false) => Some('€'),

        (KeyCode::T, false) => Some('₺'),

        // ----------------------------------------------------
        // I / İ
        // ----------------------------------------------------

        (KeyCode::I, false) => Some('ı'),
        (KeyCode::I, true) => Some('İ'),

        // ----------------------------------------------------
        // Ü
        // ----------------------------------------------------

        (KeyCode::RightBracket, false) => Some('~'),

        // ----------------------------------------------------
        // ISO
        // ----------------------------------------------------

        (KeyCode::Oem102, false) => Some('|'),
        (KeyCode::Oem102, true) => Some('\\'),

        // ----------------------------------------------------
        // Home row
        // ----------------------------------------------------

        (KeyCode::Semicolon, false) => Some('´'),

        (KeyCode::Apostrophe, false) => Some('`'),

        // ----------------------------------------------------
        // Bottom row
        // ----------------------------------------------------

        (KeyCode::Comma, false) => Some('¨'),

        _ => None,
    }
}

// ============================================================
// CAPS LOCK
// ============================================================

pub fn apply_caps_lock(
    character: char,
    caps_lock: bool,
    shift: bool,
) -> char {
    if !caps_lock {
        return character;
    }

    match (character, shift) {
        ('a', false) => 'A',
        ('b', false) => 'B',
        ('c', false) => 'C',
        ('ç', false) => 'Ç',
        ('d', false) => 'D',
        ('e', false) => 'E',
        ('f', false) => 'F',
        ('g', false) => 'G',
        ('ğ', false) => 'Ğ',
        ('h', false) => 'H',
        ('ı', false) => 'I',
        ('i', false) => 'İ',
        ('j', false) => 'J',
        ('k', false) => 'K',
        ('l', false) => 'L',
        ('m', false) => 'M',
        ('n', false) => 'N',
        ('o', false) => 'O',
        ('ö', false) => 'Ö',
        ('p', false) => 'P',
        ('q', false) => 'Q',
        ('r', false) => 'R',
        ('s', false) => 'S',
        ('ş', false) => 'Ş',
        ('t', false) => 'T',
        ('u', false) => 'U',
        ('ü', false) => 'Ü',
        ('v', false) => 'V',
        ('w', false) => 'W',
        ('x', false) => 'X',
        ('y', false) => 'Y',
        ('z', false) => 'Z',

        ('A', true) => 'a',
        ('B', true) => 'b',
        ('C', true) => 'c',
        ('Ç', true) => 'ç',
        ('D', true) => 'd',
        ('E', true) => 'e',
        ('F', true) => 'f',
        ('G', true) => 'g',
        ('Ğ', true) => 'ğ',
        ('H', true) => 'h',
        ('I', true) => 'ı',
        ('İ', true) => 'i',
        ('J', true) => 'j',
        ('K', true) => 'k',
        ('L', true) => 'l',
        ('M', true) => 'm',
        ('N', true) => 'n',
        ('O', true) => 'o',
        ('Ö', true) => 'ö',
        ('P', true) => 'p',
        ('Q', true) => 'q',
        ('R', true) => 'r',
        ('S', true) => 's',
        ('Ş', true) => 'ş',
        ('T', true) => 't',
        ('U', true) => 'u',
        ('Ü', true) => 'ü',
        ('V', true) => 'v',
        ('W', true) => 'w',
        ('X', true) => 'x',
        ('Y', true) => 'y',
        ('Z', true) => 'z',

        _ => character,
    }
}
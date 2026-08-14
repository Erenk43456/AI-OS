use super::Modifiers;
use crate::keyboard::keycode::KeyCode;

pub fn translate(
    key: KeyCode,
    modifiers: Modifiers,
) -> Option<char> {
    let shift = modifiers.shift;
    let caps = modifiers.caps_lock;

    match key {
        // ====================================================
        // NUMBER ROW
        // ====================================================

        KeyCode::Num1 => {
            Some(if shift { '!' } else { '1' })
        }

        KeyCode::Num2 => {
            Some(if shift { '"' } else { '2' })
        }

        KeyCode::Num3 => {
            Some(if shift { '^' } else { '3' })
        }

        KeyCode::Num4 => {
            Some(if shift { '+' } else { '4' })
        }

        KeyCode::Num5 => {
            Some(if shift { '%' } else { '5' })
        }

        KeyCode::Num6 => {
            Some(if shift { '&' } else { '6' })
        }

        KeyCode::Num7 => {
            Some(if shift { '/' } else { '7' })
        }

        KeyCode::Num8 => {
            Some(if shift { '(' } else { '8' })
        }

        KeyCode::Num9 => {
            Some(if shift { ')' } else { '9' })
        }

        KeyCode::Num0 => {
            Some(if shift { '=' } else { '0' })
        }

        // * / ?
        KeyCode::StarQuestion => {
            Some(if shift { '?' } else { '*' })
        }

        // - / _
        KeyCode::Minus => {
            Some(if shift { '_' } else { '-' })
        }

        // ====================================================
        // Q ROW
        // ====================================================

        KeyCode::Q => alpha('q', 'Q', shift, caps),
        KeyCode::W => alpha('w', 'W', shift, caps),
        KeyCode::E => alpha('e', 'E', shift, caps),
        KeyCode::R => alpha('r', 'R', shift, caps),
        KeyCode::T => alpha('t', 'T', shift, caps),
        KeyCode::Y => alpha('y', 'Y', shift, caps),
        KeyCode::U => alpha('u', 'U', shift, caps),

        // ı / I
        KeyCode::I => turkish_i(shift, caps),

        KeyCode::O => alpha('o', 'O', shift, caps),
        KeyCode::P => alpha('p', 'P', shift, caps),

        // ğ / Ğ
        KeyCode::LeftBracket => {
            alpha('ğ', 'Ğ', shift, caps)
        }

        // ü / Ü
        KeyCode::RightBracket => {
            alpha('ü', 'Ü', shift, caps)
        }

        // ====================================================
        // HOME ROW
        // ====================================================

        KeyCode::A => alpha('a', 'A', shift, caps),
        KeyCode::S => alpha('s', 'S', shift, caps),
        KeyCode::D => alpha('d', 'D', shift, caps),
        KeyCode::F => alpha('f', 'F', shift, caps),
        KeyCode::G => alpha('g', 'G', shift, caps),
        KeyCode::H => alpha('h', 'H', shift, caps),
        KeyCode::J => alpha('j', 'J', shift, caps),
        KeyCode::K => alpha('k', 'K', shift, caps),
        KeyCode::L => alpha('l', 'L', shift, caps),

        // ş / Ş
        KeyCode::Semicolon => {
            alpha('ş', 'Ş', shift, caps)
        }

        // i / İ
        KeyCode::Apostrophe => {
            alpha('i', 'İ', shift, caps)
        }

        // " / é
        KeyCode::Grave => {
            Some(if shift { 'é' } else { '"' })
        }

        // ====================================================
        // COMMA
        // ====================================================

        // , / ;
        KeyCode::Comma => {
            Some(if shift { ';' } else { ',' })
        }

        // ====================================================
        // BOTTOM ROW
        // ====================================================

        KeyCode::Z => alpha('z', 'Z', shift, caps),
        KeyCode::X => alpha('x', 'X', shift, caps),
        KeyCode::C => alpha('c', 'C', shift, caps),
        KeyCode::V => alpha('v', 'V', shift, caps),
        KeyCode::B => alpha('b', 'B', shift, caps),
        KeyCode::N => alpha('n', 'N', shift, caps),
        KeyCode::M => alpha('m', 'M', shift, caps),

        // ö / Ö
        KeyCode::Oem2 => {
            alpha('ö', 'Ö', shift, caps)
        }

        // ç / Ç
        KeyCode::Oem5 => {
            alpha('ç', 'Ç', shift, caps)
        }

        // . / :
        KeyCode::Period => {
            Some(if shift { ':' } else { '.' })
        }

        // ====================================================
        // SPACE
        // ====================================================

        KeyCode::Space => Some(' '),

        // ====================================================
        // NON CHARACTER
        // ====================================================

        _ => None,
    }
}

fn alpha(
    lower: char,
    upper: char,
    shift: bool,
    caps: bool,
) -> Option<char> {
    if shift ^ caps {
        Some(upper)
    } else {
        Some(lower)
    }
}

fn turkish_i(
    shift: bool,
    caps: bool,
) -> Option<char> {
    match (shift, caps) {
        (false, false) => Some('ı'),
        (true, false) => Some('I'),
        (false, true) => Some('I'),
        (true, true) => Some('ı'),
    }
}
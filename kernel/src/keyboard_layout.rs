use pc_keyboard::{DecodedKey, HandleControl, KeyCode, KeyboardLayout, Modifiers};

#[derive(Debug, Copy, Clone)]
pub struct TurkishQ;

impl TurkishQ {
    fn letter(lower: char, upper: char, modifiers: &Modifiers) -> DecodedKey {
        if modifiers.is_caps() {
            DecodedKey::Unicode(upper)
        } else {
            DecodedKey::Unicode(lower)
        }
    }

    fn shifted_or(normal: char, shifted: char, modifiers: &Modifiers) -> DecodedKey {
        if modifiers.is_shifted() {
            DecodedKey::Unicode(shifted)
        } else {
            DecodedKey::Unicode(normal)
        }
    }

    fn raw(keycode: KeyCode) -> DecodedKey {
        DecodedKey::RawKey(keycode)
    }
}

impl KeyboardLayout for TurkishQ {
    fn map_keycode(
        &self,
        keycode: KeyCode,
        modifiers: &Modifiers,
        handle_ctrl: HandleControl,
    ) -> DecodedKey {
        /*
         * Ctrl + A-Z
         *
         * MapLettersToUnicode davranışını burada kendimiz uyguluyoruz.
         */
        if modifiers.is_ctrl() {
            if let Some(control) = control_character(keycode) {
                return match handle_ctrl {
                    HandleControl::MapLettersToUnicode => DecodedKey::Unicode(control),

                    HandleControl::Ignore => Self::raw(keycode),
                };
            }
        }

        match keycode {
            // ------------------------------------------------------------
            // FUNCTION KEYS
            // ------------------------------------------------------------
            KeyCode::Escape => Self::raw(KeyCode::Escape),

            KeyCode::F1 => Self::raw(KeyCode::F1),
            KeyCode::F2 => Self::raw(KeyCode::F2),
            KeyCode::F3 => Self::raw(KeyCode::F3),
            KeyCode::F4 => Self::raw(KeyCode::F4),
            KeyCode::F5 => Self::raw(KeyCode::F5),
            KeyCode::F6 => Self::raw(KeyCode::F6),
            KeyCode::F7 => Self::raw(KeyCode::F7),
            KeyCode::F8 => Self::raw(KeyCode::F8),
            KeyCode::F9 => Self::raw(KeyCode::F9),
            KeyCode::F10 => Self::raw(KeyCode::F10),
            KeyCode::F11 => Self::raw(KeyCode::F11),
            KeyCode::F12 => Self::raw(KeyCode::F12),

            KeyCode::PrintScreen => Self::raw(KeyCode::PrintScreen),
            KeyCode::ScrollLock => Self::raw(KeyCode::ScrollLock),
            KeyCode::PauseBreak => Self::raw(KeyCode::PauseBreak),

            // ------------------------------------------------------------
            // NUMBER ROW — TURKISH Q
            // ------------------------------------------------------------
            KeyCode::Key1 => Self::shifted_or('1', '!', modifiers),

            KeyCode::Key2 => Self::shifted_or('2', '"', modifiers),

            KeyCode::Key3 => Self::shifted_or('3', '^', modifiers),

            KeyCode::Key4 => Self::shifted_or('4', '+', modifiers),

            KeyCode::Key5 => Self::shifted_or('5', '%', modifiers),

            KeyCode::Key6 => Self::shifted_or('6', '&', modifiers),

            KeyCode::Key7 => Self::shifted_or('7', '/', modifiers),

            KeyCode::Key8 => Self::shifted_or('8', '(', modifiers),

            KeyCode::Key9 => Self::shifted_or('9', ')', modifiers),

            KeyCode::Key0 => Self::shifted_or('0', '=', modifiers),

            KeyCode::OemMinus => Self::shifted_or('*', '?', modifiers),

            KeyCode::OemPlus => Self::shifted_or('-', '_', modifiers),

            // ------------------------------------------------------------
            // BACKSPACE / TAB
            // ------------------------------------------------------------
            KeyCode::Backspace => Self::raw(KeyCode::Backspace),

            KeyCode::Tab => DecodedKey::Unicode('\t'),

            // ------------------------------------------------------------
            // Q ROW
            // ------------------------------------------------------------
            KeyCode::Q => Self::letter('q', 'Q', modifiers),

            KeyCode::W => Self::letter('w', 'W', modifiers),

            KeyCode::E => Self::letter('e', 'E', modifiers),

            KeyCode::R => Self::letter('r', 'R', modifiers),

            KeyCode::T => Self::letter('t', 'T', modifiers),

            KeyCode::Y => Self::letter('y', 'Y', modifiers),

            KeyCode::U => Self::letter('u', 'U', modifiers),

            // Türkçe Q:
            // I tuşu = ı / I
            KeyCode::I => Self::letter('ı', 'I', modifiers),

            KeyCode::O => Self::letter('o', 'O', modifiers),

            KeyCode::P => Self::letter('p', 'P', modifiers),

            // Ğ
            KeyCode::Oem4 => Self::letter('ğ', 'Ğ', modifiers),

            // Ü
            KeyCode::Oem6 => Self::letter('ü', 'Ü', modifiers),

            // ------------------------------------------------------------
            // ENTER
            // ------------------------------------------------------------
            KeyCode::Return => DecodedKey::Unicode('\n'),

            // ------------------------------------------------------------
            // ASDF ROW
            // ------------------------------------------------------------
            KeyCode::CapsLock => Self::raw(KeyCode::CapsLock),

            KeyCode::A => Self::letter('a', 'A', modifiers),

            KeyCode::S => Self::letter('s', 'S', modifiers),

            KeyCode::D => Self::letter('d', 'D', modifiers),

            KeyCode::F => Self::letter('f', 'F', modifiers),

            KeyCode::G => Self::letter('g', 'G', modifiers),

            KeyCode::H => Self::letter('h', 'H', modifiers),

            KeyCode::J => Self::letter('j', 'J', modifiers),

            KeyCode::K => Self::letter('k', 'K', modifiers),

            KeyCode::L => Self::letter('l', 'L', modifiers),

            // Ş
            KeyCode::Oem1 => Self::letter('ş', 'Ş', modifiers),

            // İ
            KeyCode::Oem3 => Self::letter('i', 'İ', modifiers),

            // ------------------------------------------------------------
            // SHIFT
            // ------------------------------------------------------------
            KeyCode::LShift => Self::raw(KeyCode::LShift),
            KeyCode::RShift => Self::raw(KeyCode::RShift),

            // ------------------------------------------------------------
            // ZXCV ROW
            // ------------------------------------------------------------
            KeyCode::Z => Self::letter('z', 'Z', modifiers),

            KeyCode::X => Self::letter('x', 'X', modifiers),

            KeyCode::C => Self::letter('c', 'C', modifiers),

            KeyCode::V => Self::letter('v', 'V', modifiers),

            KeyCode::B => Self::letter('b', 'B', modifiers),

            KeyCode::N => Self::letter('n', 'N', modifiers),

            KeyCode::M => Self::letter('m', 'M', modifiers),

            // Ö
            KeyCode::OemComma => Self::letter('ö', 'Ö', modifiers),

            // Ç
            KeyCode::OemPeriod => Self::letter('ç', 'Ç', modifiers),

            // /
            KeyCode::Oem2 => Self::shifted_or('.', ':', modifiers),

            // ------------------------------------------------------------
            // SPACE
            // ------------------------------------------------------------
            KeyCode::Spacebar => DecodedKey::Unicode(' '),

            // ------------------------------------------------------------
            // MODIFIERS
            // ------------------------------------------------------------
            KeyCode::LControl => Self::raw(KeyCode::LControl),
            KeyCode::RControl => Self::raw(KeyCode::RControl),

            KeyCode::LAlt => Self::raw(KeyCode::LAlt),
            KeyCode::RAltGr => Self::raw(KeyCode::RAltGr),

            KeyCode::LWin => Self::raw(KeyCode::LWin),
            KeyCode::RWin => Self::raw(KeyCode::RWin),

            KeyCode::Apps => Self::raw(KeyCode::Apps),

            // ------------------------------------------------------------
            // NAVIGATION
            // ------------------------------------------------------------
            KeyCode::Insert => Self::raw(KeyCode::Insert),
            KeyCode::Delete => Self::raw(KeyCode::Delete),

            KeyCode::Home => Self::raw(KeyCode::Home),
            KeyCode::End => Self::raw(KeyCode::End),

            KeyCode::PageUp => Self::raw(KeyCode::PageUp),
            KeyCode::PageDown => Self::raw(KeyCode::PageDown),

            KeyCode::ArrowUp => Self::raw(KeyCode::ArrowUp),
            KeyCode::ArrowDown => Self::raw(KeyCode::ArrowDown),
            KeyCode::ArrowLeft => Self::raw(KeyCode::ArrowLeft),
            KeyCode::ArrowRight => Self::raw(KeyCode::ArrowRight),

            // ------------------------------------------------------------
            // NUMPAD
            // ------------------------------------------------------------
            KeyCode::NumpadLock => Self::raw(KeyCode::NumpadLock),

            KeyCode::Numpad0 => {
                if modifiers.numlock {
                    DecodedKey::Unicode('0')
                } else {
                    Self::raw(KeyCode::Numpad0)
                }
            }

            KeyCode::Numpad1 => {
                if modifiers.numlock {
                    DecodedKey::Unicode('1')
                } else {
                    Self::raw(KeyCode::Numpad1)
                }
            }

            KeyCode::Numpad2 => {
                if modifiers.numlock {
                    DecodedKey::Unicode('2')
                } else {
                    Self::raw(KeyCode::Numpad2)
                }
            }

            KeyCode::Numpad3 => {
                if modifiers.numlock {
                    DecodedKey::Unicode('3')
                } else {
                    Self::raw(KeyCode::Numpad3)
                }
            }

            KeyCode::Numpad4 => {
                if modifiers.numlock {
                    DecodedKey::Unicode('4')
                } else {
                    Self::raw(KeyCode::Numpad4)
                }
            }

            KeyCode::Numpad5 => {
                if modifiers.numlock {
                    DecodedKey::Unicode('5')
                } else {
                    Self::raw(KeyCode::Numpad5)
                }
            }

            KeyCode::Numpad6 => {
                if modifiers.numlock {
                    DecodedKey::Unicode('6')
                } else {
                    Self::raw(KeyCode::Numpad6)
                }
            }

            KeyCode::Numpad7 => {
                if modifiers.numlock {
                    DecodedKey::Unicode('7')
                } else {
                    Self::raw(KeyCode::Numpad7)
                }
            }

            KeyCode::Numpad8 => {
                if modifiers.numlock {
                    DecodedKey::Unicode('8')
                } else {
                    Self::raw(KeyCode::Numpad8)
                }
            }

            KeyCode::Numpad9 => {
                if modifiers.numlock {
                    DecodedKey::Unicode('9')
                } else {
                    Self::raw(KeyCode::Numpad9)
                }
            }

            KeyCode::NumpadPeriod => {
                if modifiers.numlock {
                    DecodedKey::Unicode('.')
                } else {
                    Self::raw(KeyCode::NumpadPeriod)
                }
            }

            KeyCode::NumpadAdd => DecodedKey::Unicode('+'),

            KeyCode::NumpadSubtract => DecodedKey::Unicode('-'),

            KeyCode::NumpadMultiply => DecodedKey::Unicode('*'),

            KeyCode::NumpadDivide => DecodedKey::Unicode('/'),

            KeyCode::NumpadEnter => DecodedKey::Unicode('\n'),

            // ------------------------------------------------------------
            // OEM / UNKNOWN
            // ------------------------------------------------------------
            KeyCode::Oem5 => Self::shifted_or('<', '>', modifiers),

            KeyCode::Oem7 => Self::shifted_or(',', ';', modifiers),

            KeyCode::Oem8 => Self::shifted_or('"', 'é', modifiers),

            // Multimedia keys
            KeyCode::PrevTrack => Self::raw(KeyCode::PrevTrack),
            KeyCode::NextTrack => Self::raw(KeyCode::NextTrack),
            KeyCode::Mute => Self::raw(KeyCode::Mute),
            KeyCode::Calculator => Self::raw(KeyCode::Calculator),
            KeyCode::Play => Self::raw(KeyCode::Play),
            KeyCode::Stop => Self::raw(KeyCode::Stop),
            KeyCode::VolumeDown => Self::raw(KeyCode::VolumeDown),
            KeyCode::VolumeUp => Self::raw(KeyCode::VolumeUp),
            KeyCode::WWWHome => Self::raw(KeyCode::WWWHome),

            KeyCode::SysRq => Self::raw(KeyCode::SysRq),
            KeyCode::RControl2 => Self::raw(KeyCode::RControl2),
            KeyCode::RAlt2 => Self::raw(KeyCode::RAlt2),

            KeyCode::PowerOnTestOk => Self::raw(KeyCode::PowerOnTestOk),

            KeyCode::TooManyKeys => Self::raw(KeyCode::TooManyKeys),

            KeyCode::Oem9 => Self::raw(KeyCode::Oem9),
            KeyCode::Oem10 => Self::raw(KeyCode::Oem10),
            KeyCode::Oem11 => Self::raw(KeyCode::Oem11),
            KeyCode::Oem12 => Self::raw(KeyCode::Oem12),
            KeyCode::Oem13 => Self::raw(KeyCode::Oem13),
        }
    }
}

fn control_character(keycode: KeyCode) -> Option<char> {
    match keycode {
        KeyCode::A => Some('\u{0001}'),
        KeyCode::B => Some('\u{0002}'),
        KeyCode::C => Some('\u{0003}'),
        KeyCode::D => Some('\u{0004}'),
        KeyCode::E => Some('\u{0005}'),
        KeyCode::F => Some('\u{0006}'),
        KeyCode::G => Some('\u{0007}'),
        KeyCode::H => Some('\u{0008}'),
        KeyCode::I => Some('\u{0009}'),
        KeyCode::J => Some('\u{000A}'),
        KeyCode::K => Some('\u{000B}'),
        KeyCode::L => Some('\u{000C}'),
        KeyCode::M => Some('\u{000D}'),
        KeyCode::N => Some('\u{000E}'),
        KeyCode::O => Some('\u{000F}'),
        KeyCode::P => Some('\u{0010}'),
        KeyCode::Q => Some('\u{0011}'),
        KeyCode::R => Some('\u{0012}'),
        KeyCode::S => Some('\u{0013}'),
        KeyCode::T => Some('\u{0014}'),
        KeyCode::U => Some('\u{0015}'),
        KeyCode::V => Some('\u{0016}'),
        KeyCode::W => Some('\u{0017}'),
        KeyCode::X => Some('\u{0018}'),
        KeyCode::Y => Some('\u{0019}'),
        KeyCode::Z => Some('\u{001A}'),
        _ => None,
    }
}

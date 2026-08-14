#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyCode {
    Escape,

    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,

    StarQuestion,
    Minus,

    Backspace,
    Tab,

    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,

    LeftBracket,
    RightBracket,

    Enter,

    LeftCtrl,

    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,

    Semicolon,
    Apostrophe,
    Grave,

    LeftShift,

    Oem102,

    Z,
    X,
    C,
    V,
    B,
    N,
    M,

    Comma,
    Oem2,
    Oem5,
    Period,

    RightShift,

    NumPadMultiply,
    LeftAlt,

    Space,

    CapsLock,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,

    NumLock,
    ScrollLock,

    NumPad7,
    NumPad8,
    NumPad9,
    NumPadSubtract,

    NumPad4,
    NumPad5,
    NumPad6,
    NumPadAdd,

    NumPad1,
    NumPad2,
    NumPad3,

    NumPad0,
    NumPadDecimal,

    F11,
    F12,

    RightCtrl,
    NumPadDivide,
    NumPadEnter,

    Home,
    ArrowUp,
    PageUp,

    ArrowLeft,
    ArrowRight,

    End,
    ArrowDown,
    PageDown,

    Insert,
    Delete,

    RightAlt,

    LeftSuper,
    RightSuper,
    Menu,

    PrintScreen,
    Pause,
}

pub fn from_scancode(
    scancode: u8,
    extended: bool,
) -> Option<KeyCode> {
    if extended {
        return from_extended_scancode(scancode);
    }

    match scancode & 0x7F {
        // ----------------------------------------------------
        // ESC
        // ----------------------------------------------------

        0x01 => Some(KeyCode::Escape),

        // ----------------------------------------------------
        // NUMBER ROW
        // ----------------------------------------------------

        0x02 => Some(KeyCode::Num1),
        0x03 => Some(KeyCode::Num2),
        0x04 => Some(KeyCode::Num3),
        0x05 => Some(KeyCode::Num4),
        0x06 => Some(KeyCode::Num5),
        0x07 => Some(KeyCode::Num6),
        0x08 => Some(KeyCode::Num7),
        0x09 => Some(KeyCode::Num8),
        0x0A => Some(KeyCode::Num9),
        0x0B => Some(KeyCode::Num0),
        0x0C => Some(KeyCode::StarQuestion),
        0x0D => Some(KeyCode::Minus),

        0x0E => Some(KeyCode::Backspace),
        0x0F => Some(KeyCode::Tab),

        // ----------------------------------------------------
        // Q ROW
        // ----------------------------------------------------

        0x10 => Some(KeyCode::Q),
        0x11 => Some(KeyCode::W),
        0x12 => Some(KeyCode::E),
        0x13 => Some(KeyCode::R),
        0x14 => Some(KeyCode::T),
        0x15 => Some(KeyCode::Y),
        0x16 => Some(KeyCode::U),
        0x17 => Some(KeyCode::I),
        0x18 => Some(KeyCode::O),
        0x19 => Some(KeyCode::P),

        0x1A => Some(KeyCode::LeftBracket),
        0x1B => Some(KeyCode::RightBracket),

        0x1C => Some(KeyCode::Enter),
        0x1D => Some(KeyCode::LeftCtrl),

        // ----------------------------------------------------
        // HOME ROW
        // ----------------------------------------------------

        0x1E => Some(KeyCode::A),
        0x1F => Some(KeyCode::S),
        0x20 => Some(KeyCode::D),
        0x21 => Some(KeyCode::F),
        0x22 => Some(KeyCode::G),
        0x23 => Some(KeyCode::H),
        0x24 => Some(KeyCode::J),
        0x25 => Some(KeyCode::K),
        0x26 => Some(KeyCode::L),

        0x27 => Some(KeyCode::Semicolon),
        0x28 => Some(KeyCode::Apostrophe),
        0x29 => Some(KeyCode::Grave),

        0x2A => Some(KeyCode::LeftShift),

        // ----------------------------------------------------
        // ISO EXTRA KEY
        //
        // Physical < > | key.
        // ----------------------------------------------------

        0x56 => Some(KeyCode::Oem102),

        // ----------------------------------------------------
        // BOTTOM ROW
        // ----------------------------------------------------

        0x2C => Some(KeyCode::Z),
        0x2D => Some(KeyCode::X),
        0x2E => Some(KeyCode::C),
        0x2F => Some(KeyCode::V),
        0x30 => Some(KeyCode::B),
        0x31 => Some(KeyCode::N),
        0x32 => Some(KeyCode::M),

        0x33 => Some(KeyCode::Comma),
        0x34 => Some(KeyCode::Oem2),
        0x35 => Some(KeyCode::Oem5),

        0x36 => Some(KeyCode::RightShift),

        // ----------------------------------------------------
        // SYSTEM / NUMPAD
        // ----------------------------------------------------

        0x37 => Some(KeyCode::NumPadMultiply),
        0x38 => Some(KeyCode::LeftAlt),
        0x39 => Some(KeyCode::Space),
        0x3A => Some(KeyCode::CapsLock),

        0x3B => Some(KeyCode::F1),
        0x3C => Some(KeyCode::F2),
        0x3D => Some(KeyCode::F3),
        0x3E => Some(KeyCode::F4),
        0x3F => Some(KeyCode::F5),
        0x40 => Some(KeyCode::F6),
        0x41 => Some(KeyCode::F7),
        0x42 => Some(KeyCode::F8),
        0x43 => Some(KeyCode::F9),
        0x44 => Some(KeyCode::F10),

        0x45 => Some(KeyCode::NumLock),
        0x46 => Some(KeyCode::ScrollLock),

        0x47 => Some(KeyCode::NumPad7),
        0x48 => Some(KeyCode::NumPad8),
        0x49 => Some(KeyCode::NumPad9),
        0x4A => Some(KeyCode::NumPadSubtract),

        0x4B => Some(KeyCode::NumPad4),
        0x4C => Some(KeyCode::NumPad5),
        0x4D => Some(KeyCode::NumPad6),
        0x4E => Some(KeyCode::NumPadAdd),

        0x4F => Some(KeyCode::NumPad1),
        0x50 => Some(KeyCode::NumPad2),
        0x51 => Some(KeyCode::NumPad3),

        0x52 => Some(KeyCode::NumPad0),
        0x53 => Some(KeyCode::NumPadDecimal),

        0x57 => Some(KeyCode::F11),
        0x58 => Some(KeyCode::F12),

        _ => None,
    }
}

fn from_extended_scancode(
    scancode: u8,
) -> Option<KeyCode> {
    match scancode & 0x7F {
        // ----------------------------------------------------
        // E0 + ...
        // ----------------------------------------------------

        0x1C => Some(KeyCode::NumPadEnter),
        0x1D => Some(KeyCode::RightCtrl),
        0x35 => Some(KeyCode::NumPadDivide),

        0x37 => Some(KeyCode::PrintScreen),

        0x38 => Some(KeyCode::RightAlt),

        0x47 => Some(KeyCode::Home),
        0x48 => Some(KeyCode::ArrowUp),
        0x49 => Some(KeyCode::PageUp),

        0x4B => Some(KeyCode::ArrowLeft),
        0x4D => Some(KeyCode::ArrowRight),

        0x4F => Some(KeyCode::End),
        0x50 => Some(KeyCode::ArrowDown),
        0x51 => Some(KeyCode::PageDown),

        0x52 => Some(KeyCode::Insert),
        0x53 => Some(KeyCode::Delete),

        0x5B => Some(KeyCode::LeftSuper),
        0x5C => Some(KeyCode::RightSuper),
        0x5D => Some(KeyCode::Menu),

        _ => None,
    }
}
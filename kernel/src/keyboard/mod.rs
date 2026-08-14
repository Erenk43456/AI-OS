pub mod event;
pub mod keycode;
pub mod layout;
pub mod state;

use crate::drivers::keyboard::ps2::{
    controller::Controller,
    keyboard::Ps2Keyboard,
};

use crate::input::{
    event::InputEvent,
    queue,
};

use spin::Mutex;

use event::{
    KeyEvent,
    KeyEventKind,
};

use keycode::KeyCode;

use layout::Modifiers;

use state::KeyboardState;

// ============================================================
// DRIVER STATE
// ============================================================

static PS2_KEYBOARD: Mutex<Option<Ps2Keyboard>> =
    Mutex::new(None);

static KEYBOARD_STATE: Mutex<KeyboardState> =
    Mutex::new(KeyboardState::new());

static EXTENDED_SEQUENCE: Mutex<bool> =
    Mutex::new(false);

// ============================================================
// INITIALIZATION
// ============================================================

pub fn init() {
    let controller =
        Controller::new();

    let keyboard =
        Ps2Keyboard::new(controller);

    if keyboard.init().is_err() {
        return;
    }

    *PS2_KEYBOARD.lock() =
        Some(keyboard);
}

// ============================================================
// INTERRUPT ENTRY
// ============================================================

pub fn handle_interrupt() {
    let keyboard =
        PS2_KEYBOARD.lock();

    let Some(keyboard) =
        keyboard.as_ref()
    else {
        return;
    };

    let Ok(Some(scancode)) =
        keyboard.read_scancode()
    else {
        return;
    };

    process_scancode(scancode);
}

// ============================================================
// SCANCODE PROCESSING
// ============================================================

fn process_scancode(
    scancode: u8,
) {
    // --------------------------------------------------------
    // E0 EXTENDED PREFIX
    // --------------------------------------------------------

    if scancode == 0xE0 {
        *EXTENDED_SEQUENCE.lock() =
            true;

        return;
    }

    // --------------------------------------------------------
    // E1 PREFIX
    //
    // Pause/Break is a special multi-byte sequence.
    // We currently consume it safely instead of allowing
    // the individual bytes to become normal keys.
    // --------------------------------------------------------

    if scancode == 0xE1 {
        return;
    }

    let extended = {
        let mut state =
            EXTENDED_SEQUENCE.lock();

        let value = *state;

        *state = false;

        value
    };

    let pressed =
        scancode & 0x80 == 0;

    let Some(key) =
        keycode::from_scancode(
            scancode,
            extended,
        )
    else {
        return;
    };

    let event =
        if pressed {
            KeyEvent::press(key)
        } else {
            KeyEvent::release(key)
        };

    process_key_event(event);
}

// ============================================================
// KEY EVENT PROCESSING
// ============================================================

fn process_key_event(
    event: KeyEvent,
) {
    let pressed =
        matches!(
            event.kind,
            KeyEventKind::Press
        );

    {
        let mut state =
            KEYBOARD_STATE.lock();

        state.update(
            event.key,
            pressed,
        );
    }

    // Modifier and lock keys do not generate
    // character events themselves.
    if is_modifier_or_lock(event.key) {
        return;
    }

    if !pressed {
        return;
    }

    let state =
        *KEYBOARD_STATE.lock();

    let modifiers =
        Modifiers {
            shift: state.shift(),
            ctrl: state.ctrl(),
            alt: state.alt(),
            alt_gr: state.alt_gr(),
        };

    // --------------------------------------------------------
    // CHARACTER
    // --------------------------------------------------------

    if let Some(character) =
        layout::translate_keycode(
            event.key,
            modifiers,
        )
    {
        let character =
            layout::apply_caps_lock(
                character,
                state.caps_lock(),
                modifiers.shift,
            );
            
        queue::push(
            InputEvent::KeyPress(
                character,
            ),
        );

        return;
    }

    // --------------------------------------------------------
    // NON-CHARACTER INPUT
    // --------------------------------------------------------

    match event.key {
        KeyCode::Backspace => {
            queue::push(
                InputEvent::Backspace,
            );
        }

        KeyCode::Enter => {
            queue::push(
                InputEvent::Enter,
            );
        }

        KeyCode::Tab => {
            queue::push(
                InputEvent::Tab,
            );
        }

        KeyCode::ArrowUp => {
            queue::push(
                InputEvent::ArrowUp,
            );
        }

        KeyCode::ArrowDown => {
            queue::push(
                InputEvent::ArrowDown,
            );
        }

        KeyCode::ArrowLeft => {
            queue::push(
                InputEvent::ArrowLeft,
            );
        }

        KeyCode::ArrowRight => {
            queue::push(
                InputEvent::ArrowRight,
            );
        }

        _ => {}
    }
}

// ============================================================
// MODIFIER / LOCK CLASSIFICATION
// ============================================================

fn is_modifier_or_lock(
    key: KeyCode,
) -> bool {
    matches!(
        key,

        KeyCode::LeftShift
        | KeyCode::RightShift

        | KeyCode::LeftCtrl
        | KeyCode::RightCtrl

        | KeyCode::LeftAlt
        | KeyCode::RightAlt

        | KeyCode::LeftSuper
        | KeyCode::RightSuper

        | KeyCode::CapsLock
        | KeyCode::NumLock
        | KeyCode::ScrollLock
    )
}

// ============================================================
// PUBLIC STATE ACCESS
// ============================================================

pub fn state() -> KeyboardState {
    *KEYBOARD_STATE.lock()
}
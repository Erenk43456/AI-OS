#[derive(Clone, Copy, Debug)]
pub enum InputEvent {
    KeyPress(u8),

    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,

    Backspace,
    Enter,
    Tab,
}
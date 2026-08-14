#[derive(Clone, Copy, Debug)]
pub enum InputEvent {
    KeyPress(char),

    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,

    Backspace,
    Enter,
    Tab,
}
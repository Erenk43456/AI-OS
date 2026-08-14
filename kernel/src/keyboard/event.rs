use super::keycode::KeyCode;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyEventKind {
    Press,
    Release,
}

#[derive(Clone, Copy, Debug)]
pub struct KeyEvent {
    pub key: KeyCode,
    pub kind: KeyEventKind,
}

impl KeyEvent {
    pub const fn press(
        key: KeyCode,
    ) -> Self {
        Self {
            key,
            kind: KeyEventKind::Press,
        }
    }

    pub const fn release(
        key: KeyCode,
    ) -> Self {
        Self {
            key,
            kind: KeyEventKind::Release,
        }
    }

    pub fn pressed(&self) -> bool {
        matches!(
            self.kind,
            KeyEventKind::Press
        )
    }

    pub fn released(&self) -> bool {
        matches!(
            self.kind,
            KeyEventKind::Release
        )
    }
}
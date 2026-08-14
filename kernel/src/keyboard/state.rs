use super::keycode::KeyCode;

#[derive(Clone, Copy, Debug)]
pub struct KeyboardState {
    left_shift: bool,
    right_shift: bool,

    left_ctrl: bool,
    right_ctrl: bool,

    left_alt: bool,
    right_alt: bool,

    caps_lock: bool,
    num_lock: bool,
    scroll_lock: bool,
}

impl KeyboardState {
    pub const fn new() -> Self {
        Self {
            left_shift: false,
            right_shift: false,

            left_ctrl: false,
            right_ctrl: false,

            left_alt: false,
            right_alt: false,

            caps_lock: false,
            num_lock: false,
            scroll_lock: false,
        }
    }

    pub fn update(
        &mut self,
        key: KeyCode,
        pressed: bool,
    ) {
        match key {
            KeyCode::LeftShift => {
                self.left_shift = pressed;
            }

            KeyCode::RightShift => {
                self.right_shift = pressed;
            }

            KeyCode::LeftCtrl => {
                self.left_ctrl = pressed;
            }

            KeyCode::RightCtrl => {
                self.right_ctrl = pressed;
            }

            KeyCode::LeftAlt => {
                self.left_alt = pressed;
            }

            KeyCode::RightAlt => {
                self.right_alt = pressed;
            }

            KeyCode::CapsLock if pressed => {
                self.caps_lock = !self.caps_lock;
            }

            KeyCode::NumLock if pressed => {
                self.num_lock = !self.num_lock;
            }

            KeyCode::ScrollLock if pressed => {
                self.scroll_lock = !self.scroll_lock;
            }

            _ => {}
        }
    }

    pub fn shift(&self) -> bool {
        self.left_shift || self.right_shift
    }

    pub fn ctrl(&self) -> bool {
        self.left_ctrl || self.right_ctrl
    }

    pub fn alt(&self) -> bool {
        self.left_alt || self.right_alt
    }

    pub fn alt_gr(&self) -> bool {
        self.right_alt
    }

    pub fn caps_lock(&self) -> bool {
        self.caps_lock
    }

    pub fn num_lock(&self) -> bool {
        self.num_lock
    }

    pub fn scroll_lock(&self) -> bool {
        self.scroll_lock
    }
}
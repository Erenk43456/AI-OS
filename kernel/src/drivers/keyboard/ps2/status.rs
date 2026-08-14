#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Status {
    value: u8,
}

impl Status {
    pub const OUTPUT_BUFFER_FULL: u8 = 1 << 0;
    pub const INPUT_BUFFER_FULL: u8 = 1 << 1;
    pub const SYSTEM_FLAG: u8 = 1 << 2;
    pub const COMMAND_DATA: u8 = 1 << 3;
    pub const KEYBOARD_LOCK: u8 = 1 << 4;
    pub const AUX_OUTPUT_BUFFER_FULL: u8 = 1 << 5;
    pub const TIMEOUT_ERROR: u8 = 1 << 6;
    pub const PARITY_ERROR: u8 = 1 << 7;

    pub const fn new(value: u8) -> Self {
        Self { value }
    }

    pub const fn raw(self) -> u8 {
        self.value
    }

    pub const fn output_buffer_full(self) -> bool {
        self.value & Self::OUTPUT_BUFFER_FULL != 0
    }

    pub const fn input_buffer_full(self) -> bool {
        self.value & Self::INPUT_BUFFER_FULL != 0
    }

    pub const fn system_flag(self) -> bool {
        self.value & Self::SYSTEM_FLAG != 0
    }

    pub const fn command_data(self) -> bool {
        self.value & Self::COMMAND_DATA != 0
    }

    pub const fn keyboard_lock(self) -> bool {
        self.value & Self::KEYBOARD_LOCK != 0
    }

    pub const fn aux_output_buffer_full(self) -> bool {
        self.value & Self::AUX_OUTPUT_BUFFER_FULL != 0
    }

    pub const fn timeout_error(self) -> bool {
        self.value & Self::TIMEOUT_ERROR != 0
    }

    pub const fn parity_error(self) -> bool {
        self.value & Self::PARITY_ERROR != 0
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KeyboardResponse {
    Ack = 0xFA,
    Resend = 0xFE,
    BatOk = 0xAA,
    BatError = 0xFC,
    Echo = 0xEE,
}

impl KeyboardResponse {
    pub const fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0xFA => Some(Self::Ack),
            0xFE => Some(Self::Resend),
            0xAA => Some(Self::BatOk),
            0xFC => Some(Self::BatError),
            0xEE => Some(Self::Echo),
            _ => None,
        }
    }

    pub const fn as_u8(self) -> u8 {
        self as u8
    }
}
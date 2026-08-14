#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ControllerCommand {
    ReadConfiguration = 0x20,
    WriteConfiguration = 0x60,

    DisableFirstPort = 0xAD,
    EnableFirstPort = 0xAE,

    DisableSecondPort = 0xA7,
    EnableSecondPort = 0xA8,

    TestController = 0xAA,
    TestFirstPort = 0xAB,
    TestSecondPort = 0xA9,

    WriteFirstPortOutput = 0xD2,
    WriteSecondPortOutput = 0xD3,

    WriteFirstPortInput = 0xD4,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KeyboardCommand {
    SetLeds = 0xED,
    Echo = 0xEE,
    SetScancode = 0xF0,
    Identify = 0xF2,
    EnableScanning = 0xF4,
    DisableScanning = 0xF5,
    SetDefaults = 0xF6,
    Resend = 0xFE,
    Reset = 0xFF,
}

impl ControllerCommand {
    pub const fn as_u8(self) -> u8 {
        self as u8
    }
}

impl KeyboardCommand {
    pub const fn as_u8(self) -> u8 {
        self as u8
    }
}
use super::command::{ControllerCommand, KeyboardCommand};
use super::controller::{Controller, ControllerError};
use super::response::KeyboardResponse;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyboardError {
    Controller(ControllerError),
    UnexpectedResponse(u8),
    SelfTestFailed(u8),
    KeyboardTestFailed(u8),
    ResetFailed,
    EnableScanningFailed,
}

impl From<ControllerError> for KeyboardError {
    fn from(error: ControllerError) -> Self {
        Self::Controller(error)
    }
}

pub struct Ps2Keyboard {
    controller: Controller,
}

impl Ps2Keyboard {
    pub const fn new(controller: Controller) -> Self {
        Self { controller }
    }

    pub fn init(&self) -> Result<(), KeyboardError> {
        self.disable_keyboard()?;
        self.flush_output_buffer();

        self.test_controller()?;
        self.reset_keyboard()?;
        self.enable_scanning()?;

        Ok(())
    }

    pub fn read_scancode(&self) -> Result<Option<u8>, KeyboardError> {
        if !self.controller.data_available() {
            return Ok(None);
        }

        Ok(Some(self.controller.read_data()?))
    }

    fn disable_keyboard(&self) -> Result<(), KeyboardError> {
        self.controller
            .write_command(ControllerCommand::DisableFirstPort)?;

        Ok(())
    }

    fn test_controller(&self) -> Result<(), KeyboardError> {
        self.controller
            .write_command(ControllerCommand::TestController)?;

        let response = self.controller.read_data()?;

        if response != 0x55 {
            return Err(KeyboardError::SelfTestFailed(response));
        }

        Ok(())
    }

    fn reset_keyboard(&self) -> Result<(), KeyboardError> {
        self.controller.write_data(KeyboardCommand::Reset.as_u8())?;

        let response = self.controller.read_data()?;

        if response != KeyboardResponse::Ack.as_u8() {
            return Err(KeyboardError::ResetFailed);
        }

        let bat = self.controller.read_data()?;

        match bat {
            0xAA => Ok(()),
            value => Err(KeyboardError::KeyboardTestFailed(value)),
        }
    }

    fn enable_scanning(&self) -> Result<(), KeyboardError> {
        self.controller
            .write_data(KeyboardCommand::EnableScanning.as_u8())?;

        let response = self.controller.read_data()?;

        if response != KeyboardResponse::Ack.as_u8() {
            return Err(KeyboardError::EnableScanningFailed);
        }

        Ok(())
    }

    fn flush_output_buffer(&self) {
        while self.controller.data_available() {
            if self.controller.read_data().is_err() {
                break;
            }
        }
    }
}
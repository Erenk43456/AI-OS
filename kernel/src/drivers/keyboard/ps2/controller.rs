use crate::arch::x86_64::io;
use super::command::ControllerCommand;
use super::status::Status;

const DATA_PORT: u16 = 0x60;
const STATUS_PORT: u16 = 0x64;

const TIMEOUT_ITERATIONS: u32 = 100_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ControllerError {
    Timeout,
    OutputTimeout,
    InputTimeout,
    ParityError,
    TimeoutError,
}

pub struct Controller;

impl Controller {
    pub const fn new() -> Self {
        Self
    }

    #[inline]
    pub fn status(&self) -> Status {
        let value = unsafe { io::inb(STATUS_PORT) };
        Status::new(value)
    }

    #[inline]
    pub fn data_available(&self) -> bool {
        self.status().output_buffer_full()
    }

    pub fn wait_input_clear(&self) -> Result<(), ControllerError> {
        for _ in 0..TIMEOUT_ITERATIONS {
            if !self.status().input_buffer_full() {
                return Ok(());
            }
        }

        Err(ControllerError::InputTimeout)
    }

    pub fn wait_output_full(&self) -> Result<(), ControllerError> {
        for _ in 0..TIMEOUT_ITERATIONS {
            if self.status().output_buffer_full() {
                return Ok(());
            }
        }

        Err(ControllerError::OutputTimeout)
    }

    pub fn read_data(&self) -> Result<u8, ControllerError> {
        self.wait_output_full()?;

        let status = self.status();

        if status.timeout_error() {
            return Err(ControllerError::TimeoutError);
        }

        if status.parity_error() {
            return Err(ControllerError::ParityError);
        }

        Ok(unsafe { io::inb(DATA_PORT) })
    }

    pub fn write_data(&self, value: u8) -> Result<(), ControllerError> {
        self.wait_input_clear()?;

        unsafe {
            io::outb(DATA_PORT, value);
        }

        Ok(())
    }

    pub fn write_command(
        &self,
        command: ControllerCommand,
    ) -> Result<(), ControllerError> {
        self.wait_input_clear()?;

        unsafe {
            io::outb(STATUS_PORT, command.as_u8());
        }

        Ok(())
    }
}
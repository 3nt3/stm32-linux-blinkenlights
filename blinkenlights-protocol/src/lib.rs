#![no_std]

use core::convert::{From, Into};
use defmt::Format;

#[derive(Format, Copy, Clone)]
pub enum Level {
    High,
    Low,
}

impl From<bool> for Level {
    fn from(value: bool) -> Self {
        if value {
            Level::High
        } else {
            Level::Low
        }
    }
}

impl From<Level> for bool {
    fn from(level: Level) -> Self {
        match level {
            Level::High => true,
            Level::Low => false,
        }
    }
}

#[derive(Format, Copy, Clone)]
pub enum Command {
    SetLED(u8, Level),
    SetAll(Level),
    Unknown,
}

#[derive(Format, Copy, Clone)]
pub enum Response {
    Ok,
    Error(ErrorCode, &'static str),
}

#[derive(Format, Copy, Clone)]
pub enum ErrorCode {
    InvalidCommand,
    InvalidParameter,
    InternalError,
}

impl Command {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        if bytes.is_empty() {
            return Command::Unknown;
        }

        match bytes[0] {
            0x01 => Command::SetLED(bytes[1], (bytes[2] != 0).into()),
            0x02 => Command::SetAll((bytes[1] != 0).into()),
            _ => Command::Unknown,
        }
    }

    pub fn to_bytes(&self) -> [u8; 64] {
        let mut bytes = [0x00; 64];
        match self {
            Command::SetLED(led, level) => {
                bytes[0] = 0x01; // Command ID for SetLED
                bytes[1] = *led;
                bytes[2] = if (*level).into() { 1 } else { 0 };
            }
            Command::SetAll(level) => {
                bytes[0] = 0x02; // Command ID for SetAll
                bytes[1] = if (*level).into() { 1 } else { 0 };
            }
            Command::Unknown => {}
        }
        bytes
    }
}

impl Response {
    pub fn to_bytes(&self) -> [u8; 64] {
        match self {
            Response::Ok => [0x00; 64],
            Response::Error(error_code, msg) => {
                let mut response = [0x00; 64];
                let bytes = msg.as_bytes();
                let len = bytes.len().min(63);
                response[0] = 0xFF; // Error indicator
                response[1] = match error_code {
                    ErrorCode::InvalidCommand => 0x01,
                    ErrorCode::InvalidParameter => 0x02,
                    ErrorCode::InternalError => 0x03,
                };
                response[2..=len].copy_from_slice(&bytes[..len]);
                response
            }
        }
    }
}

#![no_std]

use core::convert::{From, Into};

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

pub enum Command {
    SetLED(u8, Level),
    SetAll(Level),
    Unknown,
}

pub enum Response {
    Ok,
    Error(ErrorCode, &'static str),
}

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
            0x01 => {
                if bytes.len() == 2 {
                    Command::SetLED(bytes[1], (bytes[1] != 0).into())
                } else {
                    Command::Unknown
                }
            }
            0x02 => {
                if bytes.len() == 2 {
                    Command::SetAll((bytes[1] != 0).into())
                } else {
                    Command::Unknown
                }
            }
            _ => Command::Unknown,
        }
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

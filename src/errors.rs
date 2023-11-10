use std::fmt;

#[derive(Debug, Clone)]
pub enum LMCError {
    ProgramTooLarge(String),
    InvalidOpcode(String),
    NumberError(String),
    IOError(String),
}

impl fmt::Display for LMCError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LMCError::ProgramTooLarge(msg) => write!(f, "Program too large: {msg}"),
            LMCError::InvalidOpcode(msg) => write!(f, "Invalid opcode: got {msg}"),
            LMCError::NumberError(msg) => write!(f, "Number error: {msg}"),
            LMCError::IOError(msg) => write!(f, "I/O error: {msg}"),
        }
    }
}

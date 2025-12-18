use std::fmt;
use std::{num::ParseFloatError, sync::atomic::AtomicBool};

pub static HAD_ERROR: AtomicBool = AtomicBool::new(false);

// Scanner Error
#[derive(Debug)]
pub enum ScannerError {
    ParseFloat(ParseFloatError),
    UnterminatedString { line: usize },
    UnexpectedCharacter { line: usize, character: char },
}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScannerError::ParseFloat(e) => write!(f, "ParseFloatError: {}", e),
            ScannerError::UnterminatedString { line } => {
                write!(f, "[line {}] Unterminated String.", line)
            }
            ScannerError::UnexpectedCharacter { line, character } => {
                write!(f, "[line{}] Unexpected character: '{}'", line, character)
            }
        }
    }
}

impl std::error::Error for ScannerError {}

impl From<ParseFloatError> for ScannerError {
    fn from(error: ParseFloatError) -> Self {
        ScannerError::ParseFloat(error)
    }
}

#[derive(Debug)]
pub enum RloxError {
    Scanner(ScannerError),
}

impl fmt::Display for RloxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RloxError::Scanner(err) => write!(f, "Scanner error: {}", err),
        }
    }
}

impl std::error::Error for RloxError {}

impl From<ScannerError> for RloxError {
    fn from(err: ScannerError) -> Self {
        RloxError::Scanner(err)
    }
}

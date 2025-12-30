use std::fmt;
use std::sync::atomic::Ordering;
use std::{num::ParseFloatError, sync::atomic::AtomicBool};

use crate::token::Token;

pub static HAD_ERROR: AtomicBool = AtomicBool::new(false);

// Top-Level Error
#[derive(Debug)]
pub enum RloxError {
    Io(std::io::Error),
    Scanner(ScannerError),
}

impl fmt::Display for RloxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RloxError::Io(err) => write!(f, "IO error: {}", err),
            RloxError::Scanner(err) => write!(f, "Scanner error: {}", err),
        }
    }
}

impl std::error::Error for RloxError {}

impl From<std::io::Error> for RloxError {
    fn from(err: std::io::Error) -> Self {
        RloxError::Io(err)
    }
}

impl From<ScannerError> for RloxError {
    fn from(err: ScannerError) -> Self {
        RloxError::Scanner(err)
    }
}

// Scanner Error
#[derive(Debug)]
pub enum ScannerError {
    ParseFloat(ParseFloatError),
    UnterminatedString { line: usize },
    UnexpectedCharacter { line: usize, character: char },
    UnterminatedBlockComment { line: usize },
}

impl ScannerError {
    pub fn new(err: Self) -> Self {
        HAD_ERROR.store(true, Ordering::Relaxed);
        err
    }
}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScannerError::ParseFloat(e) => {
                write!(f, "ParseFloatError: {}", e)
            }
            ScannerError::UnterminatedString { line } => {
                write!(f, "[line {}] Unterminated string.", line)
            }
            ScannerError::UnexpectedCharacter { line, character } => {
                write!(f, "[line {}] Unexpected character: '{}'", line, character)
            }
            ScannerError::UnterminatedBlockComment { line } => {
                write!(f, "[line {}] Unterminated block comment.", line)
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

// Parser Error
#[derive(Debug)]
pub enum ParserError {
    UnexpectedCharacter(Token),
    UnterminatedGroup(Token),
}

impl ParserError {
    pub fn new(err: Self) -> Self {
        HAD_ERROR.store(true, Ordering::Relaxed);
        err
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::UnexpectedCharacter(e) => {
                write!(f, "[line {}] Unexpected Character at {}.", e.line, e.lexeme)
            }
            ParserError::UnterminatedGroup(e) => {
                write!(
                    f,
                    "[line {}] Unterminated grouping at {}.",
                    e.line, e.lexeme
                )
            }
        }
    }
}

impl std::error::Error for ParserError {}

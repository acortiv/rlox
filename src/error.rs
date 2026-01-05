use std::fmt;
use std::num::ParseFloatError;
use std::rc::Rc;

use crate::token::Token;

pub fn report<E: fmt::Display>(err: &E) {
    eprintln!("{}", err);
}

// Top-Level Error
#[derive(Debug)]
pub enum RloxError {
    Io(std::io::Error),
    Scanner(ScannerError),
    Parser(ParserError),
    Interpreter(RuntimeError),
}

impl fmt::Display for RloxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RloxError::Io(err) => write!(f, "IO error: {}", err),
            RloxError::Scanner(err) => write!(f, "Scanner error: {}", err),
            RloxError::Parser(err) => write!(f, "Parser error: {}", err),
            RloxError::Interpreter(err) => write!(f, "Interpreter error: {}", err),
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
    UnexpectedToken(Rc<Token>),
    UnterminatedGroup(Rc<Token>),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::UnexpectedToken(e) => {
                write!(f, "[line {}] Unexpected Token at {}.", e.line, e.lexeme)
            }
            ParserError::UnterminatedGroup(e) => {
                write!(
                    f,
                    "[line {}] Unterminated Grouping.  Expect ')' after expression.",
                    e.line
                )
            }
        }
    }
}

impl std::error::Error for ParserError {}

// Interpreter Errors
#[derive(Debug)]
pub enum RuntimeError {
    TypeError(Rc<Token>),
    DivByZero(Rc<Token>),
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::TypeError(e) => {
                write!(f, "[line {}] Incorrect type at {}.", e.line, e.lexeme)
            }
            RuntimeError::DivByZero(e) => {
                write!(f, "[line {}] Unable to divide by 0.", e.line)
            }
        }
    }
}

impl std::error::Error for RuntimeError {}

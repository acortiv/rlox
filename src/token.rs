use std::fmt;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two characetr tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    EOF,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Identifier(String),
    Str(String),
    Number(f64),
    Bool(bool),
    Nil,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Identifier(string) => write!(f, "Identifier: {string}"),
            Literal::Str(string) => write!(f, "String: {string}"),
            Literal::Number(num) => write!(f, "Number: {num}"),
            Literal::Bool(bool) => write!(f, "Boolean: {bool}"),
            Literal::Nil => write!(f, "Nil"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    pub line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token Type: {:#?}, Lexeme: {}, Literal: {:#?}, Line: {}",
            self.ttype, self.lexeme, self.literal, self.line
        )
    }
}

impl Token {
    pub fn show(&self) -> String {
        format!(
            "line:{}, ttype{:?}, lexeme:{}, literal:{:?}",
            self.line, self.ttype, self.lexeme, self.literal
        )
    }
}

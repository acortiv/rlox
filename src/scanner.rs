use crate::error::error;
use crate::token::{Literal, Token, TokenType};

// Source needs to be made into U8 as Rust uses UTF-8... indexing strings is unsafe and O(n)
#[derive(Clone, Debug)]
pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
    pub start: usize,
    pub current: usize,
    pub line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source: source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()
        }

        self.tokens.push(Token {
            ttype: TokenType::EOF,
            lexeme: String::new(),
            literal: Literal::Nil,
            line: self.line,
        });
        std::mem::take(&mut self.tokens)
    }

    fn scan_token(&mut self) {
        match self.advance() {
            b'(' => self.add_token(TokenType::LeftParen),
            b')' => self.add_token(TokenType::RightParen),
            b'{' => self.add_token(TokenType::LeftBrace),
            b'}' => self.add_token(TokenType::RightBrace),
            b',' => self.add_token(TokenType::Comma),
            b'.' => self.add_token(TokenType::Dot),
            b'-' => self.add_token(TokenType::Minus),
            b'+' => self.add_token(TokenType::Plus),
            b';' => self.add_token(TokenType::Semicolon),
            b'*' => self.add_token(TokenType::Star),
            b'!' => {
                let t = if self.match_char(b'=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(t)
            }
            b'=' => {
                let t = if self.match_char(b'=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(t)
            }
            b'<' => {
                let t = if self.match_char(b'=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(t)
            }
            b'>' => {
                let t = if self.match_char(b'=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(t)
            }
            b'/' => {
                if self.match_char(b'/') {
                    while self.peek() != b'\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            b' ' | b'\r' | b'\t' => (),
            b'\n' => self.line += 1,
            b'"' => self.parse_string(),
            _ => error(self.line, "Unexpected character."),
        }
    }

    fn parse_string(&mut self) {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1
            };
            self.advance();
        }

        if self.is_at_end() {
            error(self.line, "Unterminated string.");
            return;
        }

        self.advance();
        let start = &self.start + 1;
        let end = &self.current - 1;
        let text = &self.source[start..end];
        self.add_token_prime(TokenType::String, Literal::Str(text.to_string()))
    }

    fn match_char(&mut self, expected: u8) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.as_bytes()[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() {
            return b'\0';
        }
        self.source.as_bytes()[self.current]
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> u8 {
        let char = self.source.as_bytes()[self.current];
        self.current += 1;
        char
    }

    fn add_token(&mut self, t: TokenType) {
        self.add_token_prime(t, Literal::Nil)
    }

    fn add_token_prime(&mut self, t: TokenType, literal: Literal) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token {
            ttype: t,
            lexeme: text.to_string(),
            literal: literal,
            line: self.line,
        });
    }
}

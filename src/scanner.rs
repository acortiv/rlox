use crate::error::ScannerError;
use crate::token::{Literal, Token, TokenType};

type Result<T> = std::result::Result<T, ScannerError>;

fn keyword_capture(ident: &str) -> Option<TokenType> {
    match ident {
        "and" => Some(TokenType::And),
        "class" => Some(TokenType::Class),
        "else" => Some(TokenType::Else),
        "false" => Some(TokenType::False),
        "for" => Some(TokenType::For),
        "fun" => Some(TokenType::Fun),
        "if" => Some(TokenType::If),
        "nil" => Some(TokenType::Nil),
        "or" => Some(TokenType::Or),
        "print" => Some(TokenType::Print),
        "return" => Some(TokenType::Return),
        "super" => Some(TokenType::Super),
        "this" => Some(TokenType::This),
        "true" => Some(TokenType::True),
        "var" => Some(TokenType::Var),
        "while" => Some(TokenType::While),
        _ => None,
    }
}

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

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token {
            ttype: TokenType::EOF,
            lexeme: String::new(),
            literal: Literal::Nil,
            line: self.line,
        });
        Ok(std::mem::take(&mut self.tokens))
    }

    fn scan_token(&mut self) -> Result<()> {
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
                    Ok(())
                } else if self.match_char(b'*') {
                    self.parse_block_comment()
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            b' ' | b'\r' | b'\t' => Ok(()),
            b'\n' => {
                self.line += 1;
                Ok(())
            }
            b'"' => self.parse_string(),
            c if self.is_digit(c) => self.parse_number(),
            c if self.is_alpha(c) => self.parse_identifier(),
            c => Err(ScannerError::new(ScannerError::UnexpectedCharacter {
                line: self.line,
                character: c as char,
            })),
        }
    }

    fn parse_block_comment(&mut self) -> Result<()> {
        let mut depth = 1;

        while depth > 0 {
            if self.is_at_end() {
                return Err(ScannerError::new(ScannerError::UnterminatedBlockComment {
                    line: self.line,
                }));
            }

            let c = self.advance();

            match c {
                b'/' if self.match_char(b'*') => depth += 1,
                b'*' if self.match_char(b'/') => depth -= 1,
                b'\n' => self.line += 1,
                _ => {}
            }
        }

        Ok(())
    }

    fn parse_string(&mut self) -> Result<()> {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1
            };
            self.advance();
        }

        if self.is_at_end() {
            return Err(ScannerError::new(ScannerError::UnterminatedString {
                line: self.line,
            }));
        }

        self.advance();
        let start = self.start + 1;
        let end = self.current - 1;
        let text = &self.source[start..end];
        self.add_token_prime(TokenType::String, Literal::Str(text.to_string()))?;

        Ok(())
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

    fn parse_number(&mut self) -> Result<()> {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == b'.' && self.is_digit(self.peek_next()) {
            self.advance();
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let text = &self.source[self.start..self.current];
        let value: f64 = text.parse()?;
        self.add_token_prime(TokenType::Number, Literal::Number(value))?;
        Ok(())
    }

    fn is_digit(&self, c: u8) -> bool {
        c >= b'0' && c <= b'9'
    }

    fn parse_identifier(&mut self) -> Result<()> {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        if let Some(token_type) = keyword_capture(text) {
            self.add_token(token_type)
        } else {
            self.add_token(TokenType::Identifier)
        }
    }

    fn is_alpha(&self, c: u8) -> bool {
        matches!(c, b'a'..=b'z' | b'A'..=b'Z' | b'_')
    }

    fn is_alpha_numeric(&self, c: u8) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() {
            return b'\0';
        }
        self.source.as_bytes()[self.current]
    }

    fn peek_next(&self) -> u8 {
        if self.current + 1 >= self.source.len() {
            return b'\0';
        }
        let i = self.current + 1;
        self.source.as_bytes()[i]
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> u8 {
        let char = self.source.as_bytes()[self.current];
        self.current += 1;
        char
    }

    fn add_token(&mut self, t: TokenType) -> Result<()> {
        self.add_token_prime(t, Literal::Nil)
    }

    fn add_token_prime(&mut self, t: TokenType, literal: Literal) -> Result<()> {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token {
            ttype: t,
            lexeme: text.to_string(),
            literal: literal,
            line: self.line,
        });

        Ok(())
    }
}

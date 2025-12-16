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
            _ => (),
        }
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

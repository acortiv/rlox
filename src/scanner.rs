use crate::token::{Literal, Token, TokenType};

// Source needs to be made into U8 as Rust uses UTF-8... indexing strings is unsafe and O(n)
#[derive(Clone, Debug)]
pub struct Scanner<'a> {
    pub source: &'a [u8],
    pub tokens: Vec<Token<'a>>,
    pub start: usize,
    pub current: usize,
    pub line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(mut self) -> Vec<Token<'a>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()
        }

        let empty = &self.source[self.current..self.current];
        self.tokens.push(Token {
            ttype: TokenType::EOF,
            lexeme: empty,
            literal: Literal::Nil,
            line: self.line,
        });
        self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        self.current += 1
    }

    fn advance(&mut self) -> u8 {
        let char = self.source[self.current];
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
            lexeme: text,
            literal: literal,
            line: self.line,
        });
    }
}

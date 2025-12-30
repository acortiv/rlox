use crate::{
    expr::Expr,
    token::{Token, TokenType},
};

#[derive(Clone, Debug)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            current: 0,
        }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        let potential_tokens: [TokenType; 2] = [TokenType::BangEqual, TokenType::EqualEqual];
        while self.match_token(&potential_tokens) {
            let operator = self.previous().clone();
            let right = self.comparison();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            }
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        let potential_tokens: [TokenType; 4] = [
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ];
        while self.match_token(&potential_tokens) {
            let operator = self.previous().clone();
            let right = self.term();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            }
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        let potential_tokens: [TokenType; 2] = [TokenType::Minus, TokenType::Plus];
        while self.match_token(&potential_tokens) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            }
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        let potential_tokens: [TokenType; 2] = [TokenType::Slash, TokenType::Star];
        while self.match_token(&potential_tokens) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            }
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        let potential_tokens: [TokenType; 2] = [TokenType::Bang, TokenType::Minus];
        if self.match_token(&potential_tokens) {
            let operator = self.previous().clone();
            let right = self.unary();
            return Expr::Unary {
                operator: operator,
                right: Box::new(right),
            };
        }

        self.primary()
    }

    // TODO: Implement Error Handling.  Parser Errors being UnexpectedToken & UnterminatedGroup
    // fn primary(&mut self) -> Expr {}

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for &t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, t: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().ttype == t
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().ttype == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}

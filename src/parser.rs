use crate::{
    error::{ParserError, report},
    expr::Expr,
    token::{Literal, Token, TokenType},
};

type Result<T> = std::result::Result<T, ParserError>;

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

    pub fn parse(&mut self) -> Option<Expr> {
        let Ok(expr) = self.expression() else {
            return None;
        };

        Some(expr)
    }

    fn expression(&mut self) -> Result<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut expr = self.comparison()?;
        let potential_tokens: [TokenType; 2] = [TokenType::BangEqual, TokenType::EqualEqual];
        while self.match_token(&potential_tokens) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr> {
        let mut expr = self.term()?;
        let potential_tokens: [TokenType; 4] = [
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ];
        while self.match_token(&potential_tokens) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr> {
        let mut expr = self.factor()?;
        let potential_tokens: [TokenType; 2] = [TokenType::Minus, TokenType::Plus];
        while self.match_token(&potential_tokens) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr> {
        let mut expr = self.unary()?;
        let potential_tokens: [TokenType; 2] = [TokenType::Slash, TokenType::Star];
        while self.match_token(&potential_tokens) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr> {
        let potential_tokens: [TokenType; 2] = [TokenType::Bang, TokenType::Minus];
        if self.match_token(&potential_tokens) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator: operator,
                right: Box::new(right),
            });
        }

        self.primary()
    }

    // TODO: Implement Error Handling.  Parser Errors being UnexpectedToken & UnterminatedGroup
    fn primary(&mut self) -> Result<Expr> {
        if self.match_token(&[TokenType::False]) {
            return Ok(Expr::Literal(Literal::Bool(false)));
        }

        if self.match_token(&[TokenType::True]) {
            return Ok(Expr::Literal(Literal::Bool(true)));
        }

        if self.match_token(&[TokenType::Number]) {
            if let Literal::Number(n) = &self.previous().literal {
                return Ok(Expr::Literal(Literal::Number(*n)));
            }
            unreachable!("Number token without number literal.")
        }

        if self.match_token(&[TokenType::String]) {
            if let Literal::Str(s) = &self.previous().literal {
                return Ok(Expr::Literal(Literal::Str(s.clone())));
            }
            unreachable!("String token without string literal.")
        }

        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            let _ = self.consume(TokenType::RightParen)?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        let err = ParserError::UnexpectedToken(self.peek().clone());
        report(&err);
        Err(err)
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for &t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn consume(&mut self, t: TokenType) -> Result<&Token> {
        if self.check(t) {
            return Ok(self.advance());
        }

        let err = ParserError::UnterminatedGroup(self.peek().clone());
        report(&err);
        Err(err)
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

    // fn synchronize(&mut self) {
    //     self.advance();
    //     while !self.is_at_end() {
    //         if self.previous().ttype == TokenType::Semicolon {
    //             return;
    //         }

    //         match self.peek().ttype {
    //             TokenType::Class
    //             | TokenType::For
    //             | TokenType::Fun
    //             | TokenType::If
    //             | TokenType::Print
    //             | TokenType::Return
    //             | TokenType::While
    //             | TokenType::Var => return,
    //             _ => {}
    //         }

    //         self.advance();
    //     }
    // }
}

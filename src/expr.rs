use crate::token::{Literal, Token};
use std::{fmt, rc::Rc};

#[derive(Clone, Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Rc<Token>,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
    Literal(Literal),
    Variable(Rc<Token>),
    Unary {
        operator: Rc<Token>,
        right: Box<Expr>,
    },
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                write!(f, "({}, {}, {})", left, operator, right)
            }
            Expr::Grouping(expr) => write!(f, "(group {})", expr),
            Expr::Literal(literal) => write!(f, "{literal}"),
            Expr::Variable(var) => write!(f, "{var}"),
            Expr::Unary { operator, right } => {
                write!(f, "({}, {})", operator, right)
            }
        }
    }
}

/// Pretty-print an expression as an indented tree
///
/// # Example
///
/// ````
/// let expr = Expr::Binary {
///         left: Box::new(Expr::Unary {
///             operator: rlox::token::Token {
///                 ttype: rlox::token::TokenType::Minus,
///                 lexeme: String::from("-"),
///                 literal: rlox::token::Literal::Nil,
///                 line: 1,
///             },
///             right: Box::new(Expr::Literal(rlox::token::Literal::Number(123.0))),
///         }),
///         operator: rlox::token::Token {
///             ttype: rlox::token::TokenType::Star,
///             lexeme: String::from("*"),
///             literal: rlox::token::Literal::Nil,
///             line: 1,
///         },
///         right: Box::new(Expr::Grouping(Box::new(Expr::Literal(
///             rlox::token::Literal::Number(45.67),
///         )))),
///     };

/// let expr_string = pretty_expr(&expr, 0);
/// println!("{}", expr_string);
/// ```

pub fn pretty_expr(expr: &Expr, indent: usize) -> String {
    let pad = " ".repeat(indent);
    match expr {
        Expr::Binary {
            left,
            operator,
            right,
        } => {
            format!(
                "{}Binary({})\n{}{}\n{}{}",
                pad,
                operator.lexeme,
                pretty_expr(left, indent + 1),
                pad,
                pretty_expr(right, indent + 1),
                pad
            )
        }
        Expr::Literal(literal) => format!("{}Literal{}", pad, literal),
        Expr::Variable(var) => format!("{}Variable{}", pad, var),
        Expr::Unary { operator, right } => {
            format!(
                "{}Unary({})\n{}",
                pad,
                operator.lexeme,
                pretty_expr(right, indent + 1)
            )
        }
        Expr::Grouping(inner) => format!("{}Group\n{}", pad, pretty_expr(inner, indent + 1)),
    }
}

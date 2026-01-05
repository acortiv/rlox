use std::fmt;

use crate::expr::Expr;

#[derive(Clone, Debug)]
pub enum Stmt {
    Expression(Box<Expr>),
    Print(Box<Expr>),
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::Expression(expr) => write!(f, "Expression Statement: {expr}"),
            Stmt::Print(expr) => write!(f, "Print Statment: {expr}"),
        }
    }
}

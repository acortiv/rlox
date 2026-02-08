use std::{fmt, rc::Rc};

use crate::{expr::Expr, token::Token};

#[derive(Clone, Debug)]
pub enum Stmt {
    Expression(Box<Expr>),
    Print(Box<Expr>),
    Var {
        name: Rc<Token>,
        initializer: Option<Box<Expr>>,
    },
    Block(Vec<Stmt>),
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::Expression(expr) => write!(f, "Expression Statement: {expr}"),
            Stmt::Print(expr) => write!(f, "Print Statment: {expr}"),
            Stmt::Var { name, initializer } => {
                if let Some(init) = initializer {
                    write!(f, "Token: {name}, Initializer: {init}")
                } else {
                    write!(f, "Token: {name}, Initializer: None")
                }
            }
            Stmt::Block(stmts) => write!(f, "Statements: {:?}", stmts),
        }
    }
}

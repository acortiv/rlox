use std::rc::Rc;

use crate::{
    expr::Expr,
    token::{Literal, Token, TokenType},
};

type Result = std::result::Result<RuntimeValue, RuntimeError>;

#[derive(Debug)]
pub enum RuntimeError {
    TypeError,
}

impl std::error::Error for RuntimeError {}

#[derive(Debug, PartialEq)]
pub enum RuntimeValue {
    Number(f64),
    Bool(bool),
    Str(String),
    Nil,
}

impl RuntimeValue {
    pub fn is_truthy(&self) -> bool {
        match self {
            RuntimeValue::Nil => false,
            RuntimeValue::Bool(b) => *b,
            _ => true,
        }
    }
}

impl From<f64> for RuntimeValue {
    fn from(value: f64) -> Self {
        RuntimeValue::Number(value)
    }
}

impl From<bool> for RuntimeValue {
    fn from(value: bool) -> Self {
        RuntimeValue::Bool(value)
    }
}

fn is_equal(a: RuntimeValue, b: RuntimeValue) -> bool {
    match (&a, &b) {
        (RuntimeValue::Nil, RuntimeValue::Nil) => return true,
        (RuntimeValue::Nil, _) => return false,
        _ => a == b,
    }
}

#[derive(Clone, Debug)]
pub struct Interpreter {}

impl Interpreter {
    pub fn evaluate(&self, expr: &Expr) -> Result {
        match expr {
            Expr::Binary {
                left,
                operator,
                right,
            } => self.eval_binary(left, operator, right),
            Expr::Grouping(expr) => self.evaluate(expr),
            Expr::Unary { operator, right } => self.eval_unary(operator, right),
            Expr::Literal(literal) => match literal {
                Literal::Number(num) => Ok(RuntimeValue::Number(*num)),
                Literal::Bool(bool) => Ok(RuntimeValue::Bool(*bool)),
                Literal::Str(str) => Ok(RuntimeValue::Str(str.clone())),
                _ => Ok(RuntimeValue::Nil),
            },
        }
    }

    fn eval_binary(&self, left: &Expr, operator: &Rc<Token>, right: &Expr) -> Result {
        let l = self.evaluate(left)?;
        let r = self.evaluate(right)?;

        match operator.ttype {
            TokenType::Greater => self.apply_bin_op(l, r, |a, b| a > b),
            TokenType::Minus => self.apply_bin_op(l, r, |a, b| a - b),
            TokenType::Plus => match (l, r) {
                (RuntimeValue::Number(l), RuntimeValue::Number(r)) => {
                    Ok(RuntimeValue::Number(l + r))
                }
                (RuntimeValue::Str(l), RuntimeValue::Str(r)) => {
                    Ok(RuntimeValue::Str(format!("{l}{r}")))
                }
                _ => Err(RuntimeError::TypeError),
            },
            TokenType::Slash => self.apply_bin_op(l, r, |a, b| a / b),
            TokenType::Star => self.apply_bin_op(l, r, |a, b| a * b),
            _ => Err(RuntimeError::TypeError),
        }
    }

    fn eval_unary(&self, operator: &Rc<Token>, right: &Expr) -> Result {
        let r = self.evaluate(right)?;

        match operator.ttype {
            TokenType::Bang => Ok(RuntimeValue::Bool(!r.is_truthy())),
            TokenType::Minus => {
                let RuntimeValue::Number(r_) = r else {
                    return Err(RuntimeError::TypeError);
                };
                Ok(RuntimeValue::Number(-r_))
            }
            _ => Err(RuntimeError::TypeError),
        }
    }

    fn apply_bin_op<F, R>(&self, a: RuntimeValue, b: RuntimeValue, f: F) -> Result
    where
        F: Fn(f64, f64) -> R,
        R: Into<RuntimeValue>,
    {
        let (RuntimeValue::Number(l), RuntimeValue::Number(r)) = (a, b) else {
            return Err(RuntimeError::TypeError);
        };

        Ok(f(l, r).into())
    }
}

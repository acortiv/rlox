use std::{fmt, rc::Rc};

use crate::{
    error::{RuntimeError, report},
    expr::Expr,
    token::{Literal, Token, TokenType},
};

type Result<T> = std::result::Result<T, RuntimeError>;

#[derive(Clone, Debug, PartialEq)]
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

impl fmt::Display for RuntimeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeValue::Nil => write!(f, "nil"),
            RuntimeValue::Bool(bool) => write!(f, "{bool}"),
            RuntimeValue::Number(num) => {
                let mut s = num.to_string();
                if s.ends_with(".0") {
                    s.truncate(s.len() - 2);
                }
                write!(f, "{s}")
            }
            RuntimeValue::Str(str) => write!(f, "{str}"),
        }
    }
}

fn is_equal(a: &RuntimeValue, b: &RuntimeValue) -> bool {
    match (a, b) {
        (RuntimeValue::Nil, RuntimeValue::Nil) => return true,
        (RuntimeValue::Nil, _) | (_, RuntimeValue::Nil) => return false,
        _ => a == b,
    }
}

#[derive(Clone, Debug, Default)]
pub struct Interpreter;

impl Interpreter {
    pub fn interpret(&self, expr: &Expr) -> Result<String> {
        Ok(self.evaluate(&expr)?.to_string())
    }

    fn evaluate(&self, expr: &Expr) -> Result<RuntimeValue> {
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

    fn eval_binary(&self, left: &Expr, operator: &Rc<Token>, right: &Expr) -> Result<RuntimeValue> {
        let l = self.evaluate(left)?;
        let r = self.evaluate(right)?;

        match operator.ttype {
            TokenType::Greater => self.apply_bin_op(l, r, operator, |a, b| a > b),
            TokenType::GreaterEqual => self.apply_bin_op(l, r, operator, |a, b| a >= b),
            TokenType::Less => self.apply_bin_op(l, r, operator, |a, b| a < b),
            TokenType::LessEqual => self.apply_bin_op(l, r, operator, |a, b| a <= b),
            TokenType::BangEqual => Ok(RuntimeValue::Bool(!is_equal(&l, &r))),
            TokenType::EqualEqual => Ok(RuntimeValue::Bool(is_equal(&l, &r))),
            TokenType::Minus => self.apply_bin_op(l, r, operator, |a, b| a - b),
            TokenType::Plus => match (l, r) {
                (RuntimeValue::Number(l), RuntimeValue::Number(r)) => {
                    Ok(RuntimeValue::Number(l + r))
                }
                (RuntimeValue::Str(l), RuntimeValue::Str(r)) => {
                    Ok(RuntimeValue::Str(format!("{l}{r}")))
                }
                (RuntimeValue::Str(l), RuntimeValue::Number(r)) => {
                    let s = r.to_string();
                    Ok(RuntimeValue::Str(format!("{l}{s}")))
                }
                (RuntimeValue::Number(l), RuntimeValue::Str(r)) => {
                    let s = l.to_string();
                    Ok(RuntimeValue::Str(format!("{s}{r}")))
                }
                _ => {
                    let err = RuntimeError::TypeError(Rc::clone(operator));
                    report(&err);
                    return Err(err);
                }
            },
            TokenType::Slash => self.div(l, r, operator),
            TokenType::Star => self.apply_bin_op(l, r, operator, |a, b| a * b),
            _ => {
                let err = RuntimeError::TypeError(Rc::clone(operator));
                report(&err);
                return Err(err);
            }
        }
    }

    fn eval_unary(&self, operator: &Rc<Token>, right: &Expr) -> Result<RuntimeValue> {
        let r = self.evaluate(right)?;

        match operator.ttype {
            TokenType::Bang => Ok(RuntimeValue::Bool(!r.is_truthy())),
            TokenType::Minus => {
                let RuntimeValue::Number(r_) = r else {
                    let err = RuntimeError::TypeError(Rc::clone(operator));
                    report(&err);
                    return Err(err);
                };
                Ok(RuntimeValue::Number(-r_))
            }
            _ => {
                let err = RuntimeError::TypeError(Rc::clone(operator));
                report(&err);
                return Err(err);
            }
        }
    }

    fn div(&self, a: RuntimeValue, b: RuntimeValue, operator: &Rc<Token>) -> Result<RuntimeValue> {
        let (RuntimeValue::Number(l), RuntimeValue::Number(r)) = (a, b) else {
            let err = RuntimeError::TypeError(Rc::clone(operator));
            report(&err);
            return Err(err);
        };

        if r == 0.0 || !r.is_finite() {
            let err = RuntimeError::DivByZero(Rc::clone(operator));
            report(&err);
            return Err(err);
        }

        Ok(RuntimeValue::Number(l / r))
    }

    fn apply_bin_op<F, R>(
        &self,
        a: RuntimeValue,
        b: RuntimeValue,
        operator: &Rc<Token>,
        f: F,
    ) -> Result<RuntimeValue>
    where
        F: Fn(f64, f64) -> R,
        R: Into<RuntimeValue>,
    {
        let (RuntimeValue::Number(l), RuntimeValue::Number(r)) = (a, b) else {
            let err = RuntimeError::TypeError(Rc::clone(operator));
            report(&err);
            return Err(err);
        };

        Ok(f(l, r).into())
    }
}

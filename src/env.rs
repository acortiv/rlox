use crate::{error::RuntimeError, interpreter::RuntimeValue, token::Token};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone, Debug, Default)]
pub struct Environment {
    values: HashMap<String, Rc<RefCell<Option<RuntimeValue>>>>,
}

impl Environment {
    pub fn define(&mut self, name: String, value: Option<RuntimeValue>) -> Option<RuntimeValue> {
        self.values
            .entry(name)
            .or_insert_with(|| Rc::new(RefCell::new(None)))
            .replace(value)
    }

    pub fn get(&self, name: &Rc<Token>) -> Result<RuntimeValue, RuntimeError> {
        let cell = self
            .values
            .get(&name.lexeme)
            .cloned()
            .ok_or_else(|| RuntimeError::UndefinedVariable(name.clone()))?;

        Ok(cell.borrow().clone().unwrap_or(RuntimeValue::Nil))
    }
}

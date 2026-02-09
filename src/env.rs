use crate::{error::RuntimeError, interpreter::RuntimeValue, token::Token};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone, Debug, Default)]
pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Rc<RefCell<RuntimeValue>>>,
}

impl Environment {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            enclosing: None,
            values: HashMap::new(),
        }))
    }

    pub fn new_from(enclosing: Rc<RefCell<Environment>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            enclosing: Some(enclosing),
            values: HashMap::new(),
        }))
    }

    pub fn define(
        &mut self,
        name: String,
        value: RuntimeValue,
    ) -> Option<Rc<RefCell<RuntimeValue>>> {
        self.values.insert(name, Rc::new(RefCell::new(value)))
    }

    pub fn get(&self, name: &Rc<Token>) -> Result<RuntimeValue, RuntimeError> {
        if let Some(cell) = self.values.get(&name.lexeme) {
            return Ok(cell.borrow().clone());
        }

        if let Some(ref enclosing) = self.enclosing {
            return enclosing.borrow().get(name);
        }

        Err(RuntimeError::UndefinedVariable(name.clone()))
    }

    pub fn assign(
        &self,
        name: &Rc<Token>,
        value: RuntimeValue,
    ) -> Result<RuntimeValue, RuntimeError> {
        if let Some(cell) = self.values.get(&name.lexeme) {
            *cell.borrow_mut() = value.clone();
            return Ok(value);
        }

        if let Some(ref enclosing) = self.enclosing {
            return enclosing.borrow().assign(name, value);
        }

        Err(RuntimeError::UndefinedVariable(name.clone()))
    }
}

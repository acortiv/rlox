use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{error::RuntimeError, interpreter::RuntimeValue, token::Token};

#[derive(Clone, Debug)]
pub struct Environment {
    values: HashMap<String, Rc<RefCell<RuntimeValue>>>,
}

impl Environment {
    fn define(
        &mut self,
        name: String,
        value: Rc<RefCell<RuntimeValue>>,
    ) -> Option<Rc<RefCell<RuntimeValue>>> {
        self.values.insert(name, value)
    }

    fn get(&self, name: &Rc<Token>) -> Result<Rc<RefCell<RuntimeValue>>, RuntimeError> {
        self.values
            .get(&name.lexeme)
            .cloned()
            .ok_or_else(|| RuntimeError::UndefinedVariable(Rc::clone(name)))
    }
}

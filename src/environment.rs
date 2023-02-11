use crate::token::{Literal, Token};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, Literal>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn new_enclosing(enclosing: Box<Environment>) -> Environment {
        Environment {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Option<&Literal> {
        let r1 = self.values.get(&name.lexeme);
        if r1.is_some() {
            r1
        } else if let Some(ref enclosing) = self.enclosing {
            enclosing.get(&name)
        } else {
            None
        }
    }

    pub fn assign(&mut self, name: &Token, value: &Literal) -> bool {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value.clone());
            true
        } else if let Some(ref mut enclosing) = self.enclosing {
            enclosing.assign(&name, value)
        } else {
            false
        }
    }

    pub fn enclosing(&self) -> Option<Box<Environment>> {
        self.enclosing.clone()
    }
}

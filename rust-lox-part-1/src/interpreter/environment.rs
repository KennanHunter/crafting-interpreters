use std::collections::HashMap;

use crate::{errors::RuntimeError, tree::expression::ExpressionLiteral};

#[derive(Debug, Default)]
pub struct Environment<'a> {
    map: HashMap<String, ExpressionLiteral>,
    pub enclosing_environment: Option<&'a mut Environment<'a>>,
}

impl<'a> Environment<'a> {
    pub fn new() -> Environment<'a> {
        Environment {
            map: HashMap::new(),
            enclosing_environment: None,
        }
    }

    pub fn with_parent(parent: &'a mut Environment<'a>) -> Environment<'a> {
        Environment {
            map: HashMap::new(),
            enclosing_environment: Some(parent),
        }
    }

    pub fn get_variable(
        &self,
        line_number: usize,
        name: String,
    ) -> Result<ExpressionLiteral, RuntimeError> {
        let read_variable = self.map.get(&name);

        if let Some(literal) = read_variable {
            return Ok(literal.clone());
        }

        if let Some(enclosed_environment) = &self.enclosing_environment {
            return enclosed_environment.get_variable(line_number, name);
        }

        Err(RuntimeError {
            line_number,
            message: format!("Variable {name} not found in scope"),
        })
    }

    pub fn define_variable(
        &mut self,
        line_number: usize,
        name: String,
        value: ExpressionLiteral,
    ) -> Result<(), RuntimeError> {
        if self.map.contains_key(&name) {
            return Err(RuntimeError {
                line_number,
                message: format!("Variable {name} already defined"),
            });
        }

        self.map.insert(name, value);

        Ok(())
    }

    pub fn set_variable(
        &mut self,
        line_number: usize,
        name: String,
        value: ExpressionLiteral,
    ) -> Result<ExpressionLiteral, RuntimeError> {
        if !self.map.contains_key(&name) {
            if let Some(enclosed) = self.enclosing_environment.as_mut() {
                return enclosed.set_variable(line_number, name, value);
            }

            return Err(RuntimeError {
                line_number,
                message: format!("Variable {name} not defined"),
            });
        }

        self.map.insert(name, value.clone()).unwrap();

        Ok(value)
    }
}

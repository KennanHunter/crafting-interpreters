use std::collections::HashMap;

use crate::{errors::RuntimeError, tree::expression::ExpressionLiteral};

#[derive(Debug, Default)]
pub struct Environment {
    pub variables: VariableStore,
}

#[derive(Debug, Default)]
pub struct VariableStore {
    map: HashMap<String, ExpressionLiteral>,
}

impl VariableStore {
    pub fn new() -> VariableStore {
        VariableStore {
            map: HashMap::new(),
        }
    }

    pub fn get_variable(
        &self,
        line_number: usize,
        name: String,
    ) -> Result<ExpressionLiteral, RuntimeError> {
        match self.map.get(&name) {
            Some(value) => Ok(value.clone()),
            None => Err(RuntimeError {
                line_number,
                message: format!("Undefined variable {name}"),
            }),
        }
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
}

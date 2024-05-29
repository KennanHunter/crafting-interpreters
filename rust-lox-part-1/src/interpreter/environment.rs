use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{errors::RuntimeError, tree::expression::ExpressionLiteral};

#[derive(Debug, Default)]
pub struct Environment<'a> {
    map: Rc<RefCell<HashMap<String, ExpressionLiteral>>>,
    pub enclosing_environment: Option<&'a Environment<'a>>,
}

impl<'a> Environment<'a> {
    pub fn new() -> Environment<'a> {
        Environment {
            map: Rc::new(RefCell::new(HashMap::new())),
            enclosing_environment: None,
        }
    }

    pub fn with_parent(parent: &'a Environment<'a>) -> Environment<'a> {
        Environment {
            map: Rc::new(RefCell::new(HashMap::new())),
            enclosing_environment: Some(parent),
        }
    }

    pub fn get_variable(
        &self,
        line_number: usize,
        name: String,
    ) -> Result<ExpressionLiteral, RuntimeError> {
        let read_variable = self.map.borrow().get(&name).cloned();

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
        &self,
        line_number: usize,
        name: String,
        value: ExpressionLiteral,
    ) -> Result<(), RuntimeError> {
        if self.map.borrow().contains_key(&name) {
            return Err(RuntimeError {
                line_number,
                message: format!("Variable {name} already defined"),
            });
        }

        self.map.borrow_mut().insert(name, value);

        Ok(())
    }

    pub fn set_variable(
        &self,
        line_number: usize,
        name: String,
        value: ExpressionLiteral,
    ) -> Result<ExpressionLiteral, RuntimeError> {
        if !self.map.borrow().contains_key(&name) {
            if let Some(enclosed) = self.enclosing_environment {
                return enclosed.set_variable(line_number, name, value);
            }

            return Err(RuntimeError {
                line_number,
                message: format!("Variable {name} not defined"),
            });
        }

        self.map.borrow_mut().insert(name, value.clone()).unwrap();

        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::tree::expression::ExpressionLiteral;

    use super::Environment;

    #[test]
    fn test_environment_with_parent() {
        let parent = Environment::new();

        let child = Environment::with_parent(&parent);

        child
            .map
            .borrow_mut()
            .insert("child_test".to_owned(), ExpressionLiteral::True);

        child
            .enclosing_environment
            .unwrap()
            .map
            .borrow_mut()
            .insert("test".to_owned(), ExpressionLiteral::True);

        let value = parent.map.borrow().get("test").cloned();

        assert_eq!(value, Some(ExpressionLiteral::True))
    }

    #[test]
    fn test_environment_with_acquisition_to_parent() {
        let parent = Environment::new();

        let child = Environment::with_parent(&parent);

        let definition = parent.define_variable(0, "name".to_owned(), ExpressionLiteral::True);

        assert!(definition.is_ok());

        let value = child.get_variable(0, "name".to_owned());

        assert_eq!(value, Ok(ExpressionLiteral::True))
    }
}

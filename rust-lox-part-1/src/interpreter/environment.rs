use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{errors::RuntimeError, tree::expression::ExpressionLiteral};

#[derive(Debug, Default, Clone)]
pub struct Environment {
    map: Rc<RefCell<HashMap<String, ExpressionLiteral>>>,
    pub enclosing_environment: Option<Rc<RefCell<Environment>>>,
}

pub type EnvironmentRef = Rc<RefCell<Environment>>;

impl Environment {
    pub fn new() -> Environment {
        Environment {
            map: Rc::new(RefCell::new(HashMap::new())),
            enclosing_environment: None,
        }
    }

    pub fn with_parent(parent: Rc<RefCell<Environment>>) -> Environment {
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
            return enclosed_environment
                .borrow()
                .get_variable(line_number, name);
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
            if let Some(enclosed) = &self.enclosing_environment {
                return enclosed.borrow_mut().set_variable(line_number, name, value);
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
    use std::{cell::RefCell, rc::Rc};

    use crate::tree::expression::ExpressionLiteral;

    use super::Environment;

    #[test]
    fn test_environment_with_parent() {
        let parent = Rc::new(RefCell::new(Environment::new()));

        let child = Environment::with_parent(parent.clone());

        child
            .map
            .borrow_mut()
            .insert("child_test".to_owned(), ExpressionLiteral::True);

        child
            .enclosing_environment
            .unwrap()
            .borrow()
            .map
            .borrow_mut()
            .insert("test".to_owned(), ExpressionLiteral::True);

        let value = parent.borrow().map.borrow().get("test").cloned();

        assert_eq!(value, Some(ExpressionLiteral::True))
    }

    #[test]
    fn test_environment_with_acquisition_to_parent() {
        let parent = Rc::new(RefCell::new(Environment::new()));

        let child = Environment::with_parent(parent.clone());

        let definition =
            parent
                .borrow()
                .define_variable(0, "name".to_owned(), ExpressionLiteral::True);

        assert!(definition.is_ok());

        let value = child.get_variable(0, "name".to_owned());

        assert_eq!(value, Ok(ExpressionLiteral::True))
    }
}

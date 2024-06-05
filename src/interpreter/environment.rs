use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    errors::RuntimeError,
    resolver::VariableMap,
    tree::expression::{ExpressionLiteral, ExpressionVariable},
};

#[derive(Debug, Default, Clone)]
pub struct Environment {
    resolved_variable_map: Option<Rc<VariableMap>>,
    active_variable_map: Rc<RefCell<HashMap<String, ExpressionLiteral>>>,
    pub parent_environment: Option<Rc<RefCell<Environment>>>,
}

pub type EnvironmentRef = Rc<RefCell<Environment>>;

impl Environment {
    pub fn new() -> Environment {
        Environment {
            resolved_variable_map: None,
            active_variable_map: Rc::new(RefCell::new(HashMap::new())),
            parent_environment: None,
        }
    }

    pub fn with_resolved_variable_map(variable_map: VariableMap) -> Self {
        Environment {
            resolved_variable_map: Some(Rc::new(variable_map)),
            active_variable_map: Rc::new(RefCell::new(HashMap::new())),
            parent_environment: None,
        }
    }

    pub fn with_parent(parent: Rc<RefCell<Environment>>) -> Environment {
        Environment {
            active_variable_map: Rc::new(RefCell::new(HashMap::new())),
            parent_environment: Some(parent),
            resolved_variable_map: None,
        }
    }

    pub fn get_variable(
        &self,
        line_number: usize,
        name: String,
    ) -> Result<ExpressionLiteral, RuntimeError> {
        let read_variable = self.active_variable_map.borrow().get(&name).cloned();

        if let Some(literal) = read_variable {
            return Ok(literal.clone());
        }

        if let Some(parent_environment) = &self.parent_environment {
            return parent_environment.borrow().get_variable(line_number, name);
        }

        Err(RuntimeError {
            line_number,
            message: format!("Variable {name} not found in scope"),
        })
    }

    pub fn get_variable_at(
        &self,
        line_number: usize,
        name: String,
        depth: usize,
    ) -> Result<ExpressionLiteral, RuntimeError> {
        match depth {
            1.. => {
                if let Some(parent_environment) = &self.parent_environment {
                    return parent_environment.borrow().get_variable_at(
                        line_number,
                        name,
                        depth - 1,
                    );
                } else {
                    unreachable!("parent environment referenced but does not exist")
                }
            }
            0 => {
                let read_variable = self.active_variable_map.borrow().get(&name).cloned();

                if let Some(literal) = read_variable {
                    return Ok(literal.clone());
                }

                Err(RuntimeError {
                    line_number,
                    message: format!("Variable {name} not found in scope"),
                })
            }
        }
    }

    pub fn define_variable(
        &self,
        line_number: usize,
        name: String,
        value: ExpressionLiteral,
    ) -> Result<(), RuntimeError> {
        if self.active_variable_map.borrow().contains_key(&name) {
            return Err(RuntimeError {
                line_number,
                message: format!("Variable {name} already defined"),
            });
        }

        self.active_variable_map.borrow_mut().insert(name, value);

        Ok(())
    }

    pub fn set_variable(
        &self,
        line_number: usize,
        name: String,
        value: ExpressionLiteral,
    ) -> Result<ExpressionLiteral, RuntimeError> {
        if !self.active_variable_map.borrow().contains_key(&name) {
            if let Some(parent_environment) = &self.parent_environment {
                return parent_environment
                    .borrow_mut()
                    .set_variable(line_number, name, value);
            }

            return Err(RuntimeError {
                line_number,
                message: format!("Variable {name} not defined"),
            });
        }

        self.active_variable_map
            .borrow_mut()
            .insert(name, value.clone())
            .unwrap();

        Ok(value)
    }

    pub fn set_variable_at(
        &self,
        line_number: usize,
        name: String,
        value: ExpressionLiteral,
        depth: usize,
    ) -> Result<ExpressionLiteral, RuntimeError> {
        match depth {
            1.. => {
                if let Some(parent_environment) = &self.parent_environment {
                    return parent_environment.borrow_mut().set_variable_at(
                        line_number,
                        name,
                        value,
                        depth - 1,
                    );
                } else {
                    unreachable!("parent environment referenced but does not exist")
                }
            }
            0 => {
                self.active_variable_map
                    .borrow_mut()
                    .insert(name, value.clone());

                Ok(value)
            }
        }
    }

    fn get_variable_map(&self) -> Rc<VariableMap> {
        if let Some(parent) = &self.parent_environment {
            parent.borrow().get_variable_map()
        } else {
            self.resolved_variable_map
                .clone()
                .expect("Global environment doesn't contain variable map from resolver")
        }
    }

    pub fn get_variable_with_depth(
        &self,
        variable: ExpressionVariable,
    ) -> Result<ExpressionLiteral, RuntimeError> {
        let potential_depth = self.get_variable_map().get(&variable).cloned();

        match potential_depth {
            Some(depth) => {
                self.get_variable_at(variable.line_number, variable.identifier_name, depth)
            }
            None => Err(RuntimeError {
                line_number: variable.line_number,
                // TODO: err
                message: format!("Something fucked up"),
            }),
        }
    }

    pub fn set_variable_with_depth(
        &mut self,
        variable: ExpressionVariable,
        value: ExpressionLiteral,
    ) -> Result<ExpressionLiteral, RuntimeError> {
        let potential_depth = self.get_variable_map().get(&variable).cloned();

        match potential_depth {
            Some(depth) => {
                self.set_variable_at(variable.line_number, variable.identifier_name, value, depth)
            }
            None => Err(RuntimeError {
                line_number: variable.line_number,
                // TODO: err
                message: format!("Something fucked up"),
            }),
        }
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
            .active_variable_map
            .borrow_mut()
            .insert("child_test".to_owned(), ExpressionLiteral::True);

        child
            .parent_environment
            .unwrap()
            .borrow()
            .active_variable_map
            .borrow_mut()
            .insert("test".to_owned(), ExpressionLiteral::True);

        let value = parent
            .borrow()
            .active_variable_map
            .borrow()
            .get("test")
            .cloned();

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

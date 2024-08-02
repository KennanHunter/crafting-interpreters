pub mod native;
use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

use crate::{errors::RuntimeError, tree::expression::ExpressionLiteral};

use super::types::BlockReturn;

#[derive(Debug, PartialEq, Clone)]
pub enum Reference {
    CallableReference(CallableReference),
    ClassReference(ClassReference),
    InstanceReference(InstanceReference),
}

#[derive(Clone)]
pub struct CallableReference {
    pub arity: usize,
    pub subroutine: Rc<dyn Fn(usize, Vec<ExpressionLiteral>) -> Result<BlockReturn, RuntimeError>>,
}

impl Debug for CallableReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "( func {} )", self.arity)
    }
}

/**
 * Checks if the references are to the same subroutine
 */
impl PartialEq for CallableReference {
    fn eq(&self, other: &Self) -> bool {
        self.arity == other.arity && Rc::ptr_eq(&self.subroutine, &other.subroutine)
    }

    fn ne(&self, other: &Self) -> bool {
        self.arity != other.arity || !Rc::ptr_eq(&self.subroutine, &other.subroutine)
    }
}

// TODO: Test

#[derive(Clone)]
pub struct ClassReference {
    pub name: String,
    pub methods: Rc<RefCell<HashMap<String, CallableReference>>>,
}

impl Debug for ClassReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "( class {} )", self.name)
    }
}

/**
 * Checks if the references are to the same subroutine
 */
impl PartialEq for ClassReference {
    fn eq(&self, _other: &Self) -> bool {
        false
    }

    fn ne(&self, _other: &Self) -> bool {
        true
    }
}

#[derive(Clone)]
pub struct InstanceReference {
    pub class: ClassReference,
    pub fields: Rc<RefCell<HashMap<String, ExpressionLiteral>>>,
}

impl InstanceReference {
    pub fn instantiate(class: ClassReference) -> Self {
        InstanceReference {
            class,
            fields: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn get_property(
        &self,
        line_number: usize,
        property_name: &str,
    ) -> Result<ExpressionLiteral, RuntimeError> {
        let prop = self.fields.borrow().get(property_name).cloned();

        if prop.is_some() {
            return Ok(prop.unwrap());
        }

        let method = self.class.methods.borrow().get(property_name).cloned();

        if method.is_some() {
            return Ok(ExpressionLiteral::Reference(Reference::CallableReference(
                method.unwrap(),
            )));
        }

        Err(RuntimeError {
            line_number,
            message: format!("Unable to find property {property_name}"),
        })
    }

    pub fn set_property(
        &self,
        property_name: String,
        value: ExpressionLiteral,
    ) -> Result<ExpressionLiteral, RuntimeError> {
        self.fields
            .borrow_mut()
            .insert(property_name, value.clone());

        Ok(value)
    }
}

impl Debug for InstanceReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "( instance {} )", self.class.name)
    }
}

/**
 * Checks if the references are to the same subroutine
 */
impl PartialEq for InstanceReference {
    fn eq(&self, _other: &Self) -> bool {
        false
    }

    fn ne(&self, _other: &Self) -> bool {
        true
    }
}

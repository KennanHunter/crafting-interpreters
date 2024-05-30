pub mod native;
use std::{fmt::Debug, rc::Rc};

use crate::{errors::RuntimeError, tree::expression::ExpressionLiteral};

#[derive(Clone)]
pub struct CallableReference {
    pub arity: usize,
    pub subroutine:
        Rc<dyn Fn(Vec<ExpressionLiteral>) -> Result<Option<ExpressionLiteral>, RuntimeError>>,
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

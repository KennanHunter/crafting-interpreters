use std::{rc::Rc, time::UNIX_EPOCH};

use crate::{errors::RuntimeError, tree::expression::ExpressionLiteral};

use super::CallableReference;

pub fn create_native_now() -> CallableReference {
    CallableReference {
        arity: 0,
        subroutine: Rc::new(|_args| -> Result<Option<ExpressionLiteral>, RuntimeError> {
            let timestamp = UNIX_EPOCH.elapsed().unwrap().as_millis() as f64;
            let seconds = timestamp / 1000f64;

            return Ok(Some(ExpressionLiteral::Number(seconds)));
        }),
    }
}

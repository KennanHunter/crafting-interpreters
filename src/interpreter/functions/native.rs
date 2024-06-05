use std::{rc::Rc, time::UNIX_EPOCH};

use crate::{
    errors::RuntimeError, interpreter::types::BlockReturn, tree::expression::ExpressionLiteral,
};

use super::CallableReference;

pub fn create_native_now() -> CallableReference {
    CallableReference {
        arity: 0,
        subroutine: Rc::new(|_line_number, _args| -> Result<BlockReturn, RuntimeError> {
            let timestamp = UNIX_EPOCH.elapsed().unwrap().as_millis() as f64;
            let seconds = timestamp / 1000f64;

            Ok(BlockReturn::from(ExpressionLiteral::Number(seconds)))
        }),
    }
}

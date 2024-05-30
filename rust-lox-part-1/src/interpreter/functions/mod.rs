use std::time;

use crate::tree::expression::ExpressionLiteral;

use super::callable::Callable;

#[derive(Debug, PartialEq, Clone)]
pub enum CallableReference {
    Function(CallableFunction),
    NativeFunction(CallableNativeFunction),
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallableFunction();

impl Callable for CallableFunction {
    fn call(&self) -> ExpressionLiteral {
        todo!()
    }

    fn arity(&self) -> usize {
        todo!()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CallableNativeFunction {
    Now,
}

impl Callable for CallableNativeFunction {
    fn call(&self) -> ExpressionLiteral {
        match self {
            CallableNativeFunction::Now => {
                ExpressionLiteral::Number(time::Instant::now().elapsed().as_millis_f64() / 1000f64)
            }
        }
    }

    fn arity(&self) -> usize {
        match self {
            CallableNativeFunction::Now => 0,
        }
    }
}

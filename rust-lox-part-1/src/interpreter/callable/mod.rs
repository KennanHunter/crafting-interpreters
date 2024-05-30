use crate::tree::expression::ExpressionLiteral;

pub trait Callable {
    fn call(&self) -> ExpressionLiteral;

    fn arity(&self) -> usize;
}

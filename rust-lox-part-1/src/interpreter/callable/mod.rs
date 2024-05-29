use crate::tree::expression::ExpressionLiteral;

pub trait Callable {
    fn call() -> ExpressionLiteral;

    fn arity() -> usize;
}

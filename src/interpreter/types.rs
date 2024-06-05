use crate::tree::expression::ExpressionLiteral;

#[derive(Debug, Clone)]
pub enum BlockReturn {
    Returned(Option<ExpressionLiteral>),
    NoReturn,
}

impl From<ExpressionLiteral> for BlockReturn {
    fn from(value: ExpressionLiteral) -> Self {
        BlockReturn::Returned(Some(value))
    }
}

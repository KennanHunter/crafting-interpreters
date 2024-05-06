use crate::{
    errors::RuntimeError,
    tree::expression::{
        ComparisonOperation, EqualityOperation, Expression, ExpressionLiteral, FactorOperation,
        Operation, TermOperation, UnaryOperation,
    },
};

pub fn interpret_tree(tree: Expression) -> Result<RuntimeLiteral, RuntimeError> {
    let literal = match tree {
        Expression::Grouping(grouped_expression) => interpret_tree(*grouped_expression),
        Expression::Literal(literal) => match literal {
            ExpressionLiteral::Number(number) => todo!(),
            ExpressionLiteral::String(str) => todo!(),
            ExpressionLiteral::True => todo!(),
            ExpressionLiteral::False => todo!(),
            ExpressionLiteral::Nil => todo!(),
        },
        Expression::Operation(operation) => match operation {
            Operation::Negate(UnaryOperation {
                operand,
                line_number,
            }) => match *operand {
                Expression::Literal(literal) => match literal {
                    ExpressionLiteral::Number(_) => todo!(),
                    ExpressionLiteral::Nil => Err(RuntimeError {
                        line_number,
                        message: "Tried to Negate Nil value".to_string(),
                    }),
                    literal => Err(RuntimeError {
                        line_number,
                        message: format!("Tried to Negate invalid literal: {}", literal),
                    }),
                },
                _ => 
            },
            Operation::Not(UnaryOperation {
                operand,
                line_number,
            }) => todo!(),
            Operation::Equal(EqualityOperation {
                left,
                right,
                line_number,
            }) => todo!(),
            Operation::NotEqual(EqualityOperation {
                left,
                right,
                line_number,
            }) => todo!(),
            Operation::Less(ComparisonOperation {
                left,
                right,
                line_number,
            }) => todo!(),
            Operation::LessEqual(ComparisonOperation {
                left,
                right,
                line_number,
            }) => todo!(),
            Operation::Greater(ComparisonOperation {
                left,
                right,
                line_number,
            }) => todo!(),
            Operation::GreaterEqual(ComparisonOperation {
                left,
                right,
                line_number,
            }) => todo!(),
            Operation::Plus(TermOperation {
                left,
                right,
                line_number,
            }) => todo!(),
            Operation::Minus(TermOperation {
                left,
                right,
                line_number,
            }) => todo!(),
            Operation::Multiply(FactorOperation {
                left,
                right,
                line_number,
            }) => todo!(),
            Operation::Divide(FactorOperation {
                left,
                right,
                line_number,
            }) => todo!(),
        },
    };

    return Ok(literal);
}

pub fn is_truthy(literal: ExpressionLiteral) -> bool {
    match literal {
        ExpressionLiteral::Number(number) => number != 0.0,
        ExpressionLiteral::String(str) => str.len() > 0,
        ExpressionLiteral::True => true,
        ExpressionLiteral::False => false,
        ExpressionLiteral::Nil => false,
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum RuntimeLiteral {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
}

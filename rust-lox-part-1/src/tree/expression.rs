use core::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Literal(ExpressionLiteral),
    Operation(Operation),
    Grouping(Box<Expression>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExpressionLiteral {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
}

#[derive(Debug, PartialEq, Clone)]
pub struct EqualityOperation {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ComparisonOperation {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FactorOperation {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TermOperation {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnaryOperation {
    pub operand: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operation {
    Negate(UnaryOperation),
    Not(UnaryOperation),

    Equal(EqualityOperation),
    NotEqual(EqualityOperation),
    Less(ComparisonOperation),
    LessEqual(ComparisonOperation),
    Greater(ComparisonOperation),
    GreaterEqual(ComparisonOperation),

    Plus(TermOperation),
    Minus(TermOperation),
    Multiply(FactorOperation),
    Divide(FactorOperation),
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Expression::Literal(literal) => write!(f, "{}", *literal),
            Expression::Operation(operation) => write!(f, "{}", *operation),
            Expression::Grouping(expression) => write!(f, "( {} )", *expression),
        }
    }
}

impl Display for ExpressionLiteral {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ExpressionLiteral::Number(number) => write!(f, "{:.2}", number),
            ExpressionLiteral::String(string_literal) => write!(f, "\"{}\"", string_literal),
            ExpressionLiteral::True => write!(f, "( true )"),
            ExpressionLiteral::False => write!(f, "( false )"),
            ExpressionLiteral::Nil => write!(f, "( nil )"),
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Operation::Negate(unary_operation) => {
                write!(f, "( - {} )", (*unary_operation.operand))
            }
            Operation::Not(unary_operation) => {
                write!(f, "( not {} )", (*unary_operation.operand))
            }
            Operation::Equal(binary_operation) => {
                write!(
                    f,
                    "( == {} {} )",
                    *binary_operation.left, *binary_operation.right
                )
            }
            Operation::NotEqual(binary_operation) => {
                write!(
                    f,
                    "( != {} {} )",
                    *binary_operation.left, *binary_operation.right
                )
            }
            Operation::Less(binary_operation) => {
                write!(
                    f,
                    "( < {} {} )",
                    *binary_operation.left, *binary_operation.right
                )
            }
            Operation::LessEqual(binary_operation) => {
                write!(
                    f,
                    "( <= {} {} )",
                    *binary_operation.left, *binary_operation.right
                )
            }
            Operation::Greater(binary_operation) => {
                write!(
                    f,
                    "( > {} {} )",
                    *binary_operation.left, *binary_operation.right
                )
            }
            Operation::GreaterEqual(binary_operation) => {
                write!(
                    f,
                    "( >= {} {} )",
                    *binary_operation.left, *binary_operation.right
                )
            }
            Operation::Plus(binary_operation) => {
                write!(
                    f,
                    "( + {} {} )",
                    *binary_operation.left, *binary_operation.right
                )
            }
            Operation::Minus(binary_operation) => {
                write!(
                    f,
                    "( - {} {} )",
                    *binary_operation.left, *binary_operation.right
                )
            }
            Operation::Multiply(binary_operation) => {
                write!(
                    f,
                    "( * {} {} )",
                    *binary_operation.left, *binary_operation.right
                )
            }
            Operation::Divide(binary_operation) => {
                write!(
                    f,
                    "( / {} {} )",
                    *binary_operation.left, *binary_operation.right
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tree::expression::{
        EqualityOperation, Expression, ExpressionLiteral, FactorOperation, Operation,
        TermOperation, UnaryOperation,
    };

    #[test]
    fn number_literal_can_be_pretty_printed() {
        let expression = Expression::Literal(ExpressionLiteral::Number(420.6969));
        assert_eq!(expression.to_string(), "420.70");
    }

    #[test]
    fn string_literal_can_be_pretty_printed() {
        let expression = Expression::Literal(ExpressionLiteral::String("Test String".to_string()));
        assert_eq!(expression.to_string(), "\"Test String\"");
    }

    #[test]
    fn simple_operation_can_be_pretty_printed() {
        let expression = Expression::Operation(Operation::Plus(TermOperation {
            left: Box::from(Expression::Literal(ExpressionLiteral::Number(
                10020030.3456,
            ))),
            right: Box::from(Expression::Literal(ExpressionLiteral::Number(5.2))),
        }));

        assert_eq!(expression.to_string(), "( + 10020030.35 5.20 )");
    }

    #[test]
    fn deeply_nested_expression_can_be_pretty_printed() {
        let expression = Expression::Operation(Operation::Multiply(FactorOperation {
            left: Box::new(Expression::Operation(Operation::Divide(FactorOperation {
                left: Box::new(Expression::Literal(ExpressionLiteral::Number(
                    10020030.3456,
                ))),
                right: Box::new(Expression::Operation(Operation::Plus(TermOperation {
                    left: Box::new(Expression::Literal(ExpressionLiteral::True)),
                    right: Box::new(Expression::Operation(Operation::Negate(UnaryOperation {
                        operand: Box::new(Expression::Literal(ExpressionLiteral::Number(
                            120341.2332,
                        ))),
                    }))),
                }))),
            }))),

            right: Box::new(Expression::Operation(Operation::Equal(EqualityOperation {
                left: Box::new(Expression::Literal(ExpressionLiteral::True)),
                right: Box::new(Expression::Operation(Operation::Negate(UnaryOperation {
                    operand: Box::new(Expression::Literal(ExpressionLiteral::Number(120341.2332))),
                }))),
            }))),
        }));

        assert_eq!(
            expression.to_string(),
            "( * ( / 10020030.35 ( + ( true ) ( - 120341.23 ) ) ) ( == ( true ) ( - 120341.23 ) ) )"
        );
    }
}

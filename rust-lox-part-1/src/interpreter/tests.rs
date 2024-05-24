#![cfg(test)]

use crate::{
    interpreter::is_truthy,
    tree::expression::{EqualityOperation, Expression, ExpressionLiteral, Operation},
};

use super::interpret_tree;

#[test]
fn test_equality_operation() {
    let expr: Expression = Expression::Operation(Operation::Equal(EqualityOperation {
        left: Box::new(Expression::Literal(ExpressionLiteral::True)),
        right: Box::new(Expression::Literal(ExpressionLiteral::True)),
        line_number: 0,
    }));

    let result = interpret_tree(expr);

    assert!(result.is_ok());

    assert_eq!(result.unwrap(), ExpressionLiteral::True)
}

#[test]
fn test_inequality_operation() {
    let expr: Expression = Expression::Operation(Operation::NotEqual(EqualityOperation {
        left: Box::new(Expression::Literal(ExpressionLiteral::True)),
        right: Box::new(Expression::Literal(ExpressionLiteral::True)),
        line_number: 0,
    }));

    let result = interpret_tree(expr);

    assert!(result.is_ok());

    assert_eq!(result.unwrap(), ExpressionLiteral::False)
}

#[test]
fn test_invalid_equality_operation() {
    let expr: Expression = Expression::Operation(Operation::Equal(EqualityOperation {
        left: Box::new(Expression::Literal(ExpressionLiteral::Number(10.0))),
        right: Box::new(Expression::Literal(ExpressionLiteral::True)),
        line_number: 0,
    }));

    let result = interpret_tree(expr);

    assert!(result.is_err());
}

#[test]
fn test_invalid_inequality_operation() {
    let expr: Expression = Expression::Operation(Operation::NotEqual(EqualityOperation {
        left: Box::new(Expression::Literal(ExpressionLiteral::Number(10.0))),
        right: Box::new(Expression::Literal(ExpressionLiteral::True)),
        line_number: 0,
    }));

    let result = interpret_tree(expr);

    assert!(result.is_err());
}

#[test]
fn test_is_number_truthy() {
    let expr = Expression::Literal(ExpressionLiteral::Number(0.1));

    let result = is_truthy(expr);

    assert_eq!(result, Ok(true));
}

#[test]
fn test_is_zero_falsy() {
    let expr = Expression::Literal(ExpressionLiteral::Number(0.0));

    let result = is_truthy(expr);

    assert_eq!(result, Ok(false));
}

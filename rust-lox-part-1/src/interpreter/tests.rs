#![cfg(test)]

use crate::{
    interpreter::is_truthy,
    tree::expression::{
        EqualityOperation, Expression, ExpressionLiteral, FactorOperation, Operation, TermOperation,
    },
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

#[test]
fn test_is_string_truthy() {
    let expr = Expression::Literal(ExpressionLiteral::String(
        "This string should be truthy!".to_owned(),
    ));

    let result = is_truthy(expr);

    assert_eq!(result, Ok(true));
}

#[test]
fn test_is_empty_string_falsy() {
    let expr = Expression::Literal(ExpressionLiteral::String("".to_owned()));

    let result = is_truthy(expr);

    assert_eq!(result, Ok(false));
}

#[test]
fn test_plus_operation() {
    let expr: Expression = Expression::Operation(Operation::Plus(TermOperation {
        left: Box::new(Expression::Literal(ExpressionLiteral::Number(0.1))),
        right: Box::new(Expression::Literal(ExpressionLiteral::Number(0.2))),
        line_number: 0,
    }));

    let result = interpret_tree(expr);

    assert!(result.is_ok());

    assert_eq!(result.unwrap(), ExpressionLiteral::Number(0.1 + 0.2))
}

#[test]
fn test_multiply_operation() {
    let expr: Expression = Expression::Operation(Operation::Multiply(FactorOperation {
        left: Box::new(Expression::Literal(ExpressionLiteral::Number(100.0))),
        right: Box::new(Expression::Literal(ExpressionLiteral::Number(5.0))),
        line_number: 0,
    }));

    let result = interpret_tree(expr);

    assert!(result.is_ok());

    assert_eq!(result.unwrap(), ExpressionLiteral::Number(500.0))
}

// TODO: Test division
#![cfg(test)]
use crate::tree::expression::{
    EqualityOperation, Expression, ExpressionLiteral, ExpressionVariable, FactorOperation,
    Operation, TermOperation, UnaryOperation,
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
fn variable_reference_can_be_pretty_printed() {
    let expression = Expression::Variable(ExpressionVariable {
        line_number: 0,
        identifier_name: "epic".to_string(),
    });
    assert_eq!(expression.to_string(), "( *epic )");
}

#[test]
fn simple_operation_can_be_pretty_printed() {
    let expression = Expression::Operation(Operation::Plus(TermOperation {
        left: Box::from(Expression::Literal(ExpressionLiteral::Number(
            10020030.3456,
        ))),
        right: Box::from(Expression::Literal(ExpressionLiteral::Number(5.2))),
        line_number: 0,
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
                    operand: Box::new(Expression::Literal(ExpressionLiteral::Number(120341.2332))),
                    line_number: 1,
                }))),
                line_number: 1,
            }))),

            line_number: 0,
        }))),

        right: Box::new(Expression::Operation(Operation::Equal(EqualityOperation {
            left: Box::new(Expression::Literal(ExpressionLiteral::True)),
            right: Box::new(Expression::Operation(Operation::Negate(UnaryOperation {
                operand: Box::new(Expression::Literal(ExpressionLiteral::Number(120341.2332))),
                line_number: 0,
            }))),
            line_number: 0,
        }))),

        line_number: 0,
    }));

    assert_eq!(
        expression.to_string(),
        "( * ( / 10020030.35 ( + true ( - 120341.23 ) ) ) ( == true ( - 120341.23 ) ) )"
    );
}

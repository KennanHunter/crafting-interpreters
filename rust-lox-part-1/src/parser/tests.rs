#![cfg(test)]

use crate::{
    parser::{comparison, equality, expression, term, ParsingResult},
    scanner::scan_tokens,
    tokens::{Token, TokenType},
    tree::expression::{
        ComparisonOperation, EqualityOperation, Expression, ExpressionLiteral, FactorOperation,
        Operation, TermOperation, UnaryOperation,
    },
};

use super::{factor, primary, unary, TokenIter};

#[test]
fn test_primary_parse_false_token() {
    let tokens_vec = vec![Token {
        token_type: TokenType::False,
        lexeme: "".to_string(),
        line_number: 1,
    }];

    let mut tokens: TokenIter = tokens_vec.iter().peekable();

    let result = primary(&mut tokens);

    assert_eq!(result, Ok(Expression::Literal(ExpressionLiteral::False)))
}

#[test]
fn test_primary_parse_string_token() {
    let tokens_vec = vec![Token {
        token_type: TokenType::String("Inside string".to_string()),
        lexeme: "".to_string(),
        line_number: 1,
    }];

    let mut tokens: TokenIter = tokens_vec.iter().peekable();

    let result = primary(&mut tokens);

    assert_eq!(
        result,
        Ok(Expression::Literal(ExpressionLiteral::String(
            "Inside string".to_string()
        )))
    )
}

#[test]
fn test_primary_parse_number_token() {
    let tokens_vec = vec![Token {
        token_type: TokenType::Number(10.0),
        lexeme: "".to_string(),
        line_number: 1,
    }];

    let mut tokens: TokenIter = tokens_vec.iter().peekable();

    let result: ParsingResult = primary(&mut tokens);

    assert_eq!(
        result,
        Ok(Expression::Literal(ExpressionLiteral::Number(10.0)))
    )
}

#[test]
fn test_unary_parse_simple_bang() {
    let tokens_vec = vec![
        Token {
            token_type: TokenType::Bang,
            lexeme: "".to_string(),
            line_number: 1,
        },
        Token {
            token_type: TokenType::True,
            lexeme: "".to_string(),
            line_number: 1,
        },
    ];

    let mut tokens: TokenIter = tokens_vec.iter().peekable();

    let result: ParsingResult = unary(&mut tokens);

    assert_eq!(
        result,
        Ok(Expression::Operation(Operation::Not(UnaryOperation {
            operand: (Box::new(Expression::Literal(ExpressionLiteral::True))),
            line_number: 1
        })))
    );
}

#[test]
fn test_unary_parse_multiple_bang() {
    let tokens_vec = vec![
        Token {
            token_type: TokenType::Bang,
            lexeme: "".to_string(),
            line_number: 1,
        },
        Token {
            token_type: TokenType::Bang,
            lexeme: "".to_string(),
            line_number: 1,
        },
        Token {
            token_type: TokenType::True,
            lexeme: "".to_string(),
            line_number: 1,
        },
    ];

    let mut tokens: TokenIter = tokens_vec.iter().peekable();

    let result: ParsingResult = unary(&mut tokens);

    assert_eq!(
        result,
        Ok(Expression::Operation(Operation::Not(UnaryOperation {
            operand: Box::new(Expression::Operation(Operation::Not(UnaryOperation {
                operand: (Box::new(Expression::Literal(ExpressionLiteral::True))),
                line_number: 1
            }))),
            line_number: 1
        })))
    );
}

#[test]
fn test_factor_parse_simple_multiplication() {
    let tokens_vec = vec![
        Token {
            token_type: TokenType::Number(10.0),
            lexeme: "".to_string(),
            line_number: 1,
        },
        Token {
            token_type: TokenType::Star,
            lexeme: "".to_string(),
            line_number: 1,
        },
        Token {
            token_type: TokenType::Number(4.0),
            lexeme: "".to_string(),
            line_number: 1,
        },
    ];

    let mut tokens: TokenIter = tokens_vec.iter().peekable();

    let result: ParsingResult = factor(&mut tokens);

    assert_eq!(
        result,
        Ok(Expression::Operation(Operation::Multiply(
            FactorOperation {
                left: (Box::new(Expression::Literal(ExpressionLiteral::Number(10.0)))),
                right: (Box::new(Expression::Literal(ExpressionLiteral::Number(4.0)))),
                line_number: 1
            }
        )))
    );
}

#[test]
fn test_factor_parse_multiple_multiplication() {
    let tokens = scan_tokens("10 * 4 * 3".to_string()).unwrap();

    let result: ParsingResult = factor(&mut tokens.iter().peekable());

    assert_eq!(
        result,
        Ok(Expression::Operation(Operation::Multiply(
            FactorOperation {
                left: (Box::new(Expression::Literal(ExpressionLiteral::Number(10.0)))),
                right: (Box::new(Expression::Operation(Operation::Multiply(
                    FactorOperation {
                        left: (Box::new(Expression::Literal(ExpressionLiteral::Number(4.0)))),
                        right: (Box::new(Expression::Literal(ExpressionLiteral::Number(3.0)))),
                        line_number: 1
                    }
                )))),
                line_number: 1
            }
        )))
    );
}

#[test]
fn test_term_parse_simple_addition() {
    let tokens = scan_tokens("10 + 4".to_string()).unwrap();

    let result: ParsingResult = term(&mut tokens.iter().peekable());

    assert_eq!(
        result,
        Ok(Expression::Operation(Operation::Plus(TermOperation {
            left: (Box::new(Expression::Literal(ExpressionLiteral::Number(10.0)))),
            right: (Box::new(Expression::Literal(ExpressionLiteral::Number(4.0)))),
            line_number: 1
        })))
    );
}

#[test]
fn test_term_parse_nested_addition() {
    let tokens = scan_tokens("10 * 4 + 3".to_string()).unwrap();

    let result: ParsingResult = term(&mut tokens.iter().peekable());

    assert_eq!(
        result,
        Ok(Expression::Operation(Operation::Plus(TermOperation {
            left: (Box::new(Expression::Operation(Operation::Multiply(
                FactorOperation {
                    left: Box::new(Expression::Literal(ExpressionLiteral::Number(10.0))),
                    right: (Box::new(Expression::Literal(ExpressionLiteral::Number(4.0)))),
                    line_number: 1
                }
            )))),
            right: (Box::new(Expression::Literal(ExpressionLiteral::Number(3.0)))),
            line_number: 1
        })))
    );
}

#[test]
fn test_comparison_parse_nested_comparison() {
    let tokens = scan_tokens("5 > 4 > 3 + 2".to_string()).unwrap();

    let result: ParsingResult = comparison(&mut tokens.iter().peekable());

    assert_eq!(
        result,
        Ok(Expression::Operation(Operation::Greater(
            ComparisonOperation {
                left: (Box::new(Expression::Literal(ExpressionLiteral::Number(5.0)))),
                right: (Box::new(Expression::Operation(Operation::Greater(
                    ComparisonOperation {
                        left: Box::new(Expression::Literal(ExpressionLiteral::Number(4.0))),
                        right: Box::new(Expression::Operation(Operation::Plus(TermOperation {
                            left: Box::new(Expression::Literal(ExpressionLiteral::Number(3.0))),
                            right: Box::new(Expression::Literal(ExpressionLiteral::Number(2.0))),
                            line_number: 1
                        }))),
                        line_number: 1
                    }
                )))),
                line_number: 1
            }
        )))
    );
}

#[test]
fn test_equality_parse_simple_equality() {
    let tokens = scan_tokens("true == false".to_string()).unwrap();

    let result: ParsingResult = equality(&mut tokens.iter().peekable());

    assert_eq!(
        result,
        Ok(Expression::Operation(Operation::Equal(EqualityOperation {
            left: (Box::new(Expression::Literal(ExpressionLiteral::True))),
            right: (Box::new(Expression::Literal(ExpressionLiteral::False))),
            line_number: 1
        })))
    );
}

#[test]
fn test_equality_parse_nested_equality() {
    let tokens = scan_tokens("4 * 3 > 4 + 3 == 2 / 4 < 3 / 4".to_string()).unwrap();

    let result: ParsingResult = equality(&mut tokens.iter().peekable());

    assert_eq!(
        result,
        Ok(Expression::Operation(Operation::Equal(EqualityOperation {
            left: Box::new(Expression::Operation(Operation::Greater(
                ComparisonOperation {
                    left: Box::new(Expression::Operation(Operation::Multiply(
                        FactorOperation {
                            left: Box::new(Expression::Literal(ExpressionLiteral::Number(4.0))),
                            right: Box::new(Expression::Literal(ExpressionLiteral::Number(3.0))),
                            line_number: 1
                        },
                    ))),
                    right: Box::new(Expression::Operation(Operation::Plus(TermOperation {
                        left: Box::new(Expression::Literal(ExpressionLiteral::Number(4.0))),
                        right: Box::new(Expression::Literal(ExpressionLiteral::Number(3.0))),
                        line_number: 1
                    }))),
                    line_number: 1
                }
            ))),
            right: Box::new(Expression::Operation(Operation::Less(
                ComparisonOperation {
                    left: Box::new(Expression::Operation(Operation::Divide(FactorOperation {
                        left: Box::new(Expression::Literal(ExpressionLiteral::Number(2.0))),
                        right: Box::new(Expression::Literal(ExpressionLiteral::Number(4.0))),
                        line_number: 1
                    }))),
                    right: Box::new(Expression::Operation(Operation::Divide(FactorOperation {
                        left: Box::new(Expression::Literal(ExpressionLiteral::Number(3.0))),
                        right: Box::new(Expression::Literal(ExpressionLiteral::Number(4.0))),
                        line_number: 1
                    }))),
                    line_number: 1
                }
            ))),
            line_number: 1
        })))
    );
}

#[test]
fn test_expression_parse_grouped_expression() {
    let tokens = scan_tokens("(4 + 3) * 2".to_string()).unwrap();

    let result: ParsingResult = expression(&mut tokens.iter().peekable());

    assert_eq!(
        result,
        Ok(Expression::Operation(Operation::Multiply(
            FactorOperation {
                left: Box::new(Expression::Grouping(Box::new(Expression::Operation(
                    Operation::Plus(TermOperation {
                        left: Box::new(Expression::Literal(ExpressionLiteral::Number(4.0))),
                        right: Box::new(Expression::Literal(ExpressionLiteral::Number(3.0))),
                        line_number: 1
                    })
                )))),
                right: Box::new(Expression::Literal(ExpressionLiteral::Number(2.0))),
                line_number: 1
            }
        )))
    );
}

#![cfg(test)]

use crate::{
    parser::ParsingResult,
    tokens::{Token, TokenType},
    tree::expression::{Expression, ExpressionLiteral, FactorOperation, Operation, UnaryOperation},
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
            })))
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
            }
        )))
    );
}

#[test]
fn test_factor_parse_multiple_multiplication() {
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
        Token {
            token_type: TokenType::Star,
            lexeme: "".to_string(),
            line_number: 1,
        },
        Token {
            token_type: TokenType::Number(3.0),
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
                right: (Box::new(Expression::Operation(Operation::Multiply(
                    FactorOperation {
                        left: (Box::new(Expression::Literal(ExpressionLiteral::Number(4.0)))),
                        right: (Box::new(Expression::Literal(ExpressionLiteral::Number(3.0)))),
                    }
                )))),
            }
        )))
    );
}

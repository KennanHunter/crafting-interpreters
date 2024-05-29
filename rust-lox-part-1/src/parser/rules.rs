use statements::variable_statement;

use crate::errors::ParsingError;
use crate::tokens::TokenType;
use crate::tree::expression::{
    ComparisonOperation, EqualityOperation, Expression, ExpressionLiteral, ExpressionVariable,
    FactorOperation, Operation, TermOperation, UnaryOperation,
};

use self::statements::print_statement;

use super::util::consume_expected_character;
use super::{statements, ExpressionParsingResult, ParsedBlock, ParsingResult, TokenIter};

pub fn declaration(tokens: &mut TokenIter) -> ParsingResult {
    let token = tokens.peek();

    match token.unwrap().token_type {
        TokenType::Let => variable_statement(tokens),
        _ => statement(tokens),
    }
}

pub fn statement(tokens: &mut TokenIter) -> ParsingResult {
    let token = tokens.peek();

    match token.unwrap().token_type {
        TokenType::Print => print_statement(tokens),

        _ => {
            let expr = expression(tokens)?;

            consume_expected_character(tokens, TokenType::Semicolon)?;

            Ok(ParsedBlock::Expression(expr))
        }
    }
}

pub fn expression(tokens: &mut TokenIter) -> ExpressionParsingResult {
    assignment(tokens)
}

pub fn assignment(tokens: &mut TokenIter) -> ExpressionParsingResult {
    let left_side = equality(tokens)?;

    match tokens.peek().copied() {
        Some(token) if token.token_type == TokenType::Equal => {
            // consume equals
            tokens.next();

            let right_side = assignment(tokens)?;

            match left_side {
                Expression::Variable(expression_variable) => Ok(Expression::Assign(
                    expression_variable,
                    Box::new(right_side),
                )),
                _ => Err(ParsingError {
                    line_number: token.line_number,
                    message: format!("expected left side of = operator to be identifier"),
                }),
            }
        }
        _ => Ok(left_side),
    }
}

pub fn equality(tokens: &mut TokenIter) -> ExpressionParsingResult {
    let mut expression = comparison(tokens)?;

    let new_equality_operation = |expression: Expression,
                                  line_number: usize,
                                  tokens: &mut TokenIter|
     -> Result<EqualityOperation, ParsingError> {
        Ok(EqualityOperation {
            left: Box::new(expression),
            right: Box::new(equality(tokens)?),
            line_number,
        })
    };

    loop {
        // Look at the next token, if it is a equality
        match tokens.peek() {
            Some(&next_token) if next_token.token_type == TokenType::BangEqual => {
                tokens.next();

                expression = Expression::Operation(Operation::NotEqual(new_equality_operation(
                    expression,
                    next_token.line_number,
                    tokens,
                )?))
            }

            Some(&next_token) if next_token.token_type == TokenType::EqualEqual => {
                tokens.next();
                expression = Expression::Operation(Operation::Equal(new_equality_operation(
                    expression,
                    next_token.line_number,
                    tokens,
                )?))
            }
            _ => break,
        }
    }

    Ok(expression)
}

pub fn comparison(tokens: &mut TokenIter) -> ExpressionParsingResult {
    let mut expression = term(tokens)?;

    let new_comparison_operation = |expression: Expression,
                                    line_number: usize,
                                    tokens: &mut TokenIter|
     -> Result<ComparisonOperation, ParsingError> {
        Ok(ComparisonOperation {
            left: Box::new(expression),
            right: Box::new(comparison(tokens)?),
            line_number,
        })
    };

    loop {
        // Look at the next token, if it is a equality
        expression = match tokens.peek() {
            Some(&next_token) if next_token.token_type == TokenType::Greater => {
                tokens.next();

                Expression::Operation(Operation::Greater(new_comparison_operation(
                    expression,
                    next_token.line_number,
                    tokens,
                )?))
            }

            Some(&next_token) if next_token.token_type == TokenType::GreaterEqual => {
                tokens.next();

                Expression::Operation(Operation::GreaterEqual(new_comparison_operation(
                    expression,
                    next_token.line_number,
                    tokens,
                )?))
            }

            Some(&next_token) if next_token.token_type == TokenType::Less => {
                tokens.next();

                Expression::Operation(Operation::Less(new_comparison_operation(
                    expression,
                    next_token.line_number,
                    tokens,
                )?))
            }

            Some(&next_token) if next_token.token_type == TokenType::LessEqual => {
                tokens.next();

                Expression::Operation(Operation::LessEqual(new_comparison_operation(
                    expression,
                    next_token.line_number,
                    tokens,
                )?))
            }

            _ => break,
        }
    }

    Ok(expression)
}

pub fn term(tokens: &mut TokenIter) -> ExpressionParsingResult {
    let mut expression = factor(tokens)?;

    let new_term_operation = |expression: Expression,
                              line_number,
                              tokens: &mut TokenIter|
     -> Result<TermOperation, ParsingError> {
        Ok(TermOperation {
            left: Box::new(expression),
            right: Box::new(term(tokens)?),
            line_number,
        })
    };

    loop {
        // Look at the next token, if it is a equality
        expression = match tokens.peek() {
            Some(&next_token) if next_token.token_type == TokenType::Plus => {
                tokens.next();
                Expression::Operation(Operation::Plus(new_term_operation(
                    expression,
                    next_token.line_number,
                    tokens,
                )?))
            }
            Some(&next_token) if next_token.token_type == TokenType::Minus => {
                tokens.next();
                Expression::Operation(Operation::Minus(new_term_operation(
                    expression,
                    next_token.line_number,
                    tokens,
                )?))
            }
            _ => break,
        }
    }

    Ok(expression)
}

pub fn factor(tokens: &mut TokenIter) -> ExpressionParsingResult {
    let mut expression = unary(tokens)?;

    let new_factor_operation = |expression: Expression,
                                line_number: usize,
                                tokens: &mut TokenIter|
     -> Result<FactorOperation, ParsingError> {
        Ok(FactorOperation {
            left: Box::new(expression),
            right: Box::new(factor(tokens)?),
            line_number, // TODO:
        })
    };

    loop {
        // Look at the next token, if it is a equality
        expression = match tokens.peek() {
            Some(&next_token) if next_token.token_type == TokenType::Star => {
                tokens.next();
                Expression::Operation(Operation::Multiply(new_factor_operation(
                    expression,
                    next_token.line_number,
                    tokens,
                )?))
            }
            Some(&next_token) if next_token.token_type == TokenType::Slash => {
                tokens.next();
                Expression::Operation(Operation::Divide(new_factor_operation(
                    expression,
                    next_token.line_number,
                    tokens,
                )?))
            }
            _ => break,
        };
    }

    Ok(expression)
}

pub fn unary(tokens: &mut TokenIter) -> ExpressionParsingResult {
    match tokens.peek() {
        Some(&next_token) if next_token.token_type == TokenType::Bang => {
            tokens.next();

            Ok(Expression::Operation(Operation::Not(UnaryOperation {
                operand: Box::new(unary(tokens)?),
                line_number: next_token.line_number,
            })))
        }
        _ => Ok(primary(tokens)?),
    }
}

pub fn primary(tokens: &mut TokenIter) -> ExpressionParsingResult {
    let token = tokens.next().unwrap();

    match &token.token_type {
        TokenType::True => Ok(Expression::Literal(ExpressionLiteral::True)),
        TokenType::False => Ok(Expression::Literal(ExpressionLiteral::False)),
        TokenType::Nil => Ok(Expression::Literal(ExpressionLiteral::Nil)),

        // TODO There's probably better ways of passing this string literal without cloning it
        TokenType::String(str) => Ok(Expression::Literal(ExpressionLiteral::String(str.clone()))),

        TokenType::Number(number) => Ok(Expression::Literal(ExpressionLiteral::Number(*number))),

        TokenType::LeftParen => {
            let expr = expression(tokens)?;

            consume_expected_character(tokens, TokenType::RightParen)?;

            Ok(Expression::Grouping(Box::from(expr)))
        }

        TokenType::Identifier(identifier_name) => Ok(Expression::Variable(ExpressionVariable {
            line_number: token.line_number,
            identifier_name: identifier_name.clone(),
        })),

        unrecognized_type => Err(ParsingError {
            line_number: token.line_number,
            message: format!("Unrecognized token: \"{:?}\"", *unrecognized_type).to_string(),
        }),
    }
}

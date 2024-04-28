pub mod tests;

use crate::errors::ParsingError;
use crate::tokens::{Token, TokenType};
use crate::tree::expression::{
    ComparisonOperation, EqualityOperation, Expression, ExpressionLiteral, FactorOperation,
    Operation, UnaryOperation,
};

type TokenIter<'a> = std::iter::Peekable<std::slice::Iter<'a, Token>>;
type ParsingResult = Result<Expression, ParsingError>;

pub fn parse_tokens(tokens_vec: Vec<Token>) -> ParsingResult {
    let mut tokens: TokenIter = tokens_vec.iter().peekable();

    loop {
        let token = match tokens.peek() {
            Some(token) => *token,
            None => break,
        };

        if token.token_type == TokenType::EOF {
            break;
        }

        expression(&mut tokens)?;
    }

    Err(ParsingError {
        line_number: 0,
        message: "".to_string(),
    })
}

fn expression(tokens: &mut TokenIter) -> ParsingResult {
    equality(tokens)
}

fn equality(tokens: &mut TokenIter) -> ParsingResult {
    let mut expression = comparison(tokens)?;

    let new_equality_operation = |expression: Expression,
                                  tokens: &mut TokenIter|
     -> Result<EqualityOperation, ParsingError> {
        Ok(EqualityOperation {
            left: Box::new(expression),
            right: Box::new(term(tokens)?),
        })
    };

    loop {
        // Look at the next token, if it is a equality
        match tokens.peek() {
            Some(next_token) if next_token.token_type == TokenType::BangEqual => {
                expression = Expression::Operation(Operation::NotEqual(new_equality_operation(
                    expression, tokens,
                )?))
            }
            Some(next_token) if next_token.token_type == TokenType::EqualEqual => {
                expression = Expression::Operation(Operation::Equal(new_equality_operation(
                    expression, tokens,
                )?))
            }
            _ => break,
        }
    }

    Ok(expression)
}

fn comparison(tokens: &mut TokenIter) -> ParsingResult {
    let mut expression = term(tokens)?;

    let new_comparison_operation = |expression: Expression,
                                    tokens: &mut TokenIter|
     -> Result<ComparisonOperation, ParsingError> {
        Ok(ComparisonOperation {
            left: Box::new(expression),
            right: Box::new(term(tokens)?),
        })
    };

    loop {
        // Look at the next token, if it is a equality
        expression = match tokens.peek() {
            Some(next_token) if next_token.token_type == TokenType::Greater => {
                Expression::Operation(Operation::Greater(new_comparison_operation(
                    expression, tokens,
                )?))
            }
            Some(next_token) if next_token.token_type == TokenType::GreaterEqual => {
                Expression::Operation(Operation::GreaterEqual(new_comparison_operation(
                    expression, tokens,
                )?))
            }
            Some(next_token) if next_token.token_type == TokenType::Less => Expression::Operation(
                Operation::Less(new_comparison_operation(expression, tokens)?),
            ),
            Some(next_token) if next_token.token_type == TokenType::LessEqual => {
                Expression::Operation(Operation::LessEqual(new_comparison_operation(
                    expression, tokens,
                )?))
            }
            _ => break,
        }
    }

    Ok(expression)
}

fn term(tokens: &mut TokenIter) -> ParsingResult {
    let mut expression = factor(tokens)?;

    let new_term_operation = |expression: Expression,
                              tokens: &mut TokenIter|
     -> Result<ComparisonOperation, ParsingError> {
        Ok(ComparisonOperation {
            left: Box::new(expression),
            right: Box::new(term(tokens)?),
        })
    };

    loop {
        // Look at the next token, if it is a equality
        expression = match tokens.peek() {
            Some(next_token) if next_token.token_type == TokenType::Plus => {
                Expression::Operation(Operation::Greater(new_term_operation(expression, tokens)?))
            }
            Some(next_token) if next_token.token_type == TokenType::Minus => Expression::Operation(
                Operation::GreaterEqual(new_term_operation(expression, tokens)?),
            ),
            _ => break,
        }
    }

    Ok(expression)
}

fn factor(tokens: &mut TokenIter) -> ParsingResult {
    let mut expression = unary(tokens)?;

    let new_term_operation =
        |expression: Expression, tokens: &mut TokenIter| -> Result<FactorOperation, ParsingError> {
            Ok(FactorOperation {
                left: Box::new(expression),
                right: Box::new(factor(tokens)?),
            })
        };

    loop {
        // Look at the next token, if it is a equality
        expression = match tokens.peek() {
            Some(next_token) if next_token.token_type == TokenType::Star => {
                tokens.next();
                Expression::Operation(Operation::Multiply(new_term_operation(expression, tokens)?))
            }
            Some(next_token) if next_token.token_type == TokenType::Slash => {
                tokens.next();
                Expression::Operation(Operation::Divide(new_term_operation(expression, tokens)?))
            }
            _ => break,
        };
    }

    Ok(expression)
}

fn unary(tokens: &mut TokenIter) -> ParsingResult {
    match tokens.peek() {
        Some(next_token) if next_token.token_type == TokenType::Bang => {
            tokens.next();

            Ok(Expression::Operation(Operation::Not(UnaryOperation {
                operand: Box::new(unary(tokens)?),
            })))
        }
        _ => Ok(primary(tokens)?),
    }
}

fn primary(tokens: &mut TokenIter) -> ParsingResult {
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

            // TODO: Consume right parenthesis

            Ok(Expression::Grouping(Box::from(expr)))
        }

        _ => Err(ParsingError {
            line_number: token.line_number,
            message: "Unrecognized token made it to primary".to_string(),
        }),
    }
}

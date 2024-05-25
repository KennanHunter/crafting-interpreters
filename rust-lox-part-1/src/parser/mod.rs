pub mod statements;
pub mod tests;

use crate::errors::ParsingError;
use crate::tokens::{Token, TokenType};
use crate::tree::expression::{
    ComparisonOperation, EqualityOperation, Expression, ExpressionLiteral, FactorOperation,
    Operation, TermOperation, UnaryOperation,
};

use self::statements::{print_statement, Statement};

type TokenIter<'a> = std::iter::Peekable<std::slice::Iter<'a, Token>>;
type ParsingResult = Result<ParsedBlock, ParsingError>;
type ExpressionParsingResult = Result<Expression, ParsingError>;

pub enum ParsedBlock {
    Expression(Expression),
    Statement(Statement),
}

pub fn parse_tokens(tokens_vec: Vec<Token>) -> ParsingResult {
    let mut tokens: TokenIter = tokens_vec.iter().peekable();

    let token = tokens.peek();

    match token.unwrap().token_type {
        TokenType::EOF => Err(ParsingError {
            line_number: 0,
            message: "EOF found at beginning of file".to_string(),
        }),

        TokenType::Print => print_statement(&mut tokens),

        _ => Ok(ParsedBlock::Expression(expression(&mut tokens)?)),
    }
}

fn expression(tokens: &mut TokenIter) -> ExpressionParsingResult {
    equality(tokens)
}

fn equality(tokens: &mut TokenIter) -> ExpressionParsingResult {
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

fn comparison(tokens: &mut TokenIter) -> ExpressionParsingResult {
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

fn term(tokens: &mut TokenIter) -> ExpressionParsingResult {
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

fn factor(tokens: &mut TokenIter) -> ExpressionParsingResult {
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

fn unary(tokens: &mut TokenIter) -> ExpressionParsingResult {
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

fn primary(tokens: &mut TokenIter) -> ExpressionParsingResult {
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

            match tokens.next() {
                Some(token) if token.token_type == TokenType::RightParen => {
                    Ok(Expression::Grouping(Box::from(expr)))
                }
                _ => Err(ParsingError {
                    line_number: token.line_number,
                    message: "Closing parenthesis expected".to_string(),
                }),
            }
        }

        unrecognized_type => Err(ParsingError {
            line_number: token.line_number,
            message: format!("Unrecognized token: \"{:?}\"", *unrecognized_type).to_string(),
        }),
    }
}

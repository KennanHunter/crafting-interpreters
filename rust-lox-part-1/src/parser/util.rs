use std::mem;

use crate::{
    errors::ParsingError,
    tokens::{Token, TokenType},
    tree::expression::Expression,
};

use super::{rules::expression, TokenIter};

pub fn consume_expected_character(
    tokens: &mut TokenIter,
    expected_token_type: TokenType,
) -> Result<Token, ParsingError> {
    match tokens.next() {
        Some(token)
            if mem::discriminant(&token.token_type) == mem::discriminant(&expected_token_type) =>
        {
            Ok(token.clone())
        }
        Some(unrecognized_token) => Err(ParsingError {
            message: format!(
                "Expected {:?}, found \"{:?}\"",
                expected_token_type, unrecognized_token.token_type
            ),
            line_number: unrecognized_token.line_number,
        }),
        None => Err(ParsingError {
            message: format!("Expected {:?}, didn't find character", expected_token_type),
            line_number: 0, // TODO: Find better way to get this line number
        }),
    }
}

pub fn parse_call_arguments(tokens: &mut TokenIter) -> Result<Vec<Expression>, ParsingError> {
    consume_expected_character(tokens, TokenType::LeftParen)?;

    let mut arguments: Vec<Expression> = vec![];

    if tokens
        .peek()
        .is_some_and(|token| token.token_type != TokenType::RightParen)
    {
        arguments.push(expression(tokens)?)
    }

    loop {
        match tokens.next() {
            Some(delimiter) if delimiter.token_type == TokenType::Comma => {
                arguments.push(expression(tokens)?);
            }
            Some(delimiter) if delimiter.token_type == TokenType::RightParen => {
                break;
            }
            Some(unrecognized) => {
                return Err(ParsingError {
                    line_number: unrecognized.line_number,
                    message: format!(
                        "Expected either comma or parenthesis in function arguments, found {:?}",
                        unrecognized.token_type
                    ),
                })
            }
            None => unimplemented!(),
        };
    }

    Ok(arguments)
}

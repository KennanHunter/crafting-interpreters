use crate::{
    errors::ParsingError,
    tokens::{Token, TokenType},
};

use super::TokenIter;

pub fn consume_expected_character(
    tokens: &mut TokenIter,
    expected_token_type: TokenType,
) -> Result<Token, ParsingError> {
    match tokens.next() {
        Some(token) if token.token_type == expected_token_type => Ok(token.clone()),
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

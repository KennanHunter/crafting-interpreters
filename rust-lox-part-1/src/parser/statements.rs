use crate::errors::ParsingError;
use crate::tree::expression::Expression;
use crate::{parser::expression, tokens::TokenType};

use super::{ParsedBlock, ParsingResult, TokenIter};

#[derive(Debug, Clone)]
pub enum Statement {
    Print(Expression),
}

pub fn print_statement(tokens: &mut TokenIter) -> ParsingResult {
    // consume "Print"
    tokens.next();

    let contained_expression = expression(tokens)?;

    match tokens.next() {
        Some(token) if token.token_type == TokenType::Semicolon => Ok(ParsedBlock::Statement(
            Statement::Print(contained_expression),
        )),
        Some(unrecognized_token) => Err(ParsingError {
            message: format!(
                "Expected semicolon, found \"{:?}\" following expression",
                unrecognized_token.token_type
            ),
            line_number: unrecognized_token.line_number,
        }),
        None => Err(ParsingError {
            message: format!("Exected semicolon, didn't find character"),
            line_number: 0, // TODO: Find better way to get this line number
        }),
    }
}

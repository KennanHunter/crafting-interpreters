use crate::errors::ParsingError;
use crate::tokens::TokenType;
use crate::tree::expression::Expression;

use super::{
    rules::expression, util::consume_expected_character, ParsedBlock, ParsingResult, TokenIter,
};

#[derive(Debug, Clone)]
pub enum Statement {
    Print(Expression),
    Variable(String, Expression),
}

pub fn print_statement(tokens: &mut TokenIter) -> ParsingResult {
    // consume "Print"
    tokens.next();

    let contained_expression = expression(tokens)?;

    consume_expected_character(tokens, TokenType::Semicolon)?;

    Ok(ParsedBlock::Statement(Statement::Print(
        contained_expression,
    )))
}

pub fn variable_statement(tokens: &mut TokenIter) -> ParsingResult {
    // consume "let"
    tokens.next();

    let identifier_token = tokens.next().unwrap();

    let identifier_name = match &identifier_token.token_type {
        TokenType::Identifier(token_identifier_name) => token_identifier_name,
        unrecognized_identifier => {
            return Err(ParsingError {
                line_number: identifier_token.line_number,
                message: format!(
                    "Expected identifier following \"let\", found {:?}",
                    unrecognized_identifier
                ),
            })
        }
    };

    consume_expected_character(tokens, TokenType::Equal)?;

    let value = expression(tokens)?;

    tokens
        .next_if(|value| value.token_type == TokenType::Semicolon)
        .expect("expected semicolon following variable declaration");

    Ok(ParsedBlock::Statement(Statement::Variable(
        identifier_name.clone(),
        value,
    )))
}

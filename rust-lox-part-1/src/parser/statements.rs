use crate::errors::ParsingError;
use crate::tree::expression::Expression;
use crate::{parser::expression, tokens::TokenType};

use super::{ParsedBlock, ParsingResult, TokenIter};

#[derive(Debug, Clone)]
pub enum Statement {
    Print(Expression),
    Variable(String, Expression),
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
            message: format!("Expected semicolon, didn't find character"),
            line_number: 0, // TODO: Find better way to get this line number
        }),
    }
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

    match tokens.next() {
        Some(token) if token.token_type == TokenType::Equal => {}
        Some(unrecognized_token) => {
            return Err(ParsingError {
                line_number: unrecognized_token.line_number,
                message: format!(
                    "Expected equals following identifier, found {:?}",
                    unrecognized_token
                ),
            })
        }
        None => {
            return Err(ParsingError {
                line_number: identifier_token.line_number,
                message: format!("Expected equals following identifier, found nothing"),
            })
        }
    };

    let value = expression(tokens)?;

    tokens
        .next_if(|value| value.token_type == TokenType::Semicolon)
        .expect("expected semicolon following variable declaration");

    Ok(ParsedBlock::Statement(Statement::Variable(
        identifier_name.clone(),
        value,
    )))
}

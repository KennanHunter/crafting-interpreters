use crate::errors::ParsingError;
use crate::tokens::TokenType;
use crate::tree::expression::Expression;

use super::{
    rules::{block, expression},
    util::consume_expected_character,
    ParsedStep, ParsingResult, TokenIter,
};

#[derive(Debug, Clone)]
pub enum Statement {
    Print(Expression),
    Variable(String, Expression),
    If(IfStatement),
    While(WhileStatement),
}

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub condition: Expression,
    pub then_statement: Box<ParsedStep>,
    pub else_statement: Option<Box<ParsedStep>>,
}

#[derive(Debug, Clone)]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: Box<ParsedStep>,
}

pub fn print_statement(tokens: &mut TokenIter) -> ParsingResult {
    // consume "Print"
    tokens.next();

    let contained_expression = expression(tokens)?;

    consume_expected_character(tokens, TokenType::Semicolon)?;

    Ok(ParsedStep::Statement(Statement::Print(
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

    Ok(ParsedStep::Statement(Statement::Variable(
        identifier_name.clone(),
        value,
    )))
}

pub fn if_statement(tokens: &mut TokenIter) -> ParsingResult {
    consume_expected_character(tokens, TokenType::If)?;

    let condition = expression(tokens)?;

    let then_statement = Box::new(block(tokens)?);

    let else_statement = if tokens
        .peek()
        .is_some_and(|token| token.token_type == TokenType::Else)
    {
        consume_expected_character(tokens, TokenType::Else)?;

        let else_statement = Box::new(block(tokens)?);

        Some(else_statement)
    } else {
        None
    };

    Ok(ParsedStep::Statement(Statement::If(IfStatement {
        condition,
        then_statement,
        else_statement,
    })))
}

pub fn while_statement(tokens: &mut TokenIter) -> ParsingResult {
    consume_expected_character(tokens, TokenType::While)?;

    let condition = expression(tokens)?;

    let body = Box::new(block(tokens)?);

    Ok(ParsedStep::Statement(Statement::While(WhileStatement {
        condition,
        body,
    })))
}

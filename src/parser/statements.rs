use std::vec;

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
    Fun(FunStatement),
    Return(Option<Expression>),
    Class(ClassStatement),
}

#[derive(Debug, Clone)]
pub struct FunStatement {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Box<ParsedStep>,
}

#[derive(Debug, Clone)]
pub struct ClassStatement {
    pub name: String,
    pub methods: Vec<FunStatement>,
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

pub fn function_declaration_statement(tokens: &mut TokenIter) -> ParsingResult {
    consume_expected_character(tokens, TokenType::Fun)?;

    let function_identifier = tokens.next().unwrap();
    let function_name = match &function_identifier.token_type {
        TokenType::Identifier(name) => name.clone(),
        unknown => {
            return Err(ParsingError {
                line_number: function_identifier.line_number,
                message: format!("Expected function name, found {:?}", unknown),
            })
        }
    };

    consume_expected_character(tokens, TokenType::LeftParen)?;

    let mut parameters: Vec<String> = vec![];

    loop {
        match &tokens.next().unwrap().token_type {
            TokenType::Identifier(parameter_name) => parameters.push(parameter_name.clone()),
            TokenType::RightParen => break,
            unknown => {
                return Err(ParsingError {
                    line_number: function_identifier.line_number,
                    message: format!("Expected function parameter, found {:?}", unknown),
                })
            }
        };

        match tokens.next() {
            Some(token) if token.token_type == TokenType::Comma => continue,
            Some(token) if token.token_type == TokenType::RightParen => break,
            Some(unrecognized_token) => {
                return Err(ParsingError {
                    line_number: unrecognized_token.line_number,
                    message: format!(
                        "Expected comma delimiting another parameter or closing parenthesis, found {:?}",
                        unrecognized_token.token_type
                    ),
                })
            }

            None => unimplemented!(),
        }
    }

    let body = Box::new(block(tokens)?);

    Ok(ParsedStep::Statement(Statement::Fun(FunStatement {
        name: function_name,
        parameters,
        body,
    })))
}

pub fn class_declaration_statement(tokens: &mut TokenIter) -> ParsingResult {
    consume_expected_character(tokens, TokenType::Class)?;

    let class_identifier = tokens.next().unwrap();
    let class_name = match &class_identifier.token_type {
        TokenType::Identifier(name) => name.clone(),
        unknown => {
            return Err(ParsingError {
                line_number: class_identifier.line_number,
                message: format!("Expected class name, found {:?}", unknown),
            })
        }
    };

    consume_expected_character(tokens, TokenType::LeftBrace)?;

    let mut methods: Vec<FunStatement> = Vec::new();

    loop {
        if tokens
            .peek()
            .is_some_and(|token| token.token_type == TokenType::RightBrace)
        {
            break;
        }

        match function_declaration_statement(tokens)? {
            ParsedStep::Statement(stmt) => match stmt {
                Statement::Fun(function) => methods.push(function),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    consume_expected_character(tokens, TokenType::RightBrace)?;

    Ok(ParsedStep::Statement(Statement::Class(ClassStatement {
        name: class_name,
        methods,
    })))
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

pub fn return_statement(tokens: &mut TokenIter) -> ParsingResult {
    consume_expected_character(tokens, TokenType::Return)?;

    if tokens
        .peek()
        .is_some_and(|token| token.token_type == TokenType::Semicolon)
    {
        consume_expected_character(tokens, TokenType::Semicolon)?;

        return Ok(ParsedStep::Statement(Statement::Return(None)));
    }

    let expr = expression(tokens)?;

    consume_expected_character(tokens, TokenType::Semicolon)?;

    Ok(ParsedStep::Statement(Statement::Return(Some(expr))))
}

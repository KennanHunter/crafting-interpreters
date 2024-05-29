use rules::declaration;
use statements::Statement;

use crate::{
    errors::ParsingError,
    tokens::{Token, TokenType},
    tree::expression::Expression,
};

pub mod rules;
pub mod statements;
pub mod tests;
mod util;

#[derive(Debug, Clone)]
pub enum ParsedStep {
    Expression(Expression),
    Statement(Statement),
    Block(Vec<ParsingResult>),
}

pub type TokenIter<'a> = std::iter::Peekable<std::slice::Iter<'a, Token>>;
pub type ParsingResult = Result<ParsedStep, ParsingError>;
type ExpressionParsingResult = Result<Expression, ParsingError>;

pub fn parse(tokens_vec: Vec<Token>) -> Vec<ParsingResult> {
    let mut tokens: TokenIter = tokens_vec.iter().peekable();

    top_parse_steps(&mut tokens)
}

fn top_parse_steps(tokens: &mut TokenIter) -> Vec<ParsingResult> {
    let mut return_vector: Vec<ParsingResult> = vec![];

    loop {
        return_vector.push(declaration(tokens));

        let token = tokens.peek();

        if token.is_none() || token.is_some_and(|token| token.token_type == TokenType::EOF) {
            break;
        }
    }

    return return_vector;
}

fn parse_steps(tokens: &mut TokenIter) -> Vec<ParsingResult> {
    let mut return_vector: Vec<ParsingResult> = vec![];

    loop {
        return_vector.push(declaration(tokens));

        let token = tokens.peek();

        if token.is_none()
            || token.is_some_and(|token| token.token_type == TokenType::EOF)
            || token.is_some_and(|token| token.token_type == TokenType::RightBrace)
        {
            break;
        }
    }

    return return_vector;
}

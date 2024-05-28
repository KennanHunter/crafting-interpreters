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
pub enum ParsedBlock {
    Expression(Expression),
    Statement(Statement),
}

pub type TokenIter<'a> = std::iter::Peekable<std::slice::Iter<'a, Token>>;
pub type ParsingResult = Result<ParsedBlock, ParsingError>;
type ExpressionParsingResult = Result<Expression, ParsingError>;

pub fn parse_blocks(tokens_vec: Vec<Token>) -> Vec<ParsingResult> {
    let mut tokens: TokenIter = tokens_vec.iter().peekable();

    let mut return_vector: Vec<ParsingResult> = vec![];

    loop {
        return_vector.push(declaration(&mut tokens));

        let token = tokens.peek();

        if token.is_none() || token.is_some_and(|token| token.token_type == TokenType::EOF) {
            break;
        }
    }

    return return_vector;
}

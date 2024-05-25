use crate::parser::expression;
use crate::tree::expression::Expression;

use super::{ParsedBlock, ParsingResult, TokenIter};

pub enum Statement {
    Print(Expression),
}

pub fn print_statement(tokens: &mut TokenIter) -> ParsingResult {
    Ok(ParsedBlock::Statement(Statement::Print(expression(
        tokens,
    )?)))
}

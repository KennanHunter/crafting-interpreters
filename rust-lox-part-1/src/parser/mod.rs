use crate::{
    errors::ParsingError,
    tokens::{Token, TokenType},
    tree::expression::{EqualityOperation, Expression, Operation},
};

type TokenIter<'a> = std::iter::Peekable<std::slice::Iter<'a, Token>>;
type ParsingResult = Result<Expression, ParsingError>;

pub fn parse_tokens(tokens_vec: Vec<Token>) -> Result<Expression, ParsingError> {
    let mut tokens: TokenIter = tokens_vec.iter().peekable();

    loop {
        let token = match tokens.peek() {
            Some(token) => *token,
            None => break,
        };

        if token.token_type == TokenType::EOF {
            break;
        }
    }

    Err(ParsingError {
        line_number: 0,
        message: "".to_string(),
    })
}

pub fn expression(tokens: &mut TokenIter) -> Result<Expression, ParsingError> {
    equality(tokens)
}

pub fn equality(tokens: &mut TokenIter) -> Result<Expression, ParsingError> {
    let mut expression = comparison(tokens)?;

    loop {
        // Look at the next token, if it is a equality
        match tokens.peek() {
            Some(next_token) if next_token.token_type == TokenType::BangEqual => {
                expression = Expression::Operation(Operation::NotEqual(EqualityOperation {
                    left: Box::new(expression),
                    right: Box::new(comparison(tokens)?),
                }))
            }
            Some(next_token) if next_token.token_type == TokenType::EqualEqual => {
                expression = Expression::Operation(Operation::Equal(EqualityOperation {
                    left: Box::new(expression),
                    right: Box::new(comparison(tokens)?),
                }))
            }
            _ => break,
        }
    }

    Ok(expression)
}

pub fn comparison(tokens: &mut TokenIter) -> Result<Expression, ParsingError> {
    let mut expression = term(tokens)?;

    let new_equality_operation = || -> Result<EqualityOperation, ParsingError> {
        Ok(EqualityOperation {
            left: Box::new(expression),
            right: Box::new(term(tokens)?),
        })
    };

    loop {
        // Look at the next token, if it is a equality
        expression = match tokens.peek() {
            Some(next_token) if next_token.token_type == TokenType::Greater => {
                Expression::Operation(Operation::NotEqual(new_equality_operation()?))
            }
            Some(next_token) if next_token.token_type == TokenType::EqualEqual => {
                Expression::Operation(Operation::Equal(new_equality_operation()?))
            }
            _ => break,
        }
    }

    Ok(expression)
}

pub fn term(tokens: &mut TokenIter) -> Result<Expression, ParsingError> {
    let mut expression = factor(tokens)?;

    loop {
        // Look at the next token, if it is a equality
        match tokens.peek() {
            Some(next_token) if next_token.token_type == TokenType::BangEqual => {
                expression = Expression::Operation(Operation::NotEqual(EqualityOperation {
                    left: Box::new(expression),
                    right: Box::new(factor(tokens)?),
                }))
            }
            Some(next_token) if next_token.token_type == TokenType::EqualEqual => {
                expression = Expression::Operation(Operation::Equal(EqualityOperation {
                    left: Box::new(expression),
                    right: Box::new(factor(tokens)?),
                }))
            }
            _ => break,
        }
    }

    Ok(expression)
}

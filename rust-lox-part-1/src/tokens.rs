use core::fmt;
use std::fmt::{Debug, Display, Formatter};

#[derive(PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line_number: usize,
    // TODO: Object literal
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} @ line={} : {:?}",
            self.token_type, self.line_number, self.lexeme
        )
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} @ line={} : {:?}",
            self.token_type, self.line_number, self.lexeme
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Mins,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier(String),
    String(String),
    Number(f64),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

#[cfg(test)]
mod tests {
    use crate::tokens::Token;

    #[test]
    fn token_can_be_displayed() {
        let token = Token {
            lexeme: "print".to_owned(),
            token_type: crate::tokens::TokenType::Print,
            line_number: 1,
        };

        assert_eq!(token.to_string(), "Print @ line=1 : \"print\"");
    }
}

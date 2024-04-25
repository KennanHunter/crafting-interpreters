use core::fmt;
use std::{
    collections::HashMap,
    fmt::{Debug, Display, Formatter},
};

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

impl TokenType {
    pub fn from_literal(literal: String) -> Option<TokenType> {
        // TODO: Stop this from being built every time we run TokenType
        let mut literal_lookup: HashMap<&str, TokenType> = HashMap::new();

        literal_lookup.insert("and", TokenType::And);
        literal_lookup.insert("class", TokenType::Class);
        literal_lookup.insert("else", TokenType::Else);
        literal_lookup.insert("false", TokenType::False);
        literal_lookup.insert("fun", TokenType::Fun);
        literal_lookup.insert("for", TokenType::For);
        literal_lookup.insert("if", TokenType::If);
        literal_lookup.insert("nil", TokenType::Nil);
        literal_lookup.insert("or", TokenType::Or);
        literal_lookup.insert("and", TokenType::And);
        literal_lookup.insert("print", TokenType::Print);
        literal_lookup.insert("return", TokenType::Return);
        literal_lookup.insert("super", TokenType::Super);
        literal_lookup.insert("this", TokenType::This);
        literal_lookup.insert("true", TokenType::True);
        literal_lookup.insert("var", TokenType::Var);
        literal_lookup.insert("while", TokenType::While);
        literal_lookup.insert("and", TokenType::And);

        return literal_lookup
            .get(literal.as_str())
            .map(|token| token.clone());
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

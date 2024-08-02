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
    pub fn from_literal(literal: &str) -> Option<TokenType> {
        // TODO: Stop this HashMap from being built every time we run TokenType
        let mut keyword_lookup: HashMap<&str, TokenType> = HashMap::new();

        keyword_lookup.insert("class", TokenType::Class);
        keyword_lookup.insert("else", TokenType::Else);
        keyword_lookup.insert("false", TokenType::False);
        keyword_lookup.insert("fun", TokenType::Fun);
        keyword_lookup.insert("for", TokenType::For);
        keyword_lookup.insert("if", TokenType::If);
        keyword_lookup.insert("nil", TokenType::Nil);
        keyword_lookup.insert("or", TokenType::Or);
        keyword_lookup.insert("and", TokenType::And);
        keyword_lookup.insert("return", TokenType::Return);
        keyword_lookup.insert("super", TokenType::Super);
        keyword_lookup.insert("this", TokenType::This);
        keyword_lookup.insert("true", TokenType::True);
        keyword_lookup.insert("let", TokenType::Let);
        keyword_lookup.insert("while", TokenType::While);
        keyword_lookup.insert("and", TokenType::And);

        return keyword_lookup.get(literal).cloned();
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
    Plus,
    Minus,
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
    Let,
    While,
}

#[cfg(test)]
mod tests {
    use crate::tokens::Token;

    #[test]
    fn token_can_be_displayed() {
        let token = Token {
            lexeme: "for".to_owned(),
            token_type: crate::tokens::TokenType::For,
            line_number: 1,
        };

        assert_eq!(token.to_string(), "For @ line=1 : \"for\"");
    }
}

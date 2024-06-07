#![cfg(test)]

use crate::{
    scanner::{scan_token, scan_tokens},
    tokens::{Token, TokenType},
};

#[test]
fn scan_single_token() {
    let token = scan_token(&mut "{".chars().peekable(), &mut 0);

    assert_eq!(token, Ok(Some(TokenType::LeftBrace)))
}

#[test]
fn scan_double_token() {
    let token = scan_token(&mut "==".chars().peekable(), &mut 0);

    assert_eq!(token, Ok(Some(TokenType::EqualEqual)))
}

#[test]
fn scan_single_number_literal() {
    let token = scan_token(&mut "0".chars().peekable(), &mut 0);

    assert_eq!(token, Ok(Some(TokenType::Number(0.0))))
}

#[test]
fn scan_multiple_tokens() {
    let tokens_result = scan_tokens("! == \"hii\" 5.2 { } // ignored");

    assert!(tokens_result.is_ok());

    let tokens_vec = tokens_result.unwrap();
    let mut tokens = tokens_vec.iter();

    assert_eq!(
        tokens.next(),
        Some(
            &(Token {
                lexeme: "".to_string(),
                line_number: 1,
                token_type: TokenType::Bang
            })
        )
    );

    assert_eq!(
        tokens.next(),
        Some(
            &(Token {
                lexeme: "".to_string(),
                line_number: 1,
                token_type: TokenType::EqualEqual
            })
        )
    );

    assert_eq!(
        tokens.next(),
        Some(
            &(Token {
                lexeme: "".to_string(),
                line_number: 1,
                token_type: TokenType::String("hii".to_string())
            })
        )
    );

    assert_eq!(
        tokens.next(),
        Some(
            &(Token {
                lexeme: "".to_string(),
                line_number: 1,
                token_type: TokenType::Number(5.2)
            })
        )
    );

    assert_eq!(
        tokens.next(),
        Some(
            &(Token {
                lexeme: "".to_string(),
                line_number: 1,
                token_type: TokenType::LeftBrace
            })
        )
    );

    assert_eq!(
        tokens.next(),
        Some(
            &(Token {
                lexeme: "".to_string(),
                line_number: 1,
                token_type: TokenType::RightBrace
            })
        )
    );

    assert_eq!(
        tokens.next(),
        Some(
            &(Token {
                lexeme: "".to_string(),
                line_number: 1,
                token_type: TokenType::EOF
            })
        )
    );
}

#[test]
fn scan_multiple_lines() {
    let tokens_result = scan_tokens(
        r#"
<
! // this comment should be ignored
(
)
"#,
    );

    assert!(tokens_result.is_ok());

    let tokens_vec = tokens_result.unwrap();
    let mut tokens = tokens_vec.iter();

    assert_eq!(
        tokens.next(),
        Some(
            &(Token {
                lexeme: "".to_string(),
                line_number: 2,
                token_type: TokenType::Less
            })
        )
    );

    assert_eq!(
        tokens.next(),
        Some(
            &(Token {
                lexeme: "".to_string(),
                line_number: 3,
                token_type: TokenType::Bang
            })
        )
    );

    assert_eq!(
        tokens.next(),
        Some(
            &(Token {
                lexeme: "".to_string(),
                line_number: 4,
                token_type: TokenType::LeftParen
            })
        )
    );

    assert_eq!(
        tokens.next(),
        Some(
            &(Token {
                lexeme: "".to_string(),
                line_number: 5,
                token_type: TokenType::RightParen
            })
        )
    );
}

#[test]
fn scan_single_line_string() {
    let tokens_result = scan_tokens("\"inside string\"");

    assert!(tokens_result.is_ok());

    let tokens_vec = tokens_result.unwrap();
    let mut tokens = tokens_vec.iter();

    assert_eq!(
        tokens.next(),
        Some(
            &(Token {
                lexeme: "".to_string(),
                line_number: 1,
                token_type: TokenType::String("inside string".to_string()),
            })
        )
    );
}

#[test]
fn scan_multi_line_string() {
    let tokens_result = scan_tokens("\"inside\n string\"}");

    assert!(tokens_result.is_ok());

    let tokens_vec: Vec<Token> = tokens_result.unwrap();
    let mut tokens = tokens_vec.iter();

    assert_eq!(
        tokens.next(),
        Some(
            &(Token {
                lexeme: "".to_string(),
                line_number: 2,
                token_type: TokenType::String("inside\n string".to_string()),
            })
        )
    );

    assert_eq!(
        tokens.next(),
        Some(
            &(Token {
                lexeme: "".to_string(),
                line_number: 2,
                token_type: TokenType::RightBrace,
            })
        )
    );
}

#[test]
fn scan_number_with_trailing_dot() {
    let tokens_result = scan_tokens("100.");

    assert!(tokens_result.is_ok());

    let tokens_vec = tokens_result.unwrap();
    let mut tokens = tokens_vec.iter();

    assert_eq!(
        tokens.next(),
        Some(
            &(Token {
                lexeme: "".to_string(),
                line_number: 1,
                token_type: TokenType::Number(100.0),
            })
        )
    );

    assert_eq!(
        tokens.next(),
        Some(
            &(Token {
                lexeme: "".to_string(),
                line_number: 1,
                token_type: TokenType::Dot,
            })
        )
    );
}

#[test]
fn scan_decimal_number() {
    let tokens_result = scan_tokens("420.69");

    assert!(tokens_result.is_ok());

    let tokens_vec = tokens_result.unwrap();
    let mut tokens = tokens_vec.iter();

    assert_eq!(
        tokens.next(),
        Some(
            &(Token {
                lexeme: "".to_string(),
                line_number: 1,
                token_type: TokenType::Number(420.69),
            })
        )
    );

    assert_eq!(
        tokens.next(),
        Some(
            &(Token {
                lexeme: "".to_string(),
                line_number: 1,
                token_type: TokenType::EOF,
            })
        )
    );
}

#[test]
fn scan_keywords() {
    let tokens_result = scan_tokens("return and let");

    assert!(tokens_result.is_ok());

    let tokens_vec = tokens_result.unwrap();
    let mut tokens = tokens_vec.iter();

    assert_eq!(
        tokens.next(),
        Some(
            &(Token {
                lexeme: "".to_string(),
                line_number: 1,
                token_type: TokenType::Return,
            })
        )
    );

    assert_eq!(
        tokens.next(),
        Some(
            &(Token {
                lexeme: "".to_string(),
                line_number: 1,
                token_type: TokenType::And,
            })
        )
    );
    assert_eq!(
        tokens.next(),
        Some(
            &(Token {
                lexeme: "".to_string(),
                line_number: 1,
                token_type: TokenType::Let,
            })
        )
    );
}

#[test]
fn scan_identifiers() {
    let tokens_result = scan_tokens("fun epic complex_char");

    assert!(tokens_result.is_ok());

    let tokens_vec = tokens_result.unwrap();
    let mut tokens = tokens_vec.iter();

    assert_eq!(
        tokens.next(),
        Some(
            &(Token {
                lexeme: "".to_string(),
                line_number: 1,
                token_type: TokenType::Fun,
            })
        )
    );

    assert_eq!(
        tokens.next(),
        Some(
            &(Token {
                lexeme: "".to_string(),
                line_number: 1,
                token_type: TokenType::Identifier("epic".to_string()),
            })
        )
    );

    assert_eq!(
        tokens.next(),
        Some(
            &(Token {
                lexeme: "".to_string(),
                line_number: 1,
                token_type: TokenType::Identifier("complex_char".to_string()),
            })
        )
    );
}

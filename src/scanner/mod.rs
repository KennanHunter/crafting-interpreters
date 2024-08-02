pub mod tests;
mod util;

use std::{char, iter::Peekable, str::Chars};

use util::is_valid_literal_character;

use crate::{
    errors::ScanningError,
    tokens::{Token, TokenType},
};

/// Takes in the raw source code text and converts it to either a vector of tokens
/// or a vector of the errors found
///
/// Errors found returns all errors in the scanning process, even if there are
/// multiple scanning issues
pub fn scan_tokens(source: &str) -> Result<Vec<Token>, Vec<ScanningError>> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut parsing_errors: Vec<ScanningError> = Vec::new();

    let mut line = 1;
    let mut characters: Peekable<Chars> = source.chars().peekable();

    loop {
        let token_type_result = scan_token(&mut characters, &mut line);

        match token_type_result {
            TokenScanResult::Token(token_type) => {
                let token = Token {
                    token_type: token_type.clone(),
                    lexeme: "".to_string(),
                    line_number: line,
                };

                tokens.push(token);
            }
            TokenScanResult::Whitespace => {
                continue;
            }
            TokenScanResult::Err(err) => {
                parsing_errors.push(ScanningError {
                    line_number: line,
                    message: err.message,
                });
                continue;
            }
            TokenScanResult::End => {
                break;
            }
        }
    }

    if !parsing_errors.is_empty() {
        return Err(parsing_errors);
    }

    Ok(tokens)
}

#[derive(Debug, PartialEq)]
pub enum TokenScanResult {
    Token(TokenType),
    Whitespace,
    End,
    Err(ScanningError),
}

/// Progresses characters past the next token and returns it in TokenType enum form
///
/// Can error with ScanningError on malformed literals or non-ascii characters
fn scan_token(characters: &mut Peekable<Chars>, line: &mut usize) -> TokenScanResult {
    let Some(character) = characters.next() else {
        return TokenScanResult::End;
    };

    use TokenScanResult::*;

    match character {
        '(' => Token(TokenType::LeftParen),
        ')' => Token(TokenType::RightParen),
        '{' => Token(TokenType::LeftBrace),
        '}' => Token(TokenType::RightBrace),
        ',' => Token(TokenType::Comma),
        '.' => Token(TokenType::Dot),
        '-' => Token(TokenType::Minus),
        '+' => Token(TokenType::Plus),
        ';' => Token(TokenType::Semicolon),
        '*' => Token(TokenType::Star),

        '!' => match characters.peek() {
            Some('=') => {
                characters.next();
                Token(TokenType::BangEqual)
            }
            _ => Token(TokenType::Bang),
        },

        '=' => match characters.peek() {
            Some('=') => {
                characters.next();
                Token(TokenType::EqualEqual)
            }
            _ => Token(TokenType::Equal),
        },

        '<' => match characters.peek() {
            Some('=') => {
                characters.next();
                Token(TokenType::LessEqual)
            }
            _ => Token(TokenType::Less),
        },

        '>' => match characters.peek() {
            Some('=') => {
                characters.next();
                Token(TokenType::GreaterEqual)
            }
            _ => Token(TokenType::Greater),
        },

        '/' => match characters.peek() {
            Some('/') => {
                // When we see two slashes, ignore everything past it until we receive a newline
                loop {
                    match characters.peek() {
                        Some(char) if *char == '\n' => {
                            break;
                        }
                        Some(_) => {
                            characters.next();
                        }
                        None => break,
                    }
                }

                Whitespace
            }
            _ => Token(TokenType::Slash),
        },

        '"' => {
            let mut contained_string = String::new();

            loop {
                match characters.next() {
                    Some('"') => break,
                    Some('\n') => {
                        *line += 1;
                        contained_string.push('\n');
                    }
                    Some(ch) => contained_string.push(ch),
                    None => {
                        return Err(ScanningError {
                            line_number: *line,
                            message: "Closing quotation at end of string not found".to_string(),
                        })
                    }
                }
            }

            Token(TokenType::String(contained_string))
        }

        '\n' => {
            *line += 1;
            Whitespace
        }

        ' ' | '\r' | '\t' => Whitespace,

        number if number.is_ascii_digit() => {
            let mut contained_number_literal = String::from(number);

            loop {
                match characters.peek() {
                    Some(char) if char.is_ascii_digit() => {
                        contained_number_literal.push(*char);
                        characters.next();
                    }

                    Some(char) if *char == '.' => {
                        // TODO: Figure out how to do this without cloning the iterator,
                        // it will either be a little bit slower than not cloning
                        // if it only clones the iterator logic, or VERY much slower
                        // if it clones the iterator values with it, and i don't quite
                        // know how to tell the difference
                        let post_decimal_character: Option<char> = characters.clone().nth(1);

                        match post_decimal_character {
                            Some(unwrapped_post_decimal_character)
                                if unwrapped_post_decimal_character.is_ascii_digit() =>
                            {
                                let decimal_point = characters.next();

                                contained_number_literal.push(decimal_point.unwrap());

                                contained_number_literal.push(characters.next().unwrap());
                            }
                            _ => break,
                        }
                    }

                    _ => break,
                }
            }

            match contained_number_literal.parse::<f64>() {
                Ok(parsed) => Token(TokenType::Number(parsed)),
                Result::Err(_) => {
                    return Err(ScanningError {
                        line_number: *line,
                        message: "Failed to parse number".to_string(),
                    })
                }
            }
        }

        literal if literal.is_ascii_alphabetic() => {
            let mut contained_literal = String::from(literal);

            while characters
                .peek()
                .is_some_and(|&char| is_valid_literal_character(&char))
            {
                let char = characters.next().unwrap();
                contained_literal.push(char);
            }

            match TokenType::from_literal(&contained_literal) {
                Some(keyword) => Token(keyword),
                None => Token(TokenType::Identifier(contained_literal)),
            }
        }

        unrecognized_character => {
            return Err(ScanningError {
                line_number: *line,
                message: format!("unrecognized character {}", unrecognized_character),
            })
        }
    }
}

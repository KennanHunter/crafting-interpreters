use std::{iter::Peekable, str::Chars};

use crate::{
    errors::ParsingError,
    tokens::{Token, TokenType},
};

pub fn scan_tokens(source: String) -> Result<Vec<Token>, Vec<ParsingError>> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut parsing_errors: Vec<ParsingError> = Vec::new();

    let mut line = 1;
    let mut characters: Peekable<Chars> = source.chars().peekable();

    loop {
        let token_type_result = scan_token(&mut characters, &mut line);

        match token_type_result {
            Ok(Some(token_type)) => {
                let token = Token {
                    token_type: token_type.clone(),
                    lexeme: "".to_string(),
                    line_number: line,
                };

                tokens.push(token);

                if token_type == TokenType::EOF {
                    break;
                }
            }
            Ok(None) => {
                continue;
            }
            Err(err) => {
                parsing_errors.push(ParsingError {
                    line_number: line,
                    message: err.message,
                });
                continue;
            }
        }
    }

    if !parsing_errors.is_empty() {
        return Err(parsing_errors);
    }

    return Ok(tokens);
}

fn scan_token(
    characters: &mut Peekable<Chars>,
    line: &mut usize,
) -> Result<Option<TokenType>, ParsingError> {
    let character: Option<char> = characters.next();

    if character.is_none() {
        return Ok(Some(TokenType::EOF));
    };

    let token_type = match character.unwrap() {
        '(' => Some(TokenType::LeftParen),
        ')' => Some(TokenType::RightParen),
        '{' => Some(TokenType::LeftBrace),
        '}' => Some(TokenType::RightBrace),
        ',' => Some(TokenType::Comma),
        '.' => Some(TokenType::Dot),
        '-' => Some(TokenType::Mins),
        '+' => Some(TokenType::Plus),
        ';' => Some(TokenType::Semicolon),
        '*' => Some(TokenType::Star),

        '!' => match characters.peek() {
            Some('=') => {
                characters.next();
                Some(TokenType::BangEqual)
            }
            _ => Some(TokenType::Bang),
        },

        '=' => match characters.peek() {
            Some('=') => {
                characters.next();
                Some(TokenType::EqualEqual)
            }
            _ => Some(TokenType::Equal),
        },

        '<' => match characters.peek() {
            Some('=') => {
                characters.next();
                Some(TokenType::LessEqual)
            }
            _ => Some(TokenType::Less),
        },

        '>' => match characters.peek() {
            Some('=') => {
                characters.next();
                Some(TokenType::GreaterEqual)
            }
            _ => Some(TokenType::Greater),
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

                None
            }
            _ => Some(TokenType::Slash),
        },

        '"' => {
            let mut contained_string = String::new();

            loop {
                match characters.next() {
                    Some(char) if char == '"' => break,
                    Some(char) if char == '\n' => {
                        *line += 1;
                        contained_string.push(char)
                    }

                    Some(char) => contained_string.push(char),
                    None => {
                        return Err(ParsingError {
                            line_number: *line,
                            message: "End of string not found".to_string(),
                        })
                    }
                }
            }

            Some(TokenType::String(contained_string))
        }

        '\n' => {
            *line += 1;
            None
        }

        ' ' => None,
        '\r' => None,
        '\t' => None,

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
                Ok(parsed) => Some(TokenType::Number(parsed)),
                Err(_) => {
                    return Err(ParsingError {
                        line_number: *line,
                        message: "Failed to parse number".to_string(),
                    })
                }
            }
        }

        character => {
            return Err(ParsingError {
                line_number: *line,
                message: format!("unrecognized character {}", character),
            })
        }
    };

    return Ok(token_type);
}

#[cfg(test)]
mod tests {
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
        let tokens_result = scan_tokens("! == \"hii\" 5.2 { } // ignored".to_string());

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
"#
            .to_string(),
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
        let tokens_result = scan_tokens("\"inside string\"".to_string());

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
        let tokens_result = scan_tokens("\"inside\n string\"}".to_string());

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
        let tokens_result = scan_tokens("100.".to_string());

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
        let tokens_result = scan_tokens("420.69".to_string());

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
}

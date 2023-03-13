pub mod tokens;

pub use crate::error::{
    CompileError,
    CompileResult,
    Span
};
pub use tokens::{
    Token,
    TokenType
};

/// Lex a source string into a list of tokens
pub fn lex(source: String) -> CompileResult<Vec<Token>> {
    let mut chars = source.chars().peekable();
    let mut tokens = Vec::new();
    let mut start_char = 0;
    let mut current_char = 0;

    // Helper function to add a token to the list
    // closure
    let add_token = |token_type: TokenType, tokens: &mut Vec<Token>, start, end| {
        tokens.push(Token::new(token_type, Span {
            start,
            end,
        }));
    };

    // Iterate over the characters in the source string
    while let Some(c) = chars.next() {
        start_char = current_char;
        current_char += 1;

        println!("{}: {:?}\t{:?}", current_char, c, &tokens);

        match c {
            // whitespace
            ' ' | '\r' | '\t' => {},
            // Single-character tokens
            '(' => add_token(TokenType::LeftParen, &mut tokens, start_char, current_char),
            ')' => add_token(TokenType::RightParen, &mut tokens, start_char, current_char),
            '{' => add_token(TokenType::LeftBrace, &mut tokens, start_char, current_char),
            '}' => add_token(TokenType::RightBrace, &mut tokens, start_char, current_char),
            ',' => add_token(TokenType::Comma, &mut tokens, start_char, current_char),
            '.' => add_token(TokenType::Dot, &mut tokens, start_char, current_char),
            '-' => add_token(TokenType::Minus, &mut tokens, start_char, current_char),
            '+' => add_token(TokenType::Plus, &mut tokens, start_char, current_char),
            ';' => add_token(TokenType::Semicolon, &mut tokens, start_char, current_char),
            '*' => add_token(TokenType::Star, &mut tokens, start_char, current_char),
            // One or two character tokens
            '!' => {
                // check for != next char
                if let Some('=') = chars.peek() {
                    // consume the next char
                    chars.next();
                    current_char += 1;
                    add_token(TokenType::BangEqual, &mut tokens, start_char, current_char);
                } else {
                    add_token(TokenType::Bang, &mut tokens, start_char, current_char);
                }
            },
            '=' => {
                // check for == next char
                if let Some('=') = chars.peek() {
                    // consume the next char
                    chars.next();
                    current_char += 1;
                    add_token(TokenType::EqualEqual, &mut tokens, start_char, current_char);
                } else {
                    add_token(TokenType::Equal, &mut tokens, start_char, current_char);
                }
            },
            '<' => {
                // check for <= next char
                if let Some('=') = chars.peek() {
                    // consume the next char
                    chars.next();
                    current_char += 1;
                    add_token(TokenType::LessEqual, &mut tokens, start_char, current_char);
                } else {
                    add_token(TokenType::Less, &mut tokens, start_char, current_char);
                }
            },
            '>' => {
                // check for >= next char
                if let Some('=') = chars.peek() {
                    // consume the next char
                    chars.next();
                    current_char += 1;
                    add_token(TokenType::GreaterEqual, &mut tokens, start_char, current_char);
                } else {
                    add_token(TokenType::Greater, &mut tokens, start_char, current_char);
                }
            },

            // string literal?
            '"' => {
                let mut string = String::new();
                while let Some(c) = chars.next() {
                    current_char += 1;
                    if c == '"' {
                        break;
                    }
                    string.push(c);
                }
                add_token(TokenType::StringLiteral(string), &mut tokens, start_char, current_char);
            },

            // integer/float literal?
            '0'..='9' => {
                let mut number = String::new();
                number.push(c);
                while let Some(c) = chars.peek() {
                    if c.is_numeric() {
                        number.push(*c);
                        chars.next();
                        current_char += 1;
                    } else {
                        break;
                    }
                }
                if let Some('.') = chars.peek() {
                    number.push(*chars.peek().unwrap());
                    chars.next();
                    current_char += 1;
                    while let Some(c) = chars.peek() {
                        if c.is_numeric() {
                            number.push(*c);
                            chars.next();
                            current_char += 1;
                        } else {
                            break;
                        }
                    }
                    add_token(TokenType::FloatLiteral(number.parse::<f64>().unwrap()), &mut tokens, start_char, current_char);
                } else {
                    add_token(TokenType::IntegerLiteral(number.parse::<i64>().unwrap()), &mut tokens, start_char, current_char);
                }
            },

            // identifier or keyword?
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut identifier = String::new();
                identifier.push(c);
                while let Some(c) = chars.peek() {
                    if c.is_alphanumeric() || *c == '_' {
                        identifier.push(*c);
                        chars.next();
                        current_char += 1;
                    } else {
                        break;
                    }
                }
                match identifier.as_str() {
                    "else" => add_token(TokenType::Else, &mut tokens, start_char, current_char),
                    "false" => add_token(TokenType::False, &mut tokens, start_char, current_char),
                    "if" => add_token(TokenType::If, &mut tokens, start_char, current_char),
                    "return" => add_token(TokenType::Return, &mut tokens, start_char, current_char),
                    "true" => add_token(TokenType::True, &mut tokens, start_char, current_char),
                    "let" => add_token(TokenType::Let, &mut tokens, start_char, current_char),
                    "while" => add_token(TokenType::While, &mut tokens, start_char, current_char),
                    _ => add_token(TokenType::Identifier(identifier), &mut tokens, start_char, current_char),
                }
            },

            _ => {
                return Err(CompileError::SyntaxError(
                    format!("Unexpected character: {}", c),
                    Span {
                        start: start_char,
                        end: current_char,
                    }
                ));
            }
        };
    };

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let source = r#"let a = 1 + 2 * 3;"#;
        let tokens = lex(source.to_string()).unwrap();
        println!("{:#?}", tokens);

    }
}
use crate::prelude::*;
use crate::types::{Error, JsonValue, Token, TokenType};

#[derive(Debug)]
pub struct Lexer<'a> {
    source: &'a str,
    index: usize,
    line: usize,
    column: usize,
}

impl<'a> From<&'a str> for Lexer<'a> {
    fn from(source: &'a str) -> Self {
        Lexer {
            source,
            index: 0,
            line: 0,
            column: 0,
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn lex(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens: Vec<Token> = Vec::new();

        while self.index < self.source.len() {
            if let Some(next) = self.source.chars().nth(self.index) {
                // Skip whitespace
                if next.is_ascii_whitespace() {
                    self.whitespace(next);
                    continue;
                }

                let token = match next {
                    JSON_QUOTE => self.lex_string()?,
                    n if n.is_ascii_digit() => self.lex_number()?,
                    '.' | '-' | 'e' | 'E' => self.lex_number()?,
                    't' | 'f' => self.lex_boolean()?,
                    'n' => self.lex_null()?,
                    c => self.lex_syntax(c)?,
                };
                tokens.push(token);
            }
        }

        Ok(tokens)
    }

    fn lex_string(&mut self) -> Result<Token, Error> {
        let start_column = self.column;

        self.index += 1; // Move past JSON_QUOTE
        self.column += 1;

        let mut chars = self.source.chars().skip(self.index);
        // Find index of next JSON_QUOTE
        let char_index = match chars.position(|c| c == JSON_QUOTE) {
            Some(idx) => idx,
            None => return Err(Error::UnexpectedEndOfString),
        };

        // Get characters between JSON_QUOTE's
        let json_string = self
            .source
            .chars()
            .skip(self.index)
            .take(char_index)
            .collect::<String>();

        // Increment position
        let inc = json_string.len() + 1;
        self.index += inc;
        self.column += inc;

        Ok(Token {
            token_type: TokenType::String,
            value: Some(JsonValue::String(json_string)),
            line: self.line,
            column: start_column,
        })
    }

    fn lex_number(&mut self) -> Result<Token, Error> {
        let start_column = self.column;
        let numeric_chars = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.', '-', 'e', 'E',
        ];

        let chars = self
            .source
            .chars()
            .skip(self.index)
            .take_while(|c| numeric_chars.contains(c))
            .collect::<String>();

        let number = chars.parse::<f64>()?;

        // Increment position
        let inc = chars.len();
        self.index += inc;
        self.column += inc;

        Ok(Token {
            token_type: TokenType::Number,
            value: Some(JsonValue::Number(number)),
            line: self.line,
            column: start_column,
        })
    }

    fn lex_boolean(&mut self) -> Result<Token, Error> {
        let keywords = ["true", "false"];

        for &keyword in &keywords {
            if self.source[self.index..].starts_with(keyword) {
                let json_value = match keyword {
                    "true" => JsonValue::Bool(true),
                    "false" => JsonValue::Bool(false),
                    _ => unreachable!(), // unreachable since keywords are known
                };
                let start_column = self.column;

                // Increment position
                let inc = keyword.len();
                self.index += inc;
                self.column += inc;

                return Ok(Token {
                    token_type: TokenType::Bool,
                    value: Some(json_value),
                    line: self.line,
                    column: start_column,
                });
            }
        }

        let char = self.source.chars().nth(self.index).unwrap();
        Err(Error::UnexpectedCharacter(char, (self.line, self.column)))
    }

    fn lex_null(&mut self) -> Result<Token, Error> {
        let null = "null";
        if self.source[self.index..].starts_with(null) {
            let start_column = self.column;

            // Increment position
            let inc = null.len();
            self.index += inc;
            self.column += inc;

            Ok(Token {
                token_type: TokenType::Null,
                value: Some(JsonValue::Null),
                line: self.line,
                column: start_column,
            })
        } else {
            let char = self.source.chars().nth(self.index).unwrap();
            Err(Error::UnexpectedCharacter(char, (self.line, self.column)))
        }
    }

    fn lex_syntax(&mut self, char: char) -> Result<Token, Error> {
        let token_type = match char {
            JSON_COMMA => TokenType::Comma,
            JSON_COLON => TokenType::Colon,
            JSON_LEFTBRACKET => TokenType::LeftBracket,
            JSON_RIGHTBRACKET => TokenType::RightBracket,
            JSON_LEFTBRACE => TokenType::LeftBrace,
            JSON_RIGHTBRACE => TokenType::RightBrace,
            c => return Err(Error::UnexpectedCharacter(c, (self.line, self.column))),
        };

        let start_column = self.column;

        // Increment position
        self.index += 1;
        self.column += 1;

        Ok(Token {
            token_type,
            value: None,
            line: self.line,
            column: start_column,
        })
    }

    // Skip token assignment and increment position
    fn whitespace(&mut self, char: char) {
        if char == '\n' {
            self.line += 1;
            self.index += 1;
            self.column = 0;
        } else {
            self.index += 1;
            self.column += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_empty_string() {
        let input = "";
        let mut lexer = Lexer::from(input);
        let tokens = lexer.lex();
        assert!(tokens.is_ok());
        assert!(tokens.unwrap().is_empty());
    }
    #[test]
    fn lex_empty_object() {
        let input = "{}";
        let mut lexer = Lexer::from(input);
        let tokens = lexer.lex();
        assert!(tokens.is_ok());

        let expected = vec![
            Token {
                token_type: TokenType::LeftBrace,
                value: None,
                line: 0,
                column: 0,
            },
            Token {
                token_type: TokenType::RightBrace,
                value: None,
                line: 0,
                column: 1,
            },
        ];
        assert_eq!(expected, tokens.unwrap());
    }
    #[test]
    fn lex_string_key_value_pair() {
        let input = r#"{"key":"value"}"#;
        let mut lexer = Lexer::from(input);
        let tokens = lexer.lex();
        assert!(tokens.is_ok());

        let expected = vec![
            Token {
                token_type: TokenType::LeftBrace,
                value: None,
                line: 0,
                column: 0,
            },
            Token {
                token_type: TokenType::String,
                value: Some(JsonValue::String("key".to_string())),
                line: 0,
                column: 1,
            },
            Token {
                token_type: TokenType::Colon,
                value: None,
                line: 0,
                column: 6,
            },
            Token {
                token_type: TokenType::String,
                value: Some(JsonValue::String("value".to_string())),
                line: 0,
                column: 7,
            },
            Token {
                token_type: TokenType::RightBrace,
                value: None,
                line: 0,
                column: 14,
            },
        ];
        assert_eq!(expected, tokens.unwrap());
    }
    #[test]
    fn lex_numeric_key_value_pair() {
        let input = r#"{"key":3.14}"#;
        let mut lexer = Lexer::from(input);
        let tokens = lexer.lex();
        assert!(tokens.is_ok());

        let expected = vec![
            Token {
                token_type: TokenType::LeftBrace,
                value: None,
                line: 0,
                column: 0,
            },
            Token {
                token_type: TokenType::String,
                value: Some(JsonValue::String("key".to_string())),
                line: 0,
                column: 1,
            },
            Token {
                token_type: TokenType::Colon,
                value: None,
                line: 0,
                column: 6,
            },
            Token {
                token_type: TokenType::Number,
                value: Some(JsonValue::Number(3.14)),
                line: 0,
                column: 7,
            },
            Token {
                token_type: TokenType::RightBrace,
                value: None,
                line: 0,
                column: 11,
            },
        ];
        assert_eq!(expected, tokens.unwrap());
    }
    #[test]
    fn lex_boolean_key_value_pair() {
        let input = r#"{"key":true}"#;
        let mut lexer = Lexer::from(input);
        let tokens = lexer.lex();
        assert!(tokens.is_ok());

        let expected = vec![
            Token {
                token_type: TokenType::LeftBrace,
                value: None,
                line: 0,
                column: 0,
            },
            Token {
                token_type: TokenType::String,
                value: Some(JsonValue::String("key".to_string())),
                line: 0,
                column: 1,
            },
            Token {
                token_type: TokenType::Colon,
                value: None,
                line: 0,
                column: 6,
            },
            Token {
                token_type: TokenType::Bool,
                value: Some(JsonValue::Bool(true)),
                line: 0,
                column: 7,
            },
            Token {
                token_type: TokenType::RightBrace,
                value: None,
                line: 0,
                column: 11,
            },
        ];
        assert_eq!(expected, tokens.unwrap());
    }
    #[test]
    fn lex_null_key_value_pair() {
        let input = r#"{"key":null}"#;
        let mut lexer = Lexer::from(input);
        let tokens = lexer.lex();
        assert!(tokens.is_ok());

        let expected = vec![
            Token {
                token_type: TokenType::LeftBrace,
                value: None,
                line: 0,
                column: 0,
            },
            Token {
                token_type: TokenType::String,
                value: Some(JsonValue::String("key".to_string())),
                line: 0,
                column: 1,
            },
            Token {
                token_type: TokenType::Colon,
                value: None,
                line: 0,
                column: 6,
            },
            Token {
                token_type: TokenType::Null,
                value: Some(JsonValue::Null),
                line: 0,
                column: 7,
            },
            Token {
                token_type: TokenType::RightBrace,
                value: None,
                line: 0,
                column: 11,
            },
        ];
        assert_eq!(expected, tokens.unwrap());
    }
    #[test]
    fn lex_array() {
        let input = r#"{"obj":["value"]}"#;
        let mut lexer = Lexer::from(input);
        let tokens = lexer.lex();
        assert!(tokens.is_ok());

        let expected = vec![
            Token {
                token_type: TokenType::LeftBrace,
                value: None,
                line: 0,
                column: 0,
            },
            Token {
                token_type: TokenType::String,
                value: Some(JsonValue::String("obj".to_string())),
                line: 0,
                column: 1,
            },
            Token {
                token_type: TokenType::Colon,
                value: None,
                line: 0,
                column: 6,
            },
            Token {
                token_type: TokenType::LeftBracket,
                value: None,
                line: 0,
                column: 7,
            },
            Token {
                token_type: TokenType::String,
                value: Some(JsonValue::String("value".to_string())),
                line: 0,
                column: 8,
            },
            Token {
                token_type: TokenType::RightBracket,
                value: None,
                line: 0,
                column: 15,
            },
            Token {
                token_type: TokenType::RightBrace,
                value: None,
                line: 0,
                column: 16,
            },
        ];
        assert_eq!(expected, tokens.unwrap());
    }
    #[test]
    fn lex_object() {
        let input = r#"{"obj":{"key":"value"}}"#;
        let mut lexer = Lexer::from(input);
        let tokens = lexer.lex();
        assert!(tokens.is_ok());

        let expected = vec![
            Token {
                token_type: TokenType::LeftBrace,
                value: None,
                line: 0,
                column: 0,
            },
            Token {
                token_type: TokenType::String,
                value: Some(JsonValue::String("obj".to_string())),
                line: 0,
                column: 1,
            },
            Token {
                token_type: TokenType::Colon,
                value: None,
                line: 0,
                column: 6,
            },
            Token {
                token_type: TokenType::LeftBrace,
                value: None,
                line: 0,
                column: 7,
            },
            Token {
                token_type: TokenType::String,
                value: Some(JsonValue::String("key".to_string())),
                line: 0,
                column: 8,
            },
            Token {
                token_type: TokenType::Colon,
                value: None,
                line: 0,
                column: 13,
            },
            Token {
                token_type: TokenType::String,
                value: Some(JsonValue::String("value".to_string())),
                line: 0,
                column: 14,
            },
            Token {
                token_type: TokenType::RightBrace,
                value: None,
                line: 0,
                column: 21,
            },
            Token {
                token_type: TokenType::RightBrace,
                value: None,
                line: 0,
                column: 22,
            },
        ];
        assert_eq!(expected, tokens.unwrap());
    }
}

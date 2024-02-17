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
        let numeric_chars = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.', '-', 'e', 'E'];

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


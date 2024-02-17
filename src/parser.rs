use std::collections::HashMap;

use crate::types::{Error, JsonValue, Token, TokenType};

#[derive(Debug)]
pub struct Parser<'a> {
    tokens: &'a [Token],
    index: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Parser {
        Parser { tokens, index: 0 }
    }
    pub fn parse(&mut self) -> Result<JsonValue, Error> {
        if self.tokens.is_empty() {
            return Err(Error::UnexpectedEndOfInput);
        }

        let token = &self.tokens[self.index];
        match token.token_type {
            TokenType::LeftBrace => self.parse_object(),
            TokenType::LeftBracket => self.parse_array(),
            _ => Err(Error::UnexpectedToken(format!(
                "Expected JSON object or array, got {}, line {}, col {}",
                token.token_type, token.line, token.column
            ))),
        }
    }
    fn parse_array(&mut self) -> Result<JsonValue, Error> {
        let mut array: Vec<JsonValue> = Vec::new();
        self.index += 1; // Move past TokenType::LeftBracket

        loop {
            let token = &self.tokens[self.index];
            if token.token_type == TokenType::RightBracket {
                self.index += 1;
                return Ok(JsonValue::Array(array));
            }

            // Parse array value (and increment self.index)
            let value = self.parse_value()?;
            array.push(value);

            let token = &self.tokens[self.index];
            if token.token_type == TokenType::Comma {
                self.index += 1; // Move past TokenType::Comma

                // JSON doesn't allow trailing commas
                if self.tokens[self.index].token_type == TokenType::RightBrace {
                    return Err(Error::UnexpectedToken(format!(
                        "Unexpected comma at line {}, column {}", token.line, token.column
                    )));
                }
            } else if token.token_type == TokenType::RightBracket {
                self.index += 1; // Move past TokenType::RightBracket
                return Ok(JsonValue::Array(array));
            } else {
                return Err(Error::UnexpectedToken(format!(
                    "Unexpected token in array: {}, line {}, col {}",
                    token.token_type, token.line, token.column
                )));
            }
        }
    }
    fn parse_object(&mut self) -> Result<JsonValue, Error> {
        let mut object: HashMap<String, JsonValue> = HashMap::new();
        self.index += 1; // Move past TokenType::LeftBrace

        loop {
            if self.tokens[self.index].token_type == TokenType::RightBrace {
                self.index += 1; // Move past TokenType::RightBrace
                return Ok(JsonValue::Object(object));
            }

            // Parse key
            let key_token = &self.tokens[self.index];
            let maybe_key = match key_token.token_type {
                TokenType::String => key_token.value.clone(),
                _ => {
                    return Err(Error::UnexpectedToken(
                        "Expected string as object key".to_string(),
                    ))
                }
            };
            let key = match maybe_key {
                Some(JsonValue::String(str)) => str,
                _ => {
                    return Err(Error::UnexpectedToken(
                        "Expected string as object key".to_string(),
                    ))
                }
            };
            self.index += 1; // Move past key

            // Check next token is a colon
            if self.tokens[self.index].token_type != TokenType::Colon {
                return Err(Error::UnexpectedToken(
                    "Expected colon after object key".to_string(),
                ));
            }
            self.index += 1; // Move past TokenType::Colon

            // Parse object value (and increment self.index)
            let value = self.parse_value()?;
            object.insert(key, value);

            let token = &self.tokens[self.index];
            if token.token_type == TokenType::Comma {
                self.index += 1; // Move past TokenType::Comma

                // JSON doesn't allow trailing commas
                if self.tokens[self.index].token_type == TokenType::RightBrace {
                    return Err(Error::UnexpectedToken(format!(
                        "Unexpected comma at line {}, column {}", token.line, token.column
                    )));
                }
            } else if token.token_type == TokenType::RightBrace {
                self.index += 1; // Move past TokenType::RightBrace
                return Ok(JsonValue::Object(object));
            } else {
                return Err(Error::UnexpectedToken(format!(
                    "Unexpected token in object: {}, line {}, col {}",
                    token.token_type, token.line, token.column
                )));
            }
        }
    }
    fn parse_value(&mut self) -> Result<JsonValue, Error> {
        let token = &self.tokens[self.index];
        match token.token_type {
            TokenType::LeftBrace => self.parse_object(),
            TokenType::LeftBracket => self.parse_array(),
            TokenType::String | TokenType::Number | TokenType::Bool | TokenType::Null => {
                self.index += 1;
                let value = token.value.clone().ok_or_else(|| {
                    Error::UnexpectedToken(format!(
                        "Unexpected {} at line {}, col {}",
                        token.token_type, token.line, token.column
                    ))
                })?;
                Ok(value)
            }
            _ => Err(Error::UnexpectedToken(format!(
                "Unexpected token {} at line {}, col {}",
                token.token_type, token.line, token.column
            ))),
        }
    }
}


use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Null => f.write_str("null"),
            Self::Bool(bool) => write!(f, "{}", bool),
            Self::Number(number) => write!(f, "{}", number),
            Self::String(string) => write!(f, r#""{}""#, string),
            Self::Array(_) => f.write_str("[Array]"),
            Self::Object(_) => f.write_str("[Object]"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Null,
    Bool,
    Number,
    String,
    Comma,
    Colon,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Null => f.write_str("null"),
            Self::Bool => f.write_str("boolean"),
            Self::Number => f.write_str("number"),
            Self::String => f.write_str("string"),
            Self::Comma => f.write_str(","),
            Self::Colon => f.write_str(":"),
            Self::LeftBrace => f.write_str("{"),
            Self::RightBrace => f.write_str("}"),
            Self::LeftBracket => f.write_str("["),
            Self::RightBracket => f.write_str("]"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Option<JsonValue>,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug)]
pub enum Error {
    UnexpectedEndOfString,
    UnexpectedEndOfArray,
    UnexpectedEndOfObject,
    UnexpectedEndOfInput,
    UnexpectedCharacter(char, (usize, usize)),
    ParseNumberError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UnexpectedEndOfString => f.write_str("Unexpected end-of-string quote"),
            Self::UnexpectedEndOfArray => f.write_str("Unexpected end-of-array bracket"),
            Self::UnexpectedEndOfObject => f.write_str("Unexpected end-of-object brace"),
            Self::UnexpectedEndOfInput => f.write_str("Unexpected end of input"),
            Self::UnexpectedCharacter(char, (line, col)) => write!(
                f,
                "Unexpected character: {}, line {} column {}",
                char, line, col
            ),
            Self::ParseNumberError(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::ParseNumberError(format!("Failed to parse integer: {}", err))
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(err: std::num::ParseFloatError) -> Self {
        Error::ParseNumberError(format!("Failed to parse float: {}", err))
    }
}

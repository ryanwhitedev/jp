use lexer::Lexer;
use parser::Parser;
use types::{Error, JsonValue};

mod lexer;
mod parser;
mod prelude;
mod types;

pub fn parse(input: &str) -> Result<JsonValue, Error> {
    let mut lexer = Lexer::from(input);
    let tokens = lexer.lex()?;
    let mut parser = Parser::new(&tokens);
    parser.parse()
}

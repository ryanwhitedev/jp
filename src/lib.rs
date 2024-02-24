use lexer::Lexer;
use parser::Parser;
use types::{Error, Token, TokenType};

mod lexer;
mod parser;
mod prelude;
mod types;

pub fn parse(input: &str) -> Result<(), Error> {
    // Lexical analysis
    let mut lexer = Lexer::from(input);
    let tokens = lexer.lex()?;

    // Syntactic analysis
    let mut parser = Parser::new(&tokens);
    parser.parse()?;

    // Format output
    let json = format(&tokens, 4)?;
    println!("{}", json);

    Ok(())
}

fn format(tokens: &[Token], indent: usize) -> Result<String, Error> {
    let mut offset = 0;
    let mut skip_indent = false;
    let mut skip_newline = false;

    let json = tokens
        .windows(2)
        .map(|window| {
            let token = &window[0];
            let next = &window[1];
            match token.token_type {
                TokenType::LeftBrace | TokenType::LeftBracket => {
                    let str = {
                        if token.token_type == TokenType::LeftBrace
                            && next.token_type == TokenType::RightBrace
                            || token.token_type == TokenType::LeftBracket
                                && next.token_type == TokenType::RightBracket
                        {
                            skip_newline = true;
                            format!("{}", token.token_type)
                        } else if skip_indent {
                            format!(
                                "{}\n{}",
                                token.token_type,
                                " ".repeat(indent * (offset + 1))
                            )
                        } else {
                            format!(
                                "{}{}\n{}",
                                " ".repeat(indent * offset),
                                token.token_type,
                                " ".repeat(indent * (offset + 1))
                            )
                        }
                    };
                    offset += 1;
                    skip_indent = false;
                    str
                }
                TokenType::RightBrace | TokenType::RightBracket => {
                    offset -= 1;
                    let str = {
                        if skip_newline {
                            format!("{}", token.token_type)
                        } else {
                            format!("\n{}{}", " ".repeat(indent * offset), token.token_type)
                        }
                    };
                    skip_indent = false;
                    skip_newline = false;
                    str
                }
                TokenType::Comma => {
                    skip_indent = true;
                    format!("{}\n{}", token.token_type, " ".repeat(indent * offset))
                }
                TokenType::Colon => {
                    skip_indent = true;
                    format!("{} ", token.token_type)
                }
                _ => {
                    if let Some(value) = &token.value {
                        format!("{}", value)
                    } else {
                        format!("{}", token.token_type)
                    }
                }
            }
        })
        .collect::<String>();

    Ok(json)
}

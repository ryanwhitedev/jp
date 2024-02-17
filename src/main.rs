use jp::lexer::Lexer;

fn main() {
    let json = r#"{
        "key": "value",
        "int": 42,
        "float": -3.14,
        "bool_true": true,
        "bool_false": false,
        "null_type": null,
        "empty_array": [],
        "array": ["one", "two"],
        "empty_object": {},
        "object": {
            "int": 42
        }
    }"#;

    let mut lexer = Lexer::from(json);
    let tokens = match lexer.lex() {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("Invalid JSON: {}", e);
            std::process::exit(1);
        }
    };
    dbg!(tokens);
}

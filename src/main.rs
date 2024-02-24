use jp::parse;

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

    if let Err(e) = parse(&json) {
        eprintln!("Invalid JSON: {}", e);
    }
}

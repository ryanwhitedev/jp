use std::fs::File;
use std::io::{self, IsTerminal, Read};

use jp::parse;

const USAGE: &str = "Usage: jp [FILE]";

fn print_usage() {
    println!("{}", USAGE);
    std::process::exit(0);
}

fn main() {
    let mut buffer = String::new();
    let args: Vec<String> = std::env::args().skip(1).collect();

    if let Some(filename) = args.get(0) {
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                eprintln!("jp: {}: No such file or directory", filename);
                std::process::exit(1);
            }
        };
        file.read_to_string(&mut buffer).unwrap_or_else(|e| {
            eprintln!("Error reading from file {}: {}", filename, e);
            std::process::exit(1);
        });
    } else if !io::stdin().is_terminal() {
        // Allow piped input via stdin
        io::stdin().read_to_string(&mut buffer).unwrap_or_else(|e| {
            eprintln!("Error reading from stdin: {}", e);
            std::process::exit(1);
        });
    } else {
        print_usage();
    }

    if let Err(e) = parse(&buffer) {
        eprintln!("Invalid JSON: {}", e);
        std::process::exit(1);
    }
}

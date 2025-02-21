mod lexer;

use std::env;
use std::io::{self, Write};
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    let code = match command.as_str() {
        "tokenize" => lexer::tokenize(filename),
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            0
        }
    };

    if code != 0 {
        exit(code);
    }
}

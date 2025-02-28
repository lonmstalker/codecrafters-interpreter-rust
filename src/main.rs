mod lexer;
mod parser;
mod domain;
mod test;

use std::env;
use std::io::{self, Write};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return ExitCode::SUCCESS;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let result = lexer::tokenize(filename);
            for x in result.tokens {
                println!("{}", x);
            }
            return ExitCode::from(result.code);
        }
        "parse" => {
            let result = lexer::tokenize(filename);
            if result.code != 0 {
                return ExitCode::from(result.code);
            }
            match parser::parse(result) {
                Ok(ast) => {
                    println!("{}", ast.expr);
                }
                Err(e) => {
                    if let domain::ParserError::Default(_, _, code) = e {
                        return ExitCode::from(code);
                    }
                }
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
        }
    };

    ExitCode::SUCCESS
}

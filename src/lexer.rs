use std::{fs, io};
use std::io::Write;

pub fn tokenize(filename: &String) -> i32 {

    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", *filename).unwrap();
        String::new()
    });

    process_tokens(file_contents)
}

fn process_tokens(code: String) -> i32 {
    let mut line = 1;

    if !code.is_empty() {
        for c in code.chars() {
            match c {
                '(' => println!("LEFT_PAREN ( null"),
                ')' => println!("RIGHT_PAREN ) null"),
                '{' => println!("LEFT_BRACE {{ null"),
                '}' => println!("RIGHT_BRACE }} null"),
                ',' => println!("COMMA , null"),
                '.' => println!("DOT . null"),
                '+' => println!("PLUS + null"),
                '-' => println!("MINUS - null"),
                ';' => println!("SEMICOLON ; null"),
                '*' => println!("STAR * null"),
                '\n' => line += 1,
                '$' | '#' => {
                    eprintln!("[line {}] Error: Unexpected character: {}", line, c);
                    return 65
                }
                _ => {}
            }
        }
        println!("EOF  null");
    } else {
        println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
    }

    0
}
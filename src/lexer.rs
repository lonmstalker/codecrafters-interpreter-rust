use std::{fs, io};
use std::fmt::Display;
use std::io::Write;

pub fn tokenize(filename: &String) -> i32 {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", *filename).unwrap();
        String::new()
    });

    let result = process_tokens(file_contents);
    println!("EOF  null");

    result
}

fn process_tokens(code: String) -> i32 {
    let mut line = 1;
    let mut result = 0;
    let mut tokens = Vec::new();
    let mut data = code.chars().peekable();

    if !code.is_empty() {
        while let Some(c) = data.next() {
            match c {
                '(' => tokens.push(Token::new(TokenType::LEFT_PAREN, c.to_string())),
                ')' => tokens.push(Token::new(TokenType::RIGHT_PAREN, c.to_string())),
                '{' => tokens.push(Token::new(TokenType::LEFT_BRACE, c.to_string())),
                '}' => tokens.push(Token::new(TokenType::RIGHT_BRACE, c.to_string())),
                ',' => tokens.push(Token::new(TokenType::COMMA, c.to_string())),
                '.' => tokens.push(Token::new(TokenType::DOT, c.to_string())),
                '+' => tokens.push(Token::new(TokenType::PLUS, c.to_string())),
                '-' => tokens.push(Token::new(TokenType::MINUS, c.to_string())),
                ';' => tokens.push(Token::new(TokenType::SEMICOLON, c.to_string())),
                '*' => tokens.push(Token::new(TokenType::STAR, c.to_string())),
                '=' => {
                    let next = data.peek();

                    if next.is_some() && *next.unwrap() == '=' {
                        tokens.push(Token::new(TokenType::EQUAL_EQUAL, "==".to_string()));
                        data.next();
                    } else {
                        tokens.push(Token::new(TokenType::EQUAL, c.to_string()));
                    }

                }
                '!' => {
                    let next = data.peek();
                    
                    if next.is_some() {
                        let next_data = *next.unwrap();
                        if next_data == '=' {
                            tokens.push(Token::new(TokenType::BANG_EQUAL, "!=".to_string()));
                            data.next();
                            continue;
                        }
                    } else {
                        tokens.push(Token::new(TokenType::BANG, c.to_string()));
                    }
                }
                '\n' => line += 1,
                unknown => {
                    eprintln!("[line {}] Error: Unexpected character: {}", line, unknown);
                    result = 65
                }
            }
        }
    }

    for token in tokens {
        println!("{:?} {} null", token._type, token._string)
    }

    result
}

pub struct Token {
    _type: TokenType,
    _string: String,
    _value: Option<String>,
}

impl Token {
    pub fn new(_type: TokenType, _string: String) -> Self {
        Token {
            _type,
            _string,
            _value: None,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {} {}",
            self._type,
            self._string,
            self._value.clone().unwrap_or("null".to_string())
        )
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
enum TokenType {
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    STAR,
    FOWARD_SLASH,
    EQUAL,
    EQUAL_EQUAL,
    EOF,
    BANG,
    BANG_EQUAL,
}
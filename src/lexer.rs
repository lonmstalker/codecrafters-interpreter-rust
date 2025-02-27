use std::{fs, io};
use std::fmt::Display;
use std::io::Write;
use std::iter::Peekable;
use std::str::Chars;

pub fn tokenize(filename: &String) -> i32 {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|ex| {
        writeln!(io::stderr(), "Failed to read file {}, ex: {}", *filename, ex).unwrap();
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
                '(' => tokens.push(Token::new_char(TokenType::LEFT_PAREN, c)),
                ')' => tokens.push(Token::new_char(TokenType::RIGHT_PAREN, c)),
                '{' => tokens.push(Token::new_char(TokenType::LEFT_BRACE, c)),
                '}' => tokens.push(Token::new_char(TokenType::RIGHT_BRACE, c)),
                ',' => tokens.push(Token::new_char(TokenType::COMMA, c)),
                '.' => tokens.push(Token::new_char(TokenType::DOT, c)),
                '+' => tokens.push(Token::new_char(TokenType::PLUS, c)),
                '-' => tokens.push(Token::new_char(TokenType::MINUS, c)),
                ';' => tokens.push(Token::new_char(TokenType::SEMICOLON, c)),
                '*' => tokens.push(Token::new_char(TokenType::STAR, c)),
                '=' => {
                    let token = composite_token(&mut data,
                                                '=',
                                                || Token::new(TokenType::EQUAL_EQUAL, "==".to_string()),
                                                || Token::new_char(TokenType::EQUAL, c));
                    tokens.push(token)
                }
                '!' => {
                    let token = composite_token(&mut data,
                                                '=',
                                                || Token::new(TokenType::BANG_EQUAL, "!=".to_string()),
                                                || Token::new_char(TokenType::BANG, c));
                    tokens.push(token)
                }
                '<' => {
                    let token = composite_token(&mut data,
                                                '=',
                                                || Token::new(TokenType::LESS_EQUAL, "<=".to_string()),
                                                || Token::new_char(TokenType::LESS, c));
                    tokens.push(token)
                }
                '>' => {
                    let token = composite_token(&mut data,
                                                '=',
                                                || Token::new(TokenType::GREATER_EQUAL, ">=".to_string()),
                                                || Token::new_char(TokenType::GREATER, c));
                    tokens.push(token)
                }
                '/' => {
                    if let Some(&next) = data.peek() {
                        if next == '/' {
                           skip_while(&mut data, |token| token != '\n');
                        } else {
                            tokens.push(Token::new_char(TokenType::SLASH, c))
                        }
                    }
                }
                '\n' => line += 1,
                ' ' | '\r' | '\t' => continue,
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

fn composite_token(data: &mut Peekable<Chars>,
                   next_char: char,
                   then_func: impl FnOnce() -> Token,
                   else_func: impl FnOnce() -> Token) -> Token {
    if let Some(&next) = data.peek() {
        if next == next_char {
            data.next();
            return then_func();
        }
    }
    else_func()
}

fn skip_while(data: &mut Peekable<Chars>, predict: impl Fn(char) -> bool) {
    loop {
        if let Some(&next) = data.peek() {
            if !predict(next) {
                break
            }
            data.next();
        } else {
            break
        }
    }
}

pub struct Token {
    _type: TokenType,
    _string: String,
    _value: Option<String>,
}

impl Token {

    pub fn new_char(_type: TokenType, _char: char) -> Self {
        Token {
            _type,
            _value: None,
            _string: _char.to_string(),
        }
    }

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
    EQUAL,
    EQUAL_EQUAL,
    BANG,
    BANG_EQUAL,
    LESS,
    GREATER,
    LESS_EQUAL,
    GREATER_EQUAL,
    SLASH
}
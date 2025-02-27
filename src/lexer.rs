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
                            continue;
                        }
                    }
                    tokens.push(Token::new_char(TokenType::SLASH, c))
                }
                '"' => {
                    let string_res = string(&mut data, line);
                    if string_res.2 != 0 {
                        result = string_res.2;
                    } else {
                        tokens.push(Token::new_content(TokenType::STRING, string_res.0, string_res.1));
                    }
                }
                '\n' => line += 1,
                ' ' | '\r' | '\t' => continue,
                _ => {
                    if c.is_numeric() {
                        let num_result = number(c, &mut data);
                        tokens.push(Token::new_content(TokenType::NUMBER, num_result.0, num_result.1));
                    } else if c.is_ascii_alphabetic() || c == '_' {
                        let identifier_res = identifier(c, &mut data);
                        tokens.push(Token::new(identifier_res.0, identifier_res.1));
                    } else {
                        eprintln!("[line {}] Error: Unexpected character: {}", line, c);
                        result = 65
                    }
                }
            }
        }
    }

    for token in tokens {
        println!("{}", token)
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
                break;
            }
            data.next();
        } else {
            break;
        }
    }
}

fn string(data: &mut Peekable<Chars>, line: i32) -> (String, String, i32) {
    let mut result = 0;
    let mut value = String::new();
    let mut string = String::from('"');

    loop {
        if let Some(next) = data.next() {
            string.push(next);
            if next == '"' {
                break;
            }
            value.push(next);
        } else {
            eprintln!("[line {}] Error: Unterminated string.", line);
            result = 65;
            break;
        }
    }

    (string, value, result)
}

fn number(current: char, data: &mut Peekable<Chars>) -> (String, String) {
    let mut dot_index: i32 = -1;
    let mut value = String::from(current);
    let mut number = String::from(current);

    loop {
        if let Some(&next) = data.peek() {
            if !next.is_numeric() && next != '.' {
                break;
            }

            value.push(next);
            number.push(next);

            if next == '.' {
                dot_index = value.len() as i32;
            }

            data.next();
        } else {
            break;
        }
    }

    if dot_index == -1 {
        value.push_str(".0");
    } else {
        let mut len = value.len();
        for (i, val) in value.char_indices().rev() {
            // убираем все 0 после точки, оставляя только первую
            if val != '0' || dot_index == i as i32 {
                break;
            }
            len -= 1;
        }
        value.truncate(len);
    }

    (number, value)
}

fn identifier(current: char, data: &mut Peekable<Chars>) -> (TokenType, String) {
    let mut result = String::from(current);

    loop {
        if let Some(&next) = data.peek() {
            if next.is_ascii_alphanumeric() {
                result.push(next);
                data.next();
            } else {
                break
            }
        } else {
            break
        }
    }

    (TokenType::IDENTIFIER, result)
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

    pub fn new_content(_type: TokenType, _string: String, _value: String) -> Self {
        Token {
            _type,
            _string,
            _value: Some(_value),
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
    SLASH,
    STRING,
    NUMBER,
    IDENTIFIER
}
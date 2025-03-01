use crate::domain::{KeywordType, Token, TokenType, Tokens};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::io::Write;
use std::iter::Peekable;
use std::str::Chars;
use std::{fs, io};

pub fn tokenize_code(code: String) -> Tokens {
    process_tokens(code)
}

pub fn tokenize(filename: &String) -> Tokens {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|ex| {
        writeln!(io::stderr(), "Failed to read file {}, ex: {}", *filename, ex).unwrap();
        String::new()
    });

    process_tokens(file_contents)
}

fn process_tokens(code: String) -> Tokens {
    let mut line = 1;
    let mut col = 0;

    let mut result: u8 = 0;
    let mut tokens = Vec::new();
    let mut data = code.chars().peekable();

    if !code.is_empty() {
        while let Some(c) = data.next() {
            col += 1;
            match c {
                '(' => tokens.push(Token::new_char(TokenType::LEFT_PAREN, c, line, col)),
                ')' => tokens.push(Token::new_char(TokenType::RIGHT_PAREN, c, line, col)),
                '{' => tokens.push(Token::new_char(TokenType::LEFT_BRACE, c, line, col)),
                '}' => tokens.push(Token::new_char(TokenType::RIGHT_BRACE, c, line, col)),
                ',' => tokens.push(Token::new_char(TokenType::COMMA, c, line, col)),
                '.' => tokens.push(Token::new_char(TokenType::DOT, c, line, col)),
                '+' => tokens.push(Token::new_char(TokenType::PLUS, c, line, col)),
                '-' => tokens.push(Token::new_char(TokenType::MINUS, c, line, col)),
                ';' => tokens.push(Token::new_char(TokenType::SEMICOLON, c, line, col)),
                '*' => tokens.push(Token::new_char(TokenType::STAR, c, line, col)),
                '=' => {
                    let (_type, string, cur_col) =
                        composite_token(&mut data, '=', '=', col, TokenType::EQUAL_EQUAL, TokenType::EQUAL);
                    tokens.push(Token::new(_type, string, line, col, cur_col))
                }
                '!' => {
                    let (_type, string, cur_col) =
                        composite_token(&mut data, '!', '=', col, TokenType::BANG_EQUAL, TokenType::BANG);
                    tokens.push(Token::new(_type, string, line, col, cur_col))
                }
                '<' => {
                    let (_type, string, cur_col) =
                        composite_token(&mut data, '<', '=', col, TokenType::LESS_EQUAL, TokenType::LESS);
                    tokens.push(Token::new(_type, string, line, col, cur_col))
                }
                '>' => {
                    let (_type, string, cur_col) =
                        composite_token(&mut data, '>', '=', col, TokenType::GREATER_EQUAL, TokenType::GREATER);
                    tokens.push(Token::new(_type, string, line, col, cur_col))
                }
                '/' => {
                    if let Some(&next) = data.peek() {
                        if next == '/' {
                            skip_while(&mut data, |token| token != '\n');
                            line += 1;
                            continue;
                        }
                    }
                    tokens.push(Token::new_char(TokenType::SLASH, c, line, col))
                }
                '"' => {
                    let string_res = string(&mut data, line, col);
                    if string_res.2 != 0 {
                        result = string_res.2;
                    } else {
                        let cur_col = col;
                        col = string_res.3;
                        tokens.push(Token::new_content(TokenType::STRING, string_res.0, string_res.1, line, cur_col, col));
                    }
                }
                '\n' => {
                    line += 1;
                    col = 1;
                }
                ' ' | '\r' | '\t' => continue,
                _ => {
                    // сперва строка, тк 6bz - 6 может распознаться как число, а bz отдельно identifier
                    if c.is_ascii_alphabetic() || c == '_' {

                        let identifier_res = identifier(c, &mut data, col);
                        let cur_col = col;
                        col = identifier_res.2;
                        tokens.push(Token::new(identifier_res.0, identifier_res.1, line, cur_col, col));
                    } else if c.is_numeric() {

                        let num_result = number(c, &mut data, col);
                        let cur_col = col;
                        col = num_result.2;
                        tokens.push(Token::new_content(TokenType::NUMBER, num_result.0, num_result.1, line, cur_col, col));
                    } else {

                        eprintln!("[line {}] Error: Unexpected character: {}", line, c);
                        result = 65
                    }
                }
            }
        }
    }

    tokens.push(Token::new(TokenType::EOF, String::new(), line, col, col));

    Tokens { tokens, code : result }
}

fn composite_token(data: &mut Peekable<Chars>,
                   current_char: char,
                   next_char: char,
                   column: i32,
                   then_token: TokenType,
                   else_token: TokenType) -> (TokenType, String, i32) {
    if let Some(&next) = data.peek() {
        if next == next_char {
            let mut result = String::from(current_char);
            result.push(data.next().unwrap());
            return (then_token, result, column + 1);
        }
    }
    (else_token, String::from(current_char), column + 1)
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

fn string(data: &mut Peekable<Chars>, line: i32, col: i32) -> (String, String, u8, i32) {
    let mut col = col;
    let mut result: u8 = 0;
    let mut value = String::new();
    let mut string = String::from('"');

    loop {
        if let Some(next) = data.next() {
            col += 1;
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

    (string, value, result, col)
}

fn number(current: char, data: &mut Peekable<Chars>, col: i32) -> (String, String, i32) {
    let mut col = col;
    let mut dot_index: i32 = -1;

    let mut value = String::from(current);
    let mut number = String::from(current);

    loop {
        if let Some(&next) = data.peek() {
            if !next.is_numeric() && next != '.' {
                break;
            }

            col += 1;
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

    (number, value, col)
}

fn identifier(current: char, data: &mut Peekable<Chars>, col: i32) -> (TokenType, String, i32) {
    let mut col = col;
    let mut result = String::from(current);

    loop {
        if let Some(&next) = data.peek() {
            if next.is_ascii_alphanumeric() || next == '_' || next.is_numeric() {
                col += 1;
                result.push(next);
                data.next();
            } else {
                break;
            }
        } else {
            break;
        }
    }

    match KEYWORDS.get(&result.as_str()) {
        None => (TokenType::IDENTIFIER, result, col),
        Some(keyword) => (TokenType::KEYWORD(keyword.clone()), result, col)
    }
}

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, KeywordType> = HashMap::from([
                ("and", KeywordType::AND),
                ("class", KeywordType::CLASS),
                ("else", KeywordType::ELSE),
                ("false", KeywordType::FALSE),
                ("for", KeywordType::FOR),
                ("fun", KeywordType::FUN),
                ("if", KeywordType::IF),
                ("nil", KeywordType::NIL),
                ("or", KeywordType::OR),
                ("print", KeywordType::PRINT),
                ("return", KeywordType::RETURN),
                ("super", KeywordType::SUPER),
                ("this", KeywordType::THIS),
                ("true", KeywordType::TRUE),
                ("var", KeywordType::VAR),
                ("while", KeywordType::WHILE),
            ]);
}
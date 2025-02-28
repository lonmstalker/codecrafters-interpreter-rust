use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct AST {
    pub expr: Expr,
}

#[derive(Debug, Clone)]
pub struct Tokens {
    pub code: u8,
    pub tokens: Vec<Token>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Literal(String, Token),
    Grouping(Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum ParserError {
    Default(String, Token, u8)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub _type: TokenType,
    pub _string: String,
    pub _value: Option<String>,
    pub _line: i32,
    pub _column_from: i32,
    pub _column_to: i32,
}

impl Token {
    pub fn new_char(_type: TokenType, _char: char, _line: i32, _column: i32) -> Self {
        Token {
            _type,
            _value: None,
            _string: _char.to_string(),
            _line,
            _column_from: _column,
            _column_to: _column,
        }
    }

    pub fn new_content(_type: TokenType, _string: String,
                       _value: String, _line: i32, _column_from: i32, _column_to: i32) -> Self {
        Token {
            _type,
            _string,
            _value: Some(_value),
            _line,
            _column_from,
            _column_to,
        }
    }

    pub fn new(_type: TokenType, _string: String, _line: i32, _column_from: i32, _column_to: i32) -> Self {
        Token {
            _type,
            _string,
            _value: None,
            _line,
            _column_from,
            _column_to,
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Unary(token, right) => write!(f, "{} {}", token, right),
            Expr::Binary(left, token, right) => write!(f, "{} {} {}", left, token, right),
            Expr::Literal(literal, _) => write!(f, "{}", literal),
            Expr::Grouping(literal) => write!(f, "{}", literal)
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self._type.to_string(),
            self._string,
            self._value.clone().unwrap_or("null".to_string())
        )
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    EOF,
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
    IDENTIFIER,
    KEYWORD(KeywordType),
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum KeywordType {
    AND,
    CLASS,
    ELSE,
    FALSE,
    FOR,
    FUN,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
}

impl TokenType {
    fn to_string(&self) -> &'static str {
        match self {
            TokenType::EOF => "EOF",
            TokenType::LEFT_PAREN => "LEFT_PAREN",
            TokenType::RIGHT_PAREN => "RIGHT_PAREN",
            TokenType::LEFT_BRACE => "LEFT_BRACE",
            TokenType::RIGHT_BRACE => "RIGHT_BRACE",
            TokenType::COMMA => "COMMA",
            TokenType::DOT => "DOT",
            TokenType::MINUS => "MINUS",
            TokenType::PLUS => "PLUS",
            TokenType::SEMICOLON => "SEMICOLON",
            TokenType::STAR => "STAR",
            TokenType::EQUAL => "EQUAL",
            TokenType::EQUAL_EQUAL => "EQUAL_EQUAL",
            TokenType::BANG => "BANG",
            TokenType::BANG_EQUAL => "BANG_EQUAL",
            TokenType::LESS => "LESS",
            TokenType::GREATER => "GREATER",
            TokenType::LESS_EQUAL => "LESS_EQUAL",
            TokenType::GREATER_EQUAL => "GREATER_EQUAL",
            TokenType::SLASH => "SLASH",
            TokenType::STRING => "STRING",
            TokenType::NUMBER => "NUMBER",
            TokenType::IDENTIFIER => "IDENTIFIER",
            TokenType::KEYWORD(kw) => kw.to_string()
        }
    }
}

impl KeywordType {
    pub fn to_string(&self) -> &'static str {
        match self {
            KeywordType::AND => "AND",
            KeywordType::CLASS => "CLASS",
            KeywordType::ELSE => "ELSE",
            KeywordType::FALSE => "FALSE",
            KeywordType::FOR => "FOR",
            KeywordType::FUN => "FUN",
            KeywordType::IF => "IF",
            KeywordType::NIL => "NIL",
            KeywordType::OR => "OR",
            KeywordType::PRINT => "PRINT",
            KeywordType::RETURN => "RETURN",
            KeywordType::SUPER => "SUPER",
            KeywordType::THIS => "THIS",
            KeywordType::TRUE => "TRUE",
            KeywordType::VAR => "VAR",
            KeywordType::WHILE => "WHILE"
        }
    }
}
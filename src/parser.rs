use crate::domain::TokenType::{BANG_EQUAL, EQUAL_EQUAL, GREATER, GREATER_EQUAL, LESS, LESS_EQUAL};
use crate::domain::{Expr, KeywordType, ParserError, Token, TokenType, Tokens, AST, ParserError::Default};
use std::cell::Cell;
use std::rc::Rc;

/// priority top-down
/// literal -> string, number, boolean, nil
/// unary -> ! -
/// factor -> ! *
/// term -> + -
/// comparison -> > >= < <=
/// equality -> != ==
pub fn parse(tokens: Tokens) -> Result<AST, ParserError> {
    let parser = Parser {
        tokens: Rc::new(tokens.tokens),
        result: Cell::new(0),
        current: Cell::new(0),
    };
    Ok(AST { expr: expression(&parser)? })
}

/// начало парсинга токенов с переданного оффсета
fn expression(parser: &Parser) -> Result<Expr, ParserError> {
    equality(parser)
}

/// equality -> != ==
fn equality(parser: &Parser) -> Result<Expr, ParserError> {
    let expr = comparison(parser);

    if parser.match_tokens(&[BANG_EQUAL, EQUAL_EQUAL]) {
        let operator = parser.previous();
        let right = factor(parser);
        Ok(Expr::Binary(Box::from(expr?), operator.expect("operator not found").clone(), Box::from(right?)))
    } else {
        expr
    }
}

/// comparison -> > >= < <=
fn comparison(parser: &Parser) -> Result<Expr, ParserError> {
    let expr = term(parser);

    if parser.match_tokens(&[GREATER, GREATER_EQUAL, LESS, LESS_EQUAL]) {
        let operator = parser.previous();
        let right = factor(parser);
        Ok(Expr::Binary(Box::from(expr?), operator.expect("operator not found").clone(), Box::from(right?)))
    } else {
        expr
    }
}

/// term -> + -
/// если есть + - то возрвращает Binary, иначе Unary
fn term(parser: &Parser) -> Result<Expr, ParserError> {
    let expr = factor(parser);

    if parser.match_tokens(&[TokenType::MINUS, TokenType::PLUS]) {
        let operator = parser.previous();
        let right = factor(parser);
        Ok(Expr::Binary(Box::from(expr?), operator.expect("operator not found").clone(), Box::from(right?)))
    } else {
        expr
    }
}

/// factor -> ! *
/// если есть ! * то возрвращает Binary, иначе Unary
fn factor(parser: &Parser) -> Result<Expr, ParserError> {
    let expr = unary(parser);

    if parser.match_tokens(&[TokenType::SLASH, TokenType::STAR]) {
        let operator = parser.previous();
        let right = unary(parser);
        Ok(Expr::Binary(Box::from(expr?), operator.expect("operator not found").clone(), Box::from(right?)))
    } else {
        expr
    }
}

// unary -> ! -
// если есть ! -, то возрвращает Unary, иначе Primary
fn unary(parser: &Parser) -> Result<Expr, ParserError> {
    if parser.match_tokens(&[TokenType::BANG, TokenType::MINUS]) {
        let operator = parser.previous();
        let right = unary(parser);
        Ok(Expr::Unary(operator.expect("operator not found").clone(), Box::from(right?)))
    } else {
        primary(parser)
    }
}

/// literal -> string, number, boolean, nil, (, )
fn primary(parser: &Parser) -> Result<Expr, ParserError> {
    if parser.match_token(TokenType::KEYWORD(KeywordType::FALSE)) {
        get_or_ex_value("false invalid", parser, |_| Expr::Literal("false".to_string()))
    } else if parser.match_token(TokenType::KEYWORD(KeywordType::TRUE)) {
        get_or_ex_value("true invalid", parser, |_| Expr::Literal("true".to_string()))
    } else if parser.match_token(TokenType::KEYWORD(KeywordType::NIL)) {
        get_or_ex_value("null invalid", parser, |_| Expr::Literal("null".to_string()))
    } else if parser.match_token(TokenType::STRING) {
        get_or_ex_value("string invalid", parser, |val| Expr::Literal(val))
    } else if parser.match_token(TokenType::NUMBER) {
        get_or_ex_value("number invalid", parser, |val| Expr::Literal(val))
    } else if parser.match_token(TokenType::LEFT_PAREN) {
        let expr = expression(parser)?;
        parser.consume(TokenType::LEFT_PAREN, "Expect ')' after expression.")?;
        Ok(Expr::Grouping(Box::from(expr)))
    } else {
        return Err(Default("token not supported".to_string(),
                           parser.previous().expect("token not found").clone(),
                           65))
    }
}

fn get_or_ex_value(message: &str, parser: &Parser, convert: impl FnOnce(String) -> Expr) -> Result<Expr, ParserError> {
    match parser.previous() {
        None => Err(Default(message.to_string(),
                            parser.peek().expect("token not found").clone(),
                            65)),
        Some(result) => {
            match &result._value {
                None => Err(Default(message.to_string(),
                                    parser.previous().expect("token not found").clone(),
                                    65)),
                Some(value) => Ok(convert(value.clone()))
            }
        }
    }
}

struct Parser {
    result: Cell<i32>,
    current: Cell<usize>,
    tokens: Rc<Vec<Token>>,
}

impl Parser {
    /// отдает текущий токен и не двигает оффсет
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current.get())
    }

    /// отдает следующий токен и двигает оффсет
    fn next(&self) -> Option<&Token> {
        self.current.set(self.current.get() + 1);
        self.tokens.get(self.current.get())
    }

    /// отдает предыдущий токен
    fn previous(&self) -> Option<&Token> {
        self.tokens.get(self.current.get().saturating_sub(1))
    }

    /// проверяет конец токенов
    fn at_end(&self) -> bool {
        match self.tokens.get(self.current.get()) {
            None => false,
            Some(val) => val._type == TokenType::EOF
        }
    }

    /// проверяет совпадение текущего оффсета с типом
    fn check(&self, token_type: TokenType) -> bool {
        if self.at_end() { return false; }
        if let Some(val) = self.peek() {
            val._type == token_type
        } else { false }
    }

    /// проверяет, что текущий токен нужного типа и двигает оффсет
    fn consume(&self, token_type: TokenType, message: &str) -> Result<&Token, ParserError> {
        if self.check(token_type) {
            Ok(self.next().expect("token error"))
        } else {
            Err(Default(message.to_string(), self.peek().expect("token error").clone(), 65))
        }
    }

    /// проверяет текущий токен равен ли искомым и сдвигает оффсет, возвраащая следующий элемент
    fn match_tokens(&self, token_types: &[TokenType]) -> bool {
        for token in token_types {
            if self.check(token.clone()) {
                self.next();
                return true;
            }
        }
        false
    }

    /// проверяет текущий токен равен ли искомому и сдвигает оффсет, возвраащая следующий элемент
    fn match_token(&self, token: TokenType) -> bool {
        if let Some(next) = self.peek() {
            if token == next._type {
                self.next();
                return true;
            }
        }
        false
    }
}
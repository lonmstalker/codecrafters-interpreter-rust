use crate::domain::{ParserError, Tokens, AST};
use crate::lexer::tokenize_code;
use crate::parser::parse;

#[cfg(test)]
mod test_lexer {
    use crate::domain::{Token, TokenType};
    use crate::test::generate_tokens;
    use std::process::ExitCode;

    #[test]
    fn test_base_code_tokens() {

        // given:
        let code = "_123foo bar _6az world_ _hello (=";

        // when:
        let tokens = generate_tokens(code.to_string());

        // then:
        println!("{:?}", tokens.tokens);
        assert_eq!(8, tokens.tokens.len());
        assert_eq!(ExitCode::SUCCESS, tokens.code);

        check_tokens(
            tokens.tokens,
            Vec::from([
                (TokenType::IDENTIFIER, "_123foo"),
                (TokenType::IDENTIFIER, "bar"),
                (TokenType::IDENTIFIER, "_6az"),
                (TokenType::IDENTIFIER, "world_"),
                (TokenType::IDENTIFIER, "_hello"),
                (TokenType::LEFT_PAREN, "("),
                (TokenType::EQUAL, "="),
                (TokenType::EOF, ""),
            ]),
        )
    }

    #[test]
    fn test_number_and_string_tokens() {

        // given:
        let code = "122 34.4304 333.0000 \"hello\"";

        // when:
        let tokens = generate_tokens(code.to_string());

        // then:
        println!("{:?}", tokens.tokens);
        assert_eq!(5, tokens.tokens.len());
        assert_eq!(ExitCode::SUCCESS, tokens.code);

        check_tokens_val(
            tokens.tokens,
            Vec::from([
                (TokenType::NUMBER, "122", Some("122.0")),
                (TokenType::NUMBER, "34.4304", Some("34.4304")),
                (TokenType::NUMBER, "333.0000", Some("333.0")),
                (TokenType::STRING, "\"hello\"", Some("hello")),
                (TokenType::EOF, "", None)
            ]),
        )
    }

    fn check_tokens(tokens: Vec<Token>, expected: Vec<(TokenType, &str)>) {
        for (index, expected_token) in expected.into_iter().enumerate() {
            let token = tokens.get(index);
            assert!(token.is_some());
            assert_eq!(expected_token.0, token.unwrap()._type);
            assert_eq!(expected_token.1.to_string(), *token.unwrap()._string)
        }
    }

    fn check_tokens_val(tokens: Vec<Token>, expected: Vec<(TokenType, &str, Option<&str>)>) {
        for (index, expected_token) in expected.into_iter().enumerate() {
            let token = tokens.get(index);
            assert!(token.is_some());
            assert_eq!(expected_token.0, token.unwrap()._type);
            assert_eq!(expected_token.1.to_string(), *token.unwrap()._string);

            if let Some(val) = expected_token.2 {
                assert_eq!(val.to_string(), token.unwrap().clone()._value.unwrap())
            }
        }
    }
}

#[cfg(test)]
mod test_parser {
    use crate::domain::{Expr, TokenType};
    use crate::test::parse_tokens;

    #[test]
    fn test_base_code_parser() {

        // given:
        let code = "2 + 2";

        // when:
        let ast_result = parse_tokens(code.to_string());

        // then:
        assert!(ast_result.is_ok());
        println!("{:?}", ast_result);

        let ast = ast_result.unwrap();

        match ast.expr {
            Expr::Binary(left, token, right) => {
                assert_eq!(TokenType::PLUS, token._type);

                let right_val = &*right;

                match *left {
                    Expr::Literal(val) => assert_eq!("2.0", val),
                    _ => assert!(false, "invalid type left")
                }

                match *right {
                    Expr::Literal(val) => assert_eq!("2.0", val),
                    _ => assert!(false, "invalid type right")
                }
            }
            _ => assert!(false, "invalid type main")
        }
    }
}

fn parse_tokens(code: String) -> Result<AST, ParserError> {
    parse(generate_tokens(code))
}

fn generate_tokens(code: String) -> Tokens {
    tokenize_code(code)
}
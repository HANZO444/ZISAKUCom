use thiserror::Error;

use crate::ast;
use crate::token;

type ParseResult = Result<(ast::Expr, Vec<token::Token>), Box<dyn std::error::Error>>;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("operand must be int literal but got {actual:?}")]
    OperandMustBeIntLiteral { actual: token::TokenKind },
}

pub fn parse(tokens: Vec<token::Token>) -> Result<ast::Expr, Box<dyn std::error::Error>> {
    match add_sub(tokens) {
        Ok((node, _tokens)) => Ok(node),
        Err(e) => Err(e),
    }
}

fn add_sub(tokens: Vec<token::Token>) -> ParseResult {
    let (mut lhs, mut rest) = int_literal(tokens)?;

    loop {
        let operator = rest[0].kind;
        match operator {
            token::TokenKind::PLUS => {
                let (rhs, r) = int_literal(rest[1..].to_vec())?;
                rest = r;
                lhs = ast::Expr::Add {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                };
            }
            token::TokenKind::MINUS => {
                let (rhs, r) = int_literal(rest[1..].to_vec())?;
                rest = r;
                lhs = ast::Expr::Sub {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                };
            }
            _ => break,
        }
    }

    Ok((lhs, rest))
}

fn int_literal(tokens: Vec<token::Token>) -> ParseResult {
    let tk = tokens[0].kind;

    match tk {
        token::TokenKind::Integer(value) => {
            Ok((ast::Expr::Integer { value }, tokens[1..].to_vec()))
        }
        _ => Err(Box::new(ParseError::OperandMustBeIntLiteral { actual: tk })),
    }
}

#[cfg(test)]
mod parse_tests {
    use super::*;

    #[test]
    fn add_sub_test() {
        let tokens = vec![
            token::Token {
                kind: token::TokenKind::Integer(200),
            },
            token::Token {
                kind: token::TokenKind::PLUS,
            },
            token::Token {
                kind: token::TokenKind::Integer(300),
            },
            token::Token {
                kind: token::TokenKind::EOF,
            },
        ];

        let n_result = add_sub(tokens);
        eprintln!("{:?}", n_result);
        assert!(n_result.is_ok());

        let (node, rest) = n_result.unwrap();
        assert_eq!(1, rest.len());

        match node {
            ast::Expr::Add { lhs, rhs } => {
                assert_eq!(ast::Expr::Integer { value: 200 }, *lhs);
                assert_eq!(ast::Expr::Integer { value: 300 }, *rhs);
            }
            _ => unreachable!(),
        }

        let n_result = add_sub(rest);
        assert!(n_result.is_err());
    }

    #[test]
    fn int_literal_test() {
        let tokens = vec![
            token::Token {
                kind: token::TokenKind::Integer(300),
            },
            token::Token {
                kind: token::TokenKind::EOF,
            },
        ];

        let n_result = int_literal(tokens);
        assert!(n_result.is_ok());

        let (node, rest) = n_result.unwrap();
        assert_eq!(1, rest.len());
        assert_eq!(ast::Expr::Integer { value: 300 }, node);

        let n_result = int_literal(rest);
        assert!(n_result.is_err());
    }
}

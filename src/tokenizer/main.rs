use crate::token;
use thiserror::Error;

#[derive(Default)]
#[allow(unused)]
struct TokenizationContext {
    row: usize,
    column: usize,
    offset: usize,
}

type ScanResult = Result<token::Token, Box<dyn std::error::Error>>;

#[derive(Error, Debug)]
pub enum TokenizeError {
    #[error("any pattern to match {head:?} not found")]
    AnyPatternNotMatched { head: char },
}

pub fn tokenize(program: String) -> Result<Vec<token::Token>, Box<dyn std::error::Error>> {
    let mut tokens = Vec::new();
    let mut ctxt = TokenizationContext {
        ..Default::default()
    };

    loop {
        let (t, token_length) = scan(&ctxt, &program);
        ctxt.column += token_length;
        ctxt.offset += token_length;

        let t = t?;
        let is_eof = t.kind == token::TokenKind::EOF;

        if t.kind.should_ignore() {
            continue;
        }

        tokens.push(t);

        if is_eof {
            break;
        }
    }

    Ok(tokens)
}

fn scan(ctxt: &TokenizationContext, program: &str) -> (ScanResult, usize) {
    if program.len() <= ctxt.offset {
        return (
            Ok(token::Token {
                kind: token::TokenKind::EOF,
            }),
            0,
        );
    }
    let head_char = program.as_bytes()[ctxt.offset] as char;

    match head_char {
        '0'..='9' => scan_number(ctxt, program),
        '+' | '-' => scan_symbol(ctxt, program),
        c if c.is_whitespace() => scan_whitespace(ctxt, program),
        _ => (
            Err(Box::new(TokenizeError::AnyPatternNotMatched {
                head: head_char,
            })),
            0,
        ),
    }
}

fn scan_whitespace(ctxt: &TokenizationContext, program: &str) -> (ScanResult, usize) {
    let whitespace_str: String = program[ctxt.offset..]
        .chars()
        .take_while(|b| b.is_whitespace())
        .collect();

    let t = token::Token {
        kind: token::TokenKind::BLANK,
    };

    (Ok(t), whitespace_str.len())
}

fn scan_symbol(ctxt: &TokenizationContext, program: &str) -> (ScanResult, usize) {
    // 記号は複数文字であっても空白を挟まない"はず"なので，
    // 空白まで切り取って判定
    // いずれにもマッチしなければエラーを返すようにする
    let (token_kind, length) = match program[ctxt.offset..].split(' ').collect::<Vec<&str>>()[0] {
        "+" => (token::TokenKind::PLUS, 1),
        "-" => (token::TokenKind::MINUS, 1),
        _ => {
            return (
                Err(Box::new(TokenizeError::AnyPatternNotMatched {
                    head: program.as_bytes()[0] as char,
                })),
                0,
            );
        }
    };

    (Ok(token::Token { kind: token_kind }), length)
}

fn scan_number(ctxt: &TokenizationContext, program: &str) -> (ScanResult, usize) {
    let number_str: String = program[ctxt.offset..]
        .chars()
        .take_while(|b| b.is_ascii_digit())
        .collect();

    let t = match number_str.parse() {
        Ok(value) => token::Token {
            kind: token::TokenKind::Integer(value),
        },
        Err(e) => {
            return (Err(Box::new(e)), 0);
        }
    };

    (Ok(t), number_str.len())
}

#[cfg(test)]
mod tokenizer_tests {
    use super::*;

    #[test]
    fn tokenize_with_empty_string() {
        let tokens = tokenize(String::new());
        assert!(tokens.is_ok() && tokens.unwrap()[0].kind == token::TokenKind::EOF);
    }

    #[test]
    fn tokenize_with_simple_addition_and_subtraction() {
        let expected = vec![
            token::TokenKind::Integer(100),
            token::TokenKind::PLUS,
            token::TokenKind::Integer(200),
            token::TokenKind::MINUS,
            token::TokenKind::EOF,
        ];

        let tokens = tokenize("100 + 200 -".to_string());
        assert!(tokens.is_ok());
        let tokens = tokens.unwrap();

        assert_eq!(
            expected,
            tokens
                .iter()
                .map(|t| t.kind)
                .collect::<Vec<token::TokenKind>>()
        );
    }

    #[test]
    fn scan_with_unimplemented_patterns() {
        let ctxt = tokenization();
        let (result, _length) = scan(&ctxt, "?");

        assert!(result.is_err());
    }

    #[test]
    fn scan_whitespace_with_blank_and_number() {
        let ctxt = tokenization();
        let (result, length) = scan_whitespace(&ctxt, "    100");
        assert_eq!(4, length);
        assert!(result.is_ok());

        assert_eq!(token::TokenKind::BLANK, result.unwrap().kind);
    }

    #[test]
    fn scan_symbol_with_invalid_input() {
        let ctxt = tokenization();
        let (result, length) = scan_symbol(&ctxt, "100");
        assert_eq!(0, length);
        assert!(result.is_err());
    }
    #[test]
    fn scan_symbol_with_valid_operator() {
        let ctxt = tokenization();
        let (result, length) = scan_symbol(&ctxt, "+");
        assert_eq!(1, length);
        assert!(result.is_ok());

        assert_eq!(token::TokenKind::PLUS, result.unwrap().kind);
    }

    #[test]
    fn scan_number_with_valid_string() {
        let ctxt = tokenization();
        let (result, length) = scan_number(&ctxt, "100");

        assert_eq!(3, length);
        assert!(result.is_ok());
        assert_eq!(token::TokenKind::Integer(100), result.unwrap().kind);
    }

    #[test]
    fn scan_number_with_invalid_string() {
        let ctxt = tokenization();
        let (result, length) = scan_number(&ctxt, "Drumato");

        assert_eq!(0, length);
        assert!(result.is_err());
    }

    fn tokenization() -> TokenizationContext {
        TokenizationContext {
            ..Default::default()
        }
    }
}

use crate::token;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct Token {
    pub kind: token::TokenKind,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum TokenKind {
    PLUS,
    MINUS,
    Integer(i128),
    BLANK,
    EOF,
}

impl TokenKind {
    pub fn should_ignore(&self) -> bool {
        match self {
            TokenKind::BLANK => true,
            _ => false,
        }
    }
}

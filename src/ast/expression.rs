#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Expr {
    Add { lhs: Box<Expr>, rhs: Box<Expr> },
    Sub { lhs: Box<Expr>, rhs: Box<Expr> },
    Integer { value: i128 },
}

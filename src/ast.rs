use crate::tokens::Token;
#[derive(Debug, PartialEq)]
pub enum Ast {
    Number(i64),
    UnaryOp,
    BinaryOp,
}

struct UnaryOp {
    op: Token,
    expr: Box<Ast>,
}

struct BinaryOp {
    left: Box<Ast>,
    op: Token,
    right: Box<Ast>,
}

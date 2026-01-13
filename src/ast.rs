use crate::tokens::Token;
#[derive(Debug, PartialEq)]
pub enum Ast {
    Number(i64),
    UnaryOp {
        op: Token,
        expr: Box<Ast>,
    },
    BinaryOp {
        left: Box<Ast>,
        op: Token,
        right: Box<Ast>,
    },
}

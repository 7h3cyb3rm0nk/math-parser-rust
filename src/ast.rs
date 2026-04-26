use crate::tokens::Token;
use display_tree::DisplayTree;
#[derive(Debug, PartialEq, DisplayTree)]
pub enum Ast {
    Number(i64),

    UnaryOp {
        #[node_label]
        op: Token,
        #[tree]
        expr: Box<Ast>,
    },
    BinaryOp {
        #[tree]
        left: Box<Ast>,
        #[node_label]
        op: Token,
        #[tree]
        right: Box<Ast>,
    },
}

impl std::fmt::Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ast::Number(n) => write!(f, "{n}"),
            Ast::UnaryOp { op, .. } => write!(f, "{:?}", op),
            Ast::BinaryOp { op, .. } => write!(f, "{:?}", op),
        }
    }
}

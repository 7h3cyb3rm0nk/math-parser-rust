#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Plus,
    Minus,
    Star,
    Slash,
    Power,
    Number(i64),
    ParanthesisOpen,
    ParanthesisClose,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Plus,
    Minus,
    Star,
    Slash,
    Power,
    Modulus,
    Number(i64),
    ParanthesisOpen,
    ParanthesisClose,
    Invalid,
}

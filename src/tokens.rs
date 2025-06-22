#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Plus,
    Minus,
    Star,
    Slash,
    Number(String),
    ParanthesisOpen,
    ParanthesisClose,
}

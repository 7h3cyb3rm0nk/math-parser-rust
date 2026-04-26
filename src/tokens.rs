use core::fmt;
use std::{
    fmt::Formatter,
    ops::{Add, BitXor, Div, Mul, Sub},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
    End,
}

impl Add for Token {
    type Output = Result<i64, String>;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(x), Self::Number(y)) => Ok(x + y),
            _ => Err("Invalid Operation".into()),
        }
    }
}

impl Sub for Token {
    type Output = Result<i64, String>;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(x), Self::Number(y)) => Ok(x - y),
            _ => Err("Invalid Operation".into()),
        }
    }
}

impl Mul for Token {
    type Output = Result<i64, String>;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(x), Self::Number(y)) => Ok(x * y),
            _ => Err("Invalid Operation".into()),
        }
    }
}

impl Div for Token {
    type Output = Result<i64, String>;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(x), Self::Number(y)) => Ok(x / y),
            _ => Err("Invalid Operation".into()),
        }
    }
}

impl BitXor for Token {
    type Output = Result<i64, String>;
    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(x), Self::Number(y)) => {
                if y < 0 {
                    Err("Negative Exponents Not Supported".into())
                } else {
                    Ok(x.pow(y as u32))
                }
            }
            _ => Err("Invalid Operation".into()),
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Modulus => write!(f, "%"),
            Token::Number(x) => write!(f, "{}", *x),
            Token::Power => write!(f, "^"),
            _ => write!(f, ""),
        }
    }
}

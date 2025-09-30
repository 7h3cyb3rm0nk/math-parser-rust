use std::ops::{Add, BitXor, Div, Mul, Sub};

#[derive(Debug, PartialEq, Eq, Clone)]
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

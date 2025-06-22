#![allow(unused)]

use std::fmt::Write;
use std::process::exit;
use std::sync::Mutex;

mod parser;
mod tokens;
use parser::Parser;
use tokens::Token;

fn main() {
    loop {
        let mut expression: String = String::new();
        std::io::stdin().read_line(&mut expression).unwrap();
        let mut tokens: Vec<char> = expression.chars().collect();
        if expression.len() == 2 && expression.starts_with('q') {
            return;
        }
        if expression.chars().any(|x| x.is_alphabetic()) {
            println!("invalid expression");
            continue;
        }
        let parser = Parser::from(expression.chars().collect());

        println!("{:?}", parser);
    }
}

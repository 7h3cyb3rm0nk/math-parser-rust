use crate::ast::Ast;
use crate::tokens::Token;
#[derive(Debug)]
pub struct Parser {
    pub pos: usize,
    pub tokens: Vec<Token>,
    pub ast: Option<Box<Ast>>,
}

impl Parser {
    #[allow(unused)]
    pub fn new() -> Self {
        Parser {
            ast: None,
            pos: 0,
            tokens: Vec::new(),
        }
    }
    pub fn from(input: &str) -> Self {
        let tokens = Parser::tokenize(input.chars().collect()).unwrap_or(vec![Token::Invalid]);
        Parser {
            pos: 0,
            tokens,
            ast: None,
        }
    }

    fn tokenize(input: Vec<char>) -> Result<Vec<Token>, String> {
        if input.is_empty() {
            return Ok(Vec::new());
        }
        let mut tokens: Vec<Token> = Vec::with_capacity(input.len());
        let mut i = 0;
        while i < input.len() {
            match input[i] {
                '+' => tokens.push(Token::Plus),
                '-' => tokens.push(Token::Minus),
                '/' => tokens.push(Token::Slash),
                '*' => tokens.push(Token::Star),
                '(' => tokens.push(Token::ParanthesisOpen),
                ')' => tokens.push(Token::ParanthesisClose),
                '^' => tokens.push(Token::Power),
                '%' => tokens.push(Token::Modulus),

                // logic to form number tokens
                '0'..='9' => {
                    let mut num = (input[i] as u32 - '0' as u32) as i64;
                    i += 1;

                    while i < input.len() && input[i].is_ascii_digit() {
                        num = num * 10 + (input[i] as u32 - '0' as u32) as i64;
                        i += 1;
                    }

                    tokens.push(Token::Number(num));
                    continue;
                }
                ' ' | '\n' => {}
                _ => {
                    eprintln!("error unexpected character '{}'", input[i]);
                    return Err(format!("unexpected character found: {}", input[i]));
                }
            }
            i += 1;
        }
        Ok(tokens)
    }

    pub fn parse(&mut self) {}

    fn parse_expr(&mut self) {}
    fn parse_factors(&mut self) {}
    fn parse_terms(&mut self) {}

    pub fn parse_term(node: &mut Box<Ast>) {}

    pub fn parse_factor(node: &mut Box<Ast>) {}

    pub fn parse_unary(node: &mut Box<Ast>) {}

    pub fn parse_primary(node: &mut Box<Ast>) {}
}

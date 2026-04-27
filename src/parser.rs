use std::rc::Rc;

use crate::ast::Ast;
use crate::tokens::Token;
#[derive(Debug)]
pub struct Parser {
    pub pos: usize,
    pub tokens: Vec<Token>,
    pub ast: Option<Rc<Ast>>,
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

    pub fn tokenize(input: Vec<char>) -> Result<Vec<Token>, String> {
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
        tokens.push(Token::End);
        Ok(tokens)
    }

    fn advance(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.pos);
        self.pos += 1;
        tok
    }
    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    pub fn parse(&mut self) -> Option<Rc<Ast>> {
        let ast = self.parse_expr()?;
        let ast_clone = Rc::new(ast);
        if let Some(Token::End) = self.current() {
            self.ast = Some(ast_clone.clone());
            Some(ast_clone)
        } else {
            self.ast = None;
            None
        }
    }

    fn parse_expr(&mut self) -> Option<Ast> {
        let mut node = self.parse_term()?;
        while let Some(Token::Plus) | Some(Token::Minus) = self.current() {
            let op = *self.advance()?;
            let rhs = self.parse_term()?;
            node = Ast::BinaryOp {
                left: Box::new(node),
                op,
                right: Box::new(rhs),
            };
        }

        Some(node)
    }
    fn parse_factor(&mut self) -> Option<Ast> {
        let mut node = self.parse_unary()?;
        if let Some(Token::Power) = self.current() {
            let op = *self.advance()?;
            let rhs = self.parse_factor()?;
            node = Ast::BinaryOp {
                left: Box::new(node),
                op,
                right: Box::new(rhs),
            }
        }
        Some(node)
    }
    fn parse_term(&mut self) -> Option<Ast> {
        let mut node = self.parse_factor()?;

        while let Some(Token::Star) | Some(Token::Slash) | Some(Token::Modulus) = self.current() {
            let op = *self.advance()?;
            let rhs = self.parse_factor()?;
            node = Ast::BinaryOp {
                left: Box::new(node),
                op,
                right: Box::new(rhs),
            };
        }

        Some(node)
    }
    fn parse_unary(&mut self) -> Option<Ast> {
        match self.current() {
            Some(Token::Plus) | Some(Token::Minus) => {
                let op = *self.advance()?;
                let expr = self.parse_unary()?;
                Some(Ast::UnaryOp {
                    op,
                    expr: Box::new(expr),
                })
            }
            _ => self.parse_primary(),
        }
    }
    fn parse_primary(&mut self) -> Option<Ast> {
        match self.current() {
            Some(Token::Number(x)) => {
                let val = *x;
                self.advance()?;
                Some(Ast::Number(val))
            }
            Some(Token::ParanthesisOpen) => {
                self.advance();
                let expr = self.parse_expr()?;
                match self.current() {
                    Some(Token::ParanthesisClose) => {
                        self.advance();
                        Some(expr)
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }
    #[allow(dead_code)]
    fn eval(&self) -> Option<i64> {
        None
    }
}

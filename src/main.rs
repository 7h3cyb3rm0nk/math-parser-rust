#![allow(dead_code, unused_variables)]

use std::io::{self, BufRead, Write, stdout};
mod ast;
mod parser;
mod tokens;
use parser::Parser;
use tokens::Token;

fn main() -> io::Result<()> {
    let mut input: String = String::new();
    let mut handle = std::io::stdin().lock();
    let n1 = Token::Number(1);
    let n2 = Token::Number(2);
    println!("{}", (n1 + n2).unwrap());
    loop {
        print!("> ");
        stdout().flush()?;
        input.clear();
        let n = handle.read_line(&mut input)?;
        if n == 0 {
            continue;
        }
        let expression = input.trim();
        if expression == "q" {
            return Ok(());
        }
        if expression.chars().any(|x| x.is_alphabetic()) {
            println!("invalid expression");
            continue;
        }
        let mut parser = Parser::from(expression);
        let value = parser.parse();

        println!("{:#?}", value.unwrap());
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::Ast;
    use crate::parser::Parser;
    use crate::tokens::Token;
    #[test]
    fn check_equal_tokens() {
        let char_vec = "900+90*9/20";
        let token_vec: Vec<Token> = vec![
            Token::Number(900),
            Token::Plus,
            Token::Number(90),
            Token::Star,
            Token::Number(9),
            Token::Slash,
            Token::Number(20),
        ];
        let parser: Parser = Parser::from(char_vec);
        assert_eq!(parser.tokens, token_vec);
    }
    #[test]
    fn check_different_tokens() {
        let char_vec = "900+90*3";
        let token_vec: Vec<Token> = vec![
            Token::Number(900),
            Token::Plus,
            Token::Number(90),
            Token::Star,
            Token::Number(9),
            Token::Slash,
            Token::Number(20),
        ];
        let parser: Parser = Parser::from(char_vec);
        assert_ne!(parser.tokens, token_vec);
    }

    // #[test]
    // fn check_ast_trees() {
    //     panic!()
    // }
    //
    #[test]
    fn test_single_number_parsing() {
        let single_num = "9";
        let ast = Box::new(Ast::Number(9_i64));
        let mut parser = Parser::from(single_num);
        parser.parse();
        assert_eq!(Some(ast), parser.ast)
    }
    #[test]
    fn test_ast_tree_add() {
        let expr = "9+2";
        let ast: Box<Ast> = Box::new(Ast::BinaryOp {
            left: Box::new(Ast::Number(9_i64)),
            op: Token::Plus,
            right: Box::new(Ast::Number(2_i64)),
        });
        let mut parser = Parser::from(expr);
        parser.parse();
        assert_eq!(Some(ast), parser.ast)
    }
}

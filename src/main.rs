#![allow(unused_variables, unused_imports)]

use std::io::{self, BufRead, Write, stdout};
mod ast;
mod parser;
mod tokens;
use display_tree::Color;
use display_tree::{AsTree, CharSet, StyleBuilder};
use owo_colors::{OwoColorize, Style};
use parser::Parser;
fn main() -> io::Result<()> {
    let mut input: String = String::new();
    let mut handle = std::io::stdin().lock();
    loop {
        print!("> ");
        stdout().flush()?;
        input.clear();
        let n = handle.read_line(&mut input)?;
        if n == 0 {
            continue;
        }
        let expression = input.trim();
        if expression == "q" || expression == "quit" {
            return Ok(());
        }
        if expression.chars().any(|x| x.is_alphabetic()) {
            println!("invalid expression");
            continue;
        }
        let mut parser = Parser::from(expression);
        let value = parser.parse();
        let owo_color_style = Style::new()
            .bright_red()
            .on_truecolor(142, 179, 227)
            .bold()
            .blink();
        if let Some(ast_tree) = value {
            let tree = format!(
                "{}",
                AsTree::new(ast_tree.as_ref())
                    .char_set(CharSet::SINGLE_LINE_BOLD)
                    .leaf_color(Color::Green)
                    .branch_color(Color::White)
            );
            println!("{}", tree);
            println!("evaluated value => {}", ast_tree.eval().unwrap());
        } else {
            println!("wait that's illegal!");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

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
            Token::End,
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
        let ast = Ast::Number(9_i64);
        let mut parser = Parser::from(single_num);
        parser.parse();
        assert_eq!((Some(Rc::new(ast))), parser.ast)
    }
    #[test]
    fn test_ast_tree_add() {
        let expr = "9+2";
        let ast: Ast = Ast::BinaryOp {
            left: Box::new(Ast::Number(9_i64)),
            op: Token::Plus,
            right: Box::new(Ast::Number(2_i64)),
        };
        let mut parser = Parser::from(expr);
        parser.parse();
        assert_eq!(Some(Rc::new(ast)), parser.ast)
    }
    #[test]
    fn test_ast_tree_cloning() {
        let ast: Ast = Ast::BinaryOp {
            left: Box::new(Ast::Number(9_i64)),
            op: Token::Plus,
            right: Box::new(Ast::Number(2_i64)),
        };
        let ast_cloned = ast.clone();
        assert_eq!(ast, ast_cloned)
    }
}

use std::io::{self, stdout, BufRead, Write};
mod parser;
mod tokens;
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
        if expression == "q" {
            return Ok(());
        }
        if expression.chars().any(|x| x.is_alphabetic()) {
            println!("invalid expression");
            continue;
        }
        let parser = Parser::from(expression);

        println!("{:?}", parser.tokens);
    }
}

#[cfg(test)]
mod tests {
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

    #[test]
    fn check_ast_trees() {
        panic!()
    }
}

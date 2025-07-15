mod parser;
mod tokens;
use parser::Parser;

fn main() {
    loop {
        let mut expression: String = String::new();
        std::io::stdin().read_line(&mut expression).unwrap();
        if expression.len() == 2 && expression.starts_with('q') {
            return;
        }
        if expression.chars().any(|x| x.is_alphabetic()) {
            println!("invalid expression");
            continue;
        }
        let parser = Parser::from(expression.chars().collect());

        println!("{:?}", parser.tokens);
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;
    use crate::tokens::Token;
    #[test]
    fn check_equal_tokens() {
        let char_vec: Vec<char> = vec!['9', '0', '0', '+', '9', '0', '*', '9', '/', '2', '0'];
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
        let char_vec: Vec<char> = vec!['9', '0', '+', '9', '0', '*', '9', '/', '2'];
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

use crate::tokens::Token;
#[derive(Debug)]
pub struct Parser {
    last_token: Option<char>,
    last_token_type: Option<Token>,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            last_token: None,
            last_token_type: None,
            tokens: Vec::new(),
        }
    }
    pub fn from(tokens: Vec<char>) -> Self {
        let mut tokens = Parser::tokenize(tokens);
        Parser {
            last_token: None,
            last_token_type: None,
            tokens,
        }
    }

    pub fn tokenize(tokens: Vec<char>) -> Vec<Token> {
        let mut token_vec: Vec<Token> = vec![];
        let mut token_iter = tokens.iter().peekable();
        while let Some(&current_char) = token_iter.next() {
            match current_char {
                '+' => token_vec.push(Token::Plus),
                '-' => token_vec.push(Token::Minus),
                '/' => token_vec.push(Token::Slash),
                '*' => token_vec.push(Token::Star),
                '(' => token_vec.push(Token::ParanthesisOpen),
                ')' => token_vec.push(Token::ParanthesisClose),
                x if x.is_ascii_digit() => {
                    let mut digit_str: String = x.to_string();
                    while let Some(&next_char) = token_iter.next_if(|&x| x.is_ascii_digit()) {
                        digit_str.push(next_char);
                    }
                    token_vec.push(Token::Number(digit_str))
                }
                _ => {}
            }
        }
        token_vec
    }
}

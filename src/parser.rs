use crate::tokens::Token;
#[derive(Debug)]
pub struct Parser {
    _last_token: Option<char>,
    _last_token_type: Option<Token>,
    pub tokens: Vec<Token>,
}

impl Parser {
    #[allow(unused)]
    pub fn new() -> Self {
        Parser {
            _last_token: None,
            _last_token_type: None,
            tokens: Vec::new(),
        }
    }
    pub fn from(tokens: Vec<char>) -> Self {
        let tokens = Parser::tokenize(tokens).unwrap_or(vec![Token::Invalid]);
        Parser {
            _last_token: None,
            _last_token_type: None,
            tokens,
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
}

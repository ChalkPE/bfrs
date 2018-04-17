#[derive(Debug)]
pub enum Token {
    Next,
    Prev,
    Add,
    Sub,
    Open,
    Close,
    Read,
    Write
}

pub fn tokenize(code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    for c in code.chars() {
        match c {
            '>' => tokens.push(Token::Next),
            '<' => tokens.push(Token::Prev),
            '+' => tokens.push(Token::Add),
            '-' => tokens.push(Token::Sub),
            ',' => tokens.push(Token::Read),
            '.' => tokens.push(Token::Write),
            '[' => tokens.push(Token::Open),
            ']' => tokens.push(Token::Close),
            _ => ()
        }
    }

    tokens
}

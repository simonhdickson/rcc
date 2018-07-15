use std::str::Chars;
use std::iter::Peekable;

#[derive(PartialEq, Debug)]
pub enum Token {
    LeftBrace,
    RightBrace,
    LeftParenthesis,
    RightParenthesis,
    SemiColon,
    Return,
    Integer,
    Ident(String),
    LiteralInteger(String)
}

pub fn parse(content: &str) -> Vec<Token> {
    let mut chars = content.chars().peekable();
    let mut tokens = Vec::new();

    loop {
        skip_whitespace(&mut chars);

        match next_token(&mut chars) {
            Some(token) => tokens.push(token),
            None => break,
        }
    }

    tokens
}

fn next_token(chars: &mut Peekable<Chars>) -> Option<Token> {
    match chars.next() {
        Some('{') => Some(Token::LeftBrace),
        Some('}') => Some(Token::RightBrace),
        Some('(') => Some(Token::LeftParenthesis),
        Some(')') => Some(Token::RightParenthesis),
        Some(';') => Some(Token::SemiColon),
        Some(ch @ _) => {
            if ch.is_alphabetic() {
                let literal = read_identifier(chars, ch);
                Some(parse_ident(&literal))
            } else if ch.is_numeric() {
                let i = read_number(chars, ch);
                Some(Token::LiteralInteger(i))
            } else {
                None
            }
        }
        _ => None
    }
}

fn parse_ident(ident: &str) -> Token {
    match ident {
        "return" => Token::Return,
        "int" => Token::Integer,
        ident @ _ => Token::Ident(ident.to_owned())
    }
}

fn read_identifier(chars: &mut Peekable<Chars>, first: char) -> String {
    let mut ident = String::new();
    ident.push(first);

    while peek_is_letter(chars) {
        ident.push(chars.next().unwrap());
    }

    ident
}

fn skip_whitespace(chars: &mut Peekable<Chars>) {
    while let Some(&c) = chars.peek() {
        if !c.is_whitespace() {
            break;
        }
        chars.next();
    }
}

fn peek_is_letter(chars: &mut Peekable<Chars>) -> bool {
    match chars.peek() {
        Some(&ch) => ch.is_alphabetic(),
        None => false,
    }
}

fn read_number(chars: &mut Peekable<Chars>, first: char) -> String {
    let mut number = String::new();
    number.push(first);

    while let Some(&c) = chars.peek() {
        if !c.is_numeric() {
            break;
        }
        number.push(chars.next().unwrap());
    }

    number
}
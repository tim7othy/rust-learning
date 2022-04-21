use std::iter::Peekable;
use std::str::Chars;

use super::token::Token;


pub struct Tokenizer<'a> {
    expr: Peekable<Chars<'a>>
}

impl<'a> Tokenizer<'a> {
    pub fn new(s: &'a str) -> Self {
        Tokenizer {
            expr: s.chars().peekable(),
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let next_char = self.expr.next();
        match next_char {
            Some('0'..='9') => {
                None
            },
            Some('+') => Some(Token::Add),
            None => Some(Token::EOF),
            Some(_) => None,
        }
    }
}
use std::{iter::Peekable, str::Chars};

use itertools::PeekingNext;

use crate::Token;

pub struct Scanner<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Scanner<'a> {
    #[must_use]
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
        }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(c) = self.input.next() {
            let token = match c {
                c if c.is_ascii_whitespace() => continue,
                '(' => Token::LEFT_PAREN,
                ')' => Token::RIGHT_PAREN,
                '{' => Token::LEFT_BRACE,
                '}' => Token::RIGHT_BRACE,
                ';' => Token::SEMICOLON,
                ',' => Token::COMMA,
                '+' => Token::PLUS,
                '-' => Token::MINUS,
                '*' => Token::STAR,
                '/' => Token::SLASH,
                '.' => Token::DOT,

                '!' => self
                    .input
                    .peeking_next(|x| *x == '=')
                    .map_or(Token::BANG, |_| Token::BANG_EQUAL),
                '=' => self
                    .input
                    .peeking_next(|x| *x == '=')
                    .map_or(Token::EQUAL, |_| Token::EQUAL_EQUAL),
                '<' => self
                    .input
                    .peeking_next(|x| *x == '=')
                    .map_or(Token::LESS, |_| Token::LESS_EQUAL),
                '>' => self
                    .input
                    .peeking_next(|x| *x == '=')
                    .map_or(Token::GREATER, |_| Token::GREATER_EQUAL),
                _ => unimplemented!(),
            };
            return Some(token);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn punctuators() {
        let input = "(){};,+-*!!====<=>=!=<>/.";
        let mut scanner = Scanner::new(input);
        let mut next_token = || scanner.next().unwrap();
        assert_eq!(next_token(), Token::LEFT_PAREN);
        assert_eq!(next_token(), Token::RIGHT_PAREN);
        assert_eq!(next_token(), Token::LEFT_BRACE);
        assert_eq!(next_token(), Token::RIGHT_BRACE);
        assert_eq!(next_token(), Token::SEMICOLON);
        assert_eq!(next_token(), Token::COMMA);
        assert_eq!(next_token(), Token::PLUS);
        assert_eq!(next_token(), Token::MINUS);
        assert_eq!(next_token(), Token::STAR);
        assert_eq!(next_token(), Token::BANG);
        assert_eq!(next_token(), Token::BANG_EQUAL);
        assert_eq!(next_token(), Token::EQUAL_EQUAL);
        assert_eq!(next_token(), Token::EQUAL);
        assert_eq!(next_token(), Token::LESS_EQUAL);
        assert_eq!(next_token(), Token::GREATER_EQUAL);
        assert_eq!(next_token(), Token::BANG_EQUAL);
        assert_eq!(next_token(), Token::LESS);
        assert_eq!(next_token(), Token::GREATER);
        assert_eq!(next_token(), Token::SLASH);
        assert_eq!(next_token(), Token::DOT);
        assert!(scanner.next().is_none());
    }

    // TODO
    #[test]
    fn comment() {
        let input = "\
        // (){};,+-*!!====<=>=!=<>/.\n\
        ()";
        let mut scanner = Scanner::new(input);
        let mut next_token = || scanner.next().unwrap();
        assert_eq!(next_token(), Token::LEFT_PAREN);
        assert_eq!(next_token(), Token::RIGHT_PAREN);
        assert!(scanner.next().is_none());
    }
}

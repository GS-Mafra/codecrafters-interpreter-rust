use std::{iter::Peekable, str::Chars};

use itertools::{Itertools, PeekingNext};
use thiserror::Error;

use crate::Token;

#[derive(Debug, Error)]
pub enum Error {
    #[error("[line {line}] Error: Unexpected character: {c}")]
    Lexical { line: usize, c: char },
}

impl Error {
    #[inline]
    pub(crate) const fn lexical(line: usize, c: char) -> Self {
        Self::Lexical { line, c }
    }
}

pub struct Scanner<'a> {
    input: Peekable<Chars<'a>>,
    eof: bool,
    line: usize,
}

impl<'a> Scanner<'a> {
    #[must_use]
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
            eof: false,
            line: 1,
        }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Result<Token<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.eof {
            return None;
        }

        while let Some(c) = self.input.next() {
            let token = match c {
                '\t' | '\x0C' | '\r' | ' ' => continue,
                '\n' => {
                    self.line += 1;
                    continue;
                }

                '(' => Token::LEFT_PAREN,
                ')' => Token::RIGHT_PAREN,
                '{' => Token::LEFT_BRACE,
                '}' => Token::RIGHT_BRACE,
                ';' => Token::SEMICOLON,
                ',' => Token::COMMA,
                '+' => Token::PLUS,
                '-' => Token::MINUS,
                '*' => Token::STAR,
                '.' => Token::DOT,

                '/' => {
                    if self.input.peeking_next(|c| *c == '/').is_some() {
                        self.input
                            .peeking_take_while(|c| *c != '\n')
                            .for_each(|_| ());
                        continue;
                    }
                    Token::SLASH
                }
                '!' => self
                    .input
                    .peeking_next(|c| *c == '=')
                    .map_or(Token::BANG, |_| Token::BANG_EQUAL),
                '=' => self
                    .input
                    .peeking_next(|c| *c == '=')
                    .map_or(Token::EQUAL, |_| Token::EQUAL_EQUAL),
                '<' => self
                    .input
                    .peeking_next(|c| *c == '=')
                    .map_or(Token::LESS, |_| Token::LESS_EQUAL),
                '>' => self
                    .input
                    .peeking_next(|c| *c == '=')
                    .map_or(Token::GREATER, |_| Token::GREATER_EQUAL),
                c => return Some(Err(Error::lexical(self.line, c))),
            };
            return Some(Ok(token));
        }
        self.eof = true;
        Some(Ok(Token::EOF))
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
        let mut next_token = || scanner.next().unwrap().unwrap();
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
        assert_eq!(next_token(), Token::EOF);
        assert!(scanner.next().is_none());
    }

    #[test]
    fn comment() {
        let input = "\
        // comment (){};,+-*!!====<=>=!=<>/.\n\
        ()\n\
        // comment\n\
        // comment\n\
        *";
        let mut scanner = Scanner::new(input);
        let mut next_token = || scanner.next().unwrap().unwrap();
        assert_eq!(next_token(), Token::LEFT_PAREN);
        assert_eq!(next_token(), Token::RIGHT_PAREN);
        assert_eq!(next_token(), Token::STAR);
        assert_eq!(next_token(), Token::EOF);
        assert!(scanner.next().is_none());
    }
}

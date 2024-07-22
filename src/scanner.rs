use std::{iter::Peekable, str::CharIndices};

use itertools::{Itertools, PeekingNext};
use thiserror::Error;

use crate::{Literal, Token, Type};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error("[line {line}] Error: {kind}")]
    Lexical { line: usize, kind: LexicalKind },
}

impl Error {
    #[inline]
    pub(crate) const fn lexical(line: usize, kind: LexicalKind) -> Self {
        Self::Lexical { line, kind }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum LexicalKind {
    #[error("Unexpected character: {0}")]
    UnexpectedCharacter(char),
    #[error("Unterminated string.")]
    UnterminatedString,
}

pub struct Scanner<'a> {
    raw: &'a str,
    chars: Peekable<CharIndices<'a>>,
    eof: bool,
    line: usize,
}

impl<'a> Scanner<'a> {
    #[must_use]
    pub fn new(input: &'a str) -> Self {
        Self {
            raw: input, // FIXME no Chars::as_str() for peekable :( https://github.com/rust-lang/rust/issues/33881
            chars: input.char_indices().peekable(),
            eof: false,
            line: 1,
        }
    }

    fn string(&mut self, start: usize) -> Result<Token<'a>, Error> {
        let start_line = self.line;
        let end = self
            .chars
            .by_ref()
            .inspect(|(_, c)| {
                if *c == '\n' {
                    self.line += 1;
                }
            })
            .position(|(_, c)| c == '"')
            .map(|pos| start + pos + 1);

        let Some(end) = end else {
            return Err(Error::lexical(start_line, LexicalKind::UnterminatedString));
        };
        let lexeme = &self.raw[start..=end];
        let literal = &self.raw[start + 1..end];
        Ok(Token::new(Type::String, lexeme, Literal::String(literal)))
    }

    fn num(&mut self, start: usize) -> Token<'a> {
        let mut seen_dot = false;
        let mut end = start;

        while let Some((_, c)) = self.chars.peek() {
            match c {
                c if c.is_ascii_digit() => (),
                '.' => {
                    let is_next_digit = self
                        .raw
                        .as_bytes()
                        .get(end + 2)
                        .is_some_and(u8::is_ascii_digit);
                    if seen_dot || !is_next_digit {
                        break;
                    }
                    seen_dot = true;
                }
                _ => break,
            }
            self.chars.next();
            end += 1;
        }

        let num = &self.raw[start..=end];
        Token::new(
            Type::Number,
            num,
            Literal::Number(num.parse::<f64>().unwrap()),
        )
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Result<Token<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.eof {
            return None;
        }

        while let Some((i, c)) = self.chars.next() {
            let token = match c {
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

                '\t' | '\x0C' | '\r' | ' ' => continue,
                '\n' => {
                    self.line += 1;
                    continue;
                }

                c if c.is_ascii_digit() => return Some(Ok(self.num(i))),
                '"' => return Some(self.string(i)),

                '/' => {
                    if self.chars.peeking_next(|(_, c)| *c == '/').is_some() {
                        self.chars
                            .peeking_take_while(|(_, c)| *c != '\n')
                            .for_each(|_| ());
                        continue;
                    }
                    Token::SLASH
                }
                '!' => self
                    .chars
                    .peeking_next(|(_, c)| *c == '=')
                    .map_or(Token::BANG, |_| Token::BANG_EQUAL),
                '=' => self
                    .chars
                    .peeking_next(|(_, c)| *c == '=')
                    .map_or(Token::EQUAL, |_| Token::EQUAL_EQUAL),
                '<' => self
                    .chars
                    .peeking_next(|(_, c)| *c == '=')
                    .map_or(Token::LESS, |_| Token::LESS_EQUAL),
                '>' => self
                    .chars
                    .peeking_next(|(_, c)| *c == '=')
                    .map_or(Token::GREATER, |_| Token::GREATER_EQUAL),
                c => {
                    return Some(Err(Error::lexical(
                        self.line,
                        LexicalKind::UnexpectedCharacter(c),
                    )))
                }
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

    #[test]
    fn unterminated() {
        let input = "\
        *\n\
        // comment\n\
        \"*string*\"\n\
        \"unterminated\n\
        ***\n\
        ";
        let mut scanner = Scanner::new(input);
        let mut next_token = || scanner.next().unwrap();
        assert_eq!(next_token().unwrap(), Token::STAR);
        assert_eq!(
            next_token().unwrap(),
            Token::new(Type::String, "\"*string*\"", Literal::String("*string*"))
        );
        assert_eq!(
            next_token(),
            Err(Error::lexical(4, LexicalKind::UnterminatedString))
        );
        assert_eq!(next_token().unwrap(), Token::EOF);
        assert!(scanner.next().is_none());
    }

    #[test]
    fn multi_line() {
        let multi_line = "\
        \"\n\
            multi\n\
            line\n\
            string\n\
        \"";

        let input = format!(
            "{multi_line}
        $\n\
        *"
        );

        let mut scanner = Scanner::new(&input);
        let mut next_token = || scanner.next().unwrap();
        assert_eq!(
            next_token().unwrap(),
            Token::new(
                Type::String,
                multi_line,
                Literal::String(&multi_line[1..multi_line.len() - 1])
            )
        );

        assert_eq!(
            next_token(),
            Err(Error::lexical(6, LexicalKind::UnexpectedCharacter('$')))
        );
        assert_eq!(next_token().unwrap(), Token::STAR);
        assert_eq!(next_token().unwrap(), Token::EOF);
        assert!(scanner.next().is_none());
    }

    #[test]
    fn num() {
        let input = "\
        1234.1234.1234.\n\
        ";

        let mut scanner = Scanner::new(input);
        let mut next_token = || scanner.next().unwrap().unwrap();

        assert_eq!(next_token(), Token::new(Type::Number, "1234.1234", Literal::Number(1234.1234_f64)));
        assert_eq!(next_token(), Token::DOT);
        assert_eq!(next_token(), Token::new(Type::Number, "1234", Literal::Number(1234_f64)));
        assert_eq!(next_token(), Token::DOT);
        assert_eq!(next_token(), Token::EOF);
    }
}

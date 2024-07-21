use std::io::{self, Write};

use crate::Scanner;

pub struct Tokenizer<'a, O, E> {
    scanner: Scanner<'a>,
    out: &'a mut O,
    err: &'a mut E,
    pub code: i32,
}

impl<'a, O, E> Tokenizer<'a, O, E>
where
    O: Write,
    E: Write,
{
    pub fn new(input: &'a str, out: &'a mut O, err: &'a mut E) -> Self {
        let scanner = Scanner::new(input);
        Self {
            scanner,
            out,
            err,
            code: 0,
        }
    }

    pub fn tokenize(&mut self) -> io::Result<()> {
        for token in self.scanner.by_ref() {
            match token {
                Ok(token) => writeln!(self.out, "{token}")?,
                Err(e) => {
                    self.code = 65; // TODO
                    writeln!(self.err, "{e}")?;
                }
            };
        }

        self.out.flush()?;
        self.err.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{scanner, Token};

    use pretty_assertions::assert_eq;

    #[test]
    fn out() {
        let mut buf = Vec::new();

        let input = ",.$(#";

        Tokenizer::new(input, &mut buf, &mut io::sink())
            .tokenize()
            .unwrap();

        let expected = {
            let mut buf = Vec::new();
            let tokens = [Token::COMMA, Token::DOT, Token::LEFT_PAREN, Token::EOF];
            for token in tokens {
                writeln!(buf, "{token}").unwrap();
            }
            buf
        };
        assert_eq!(buf, expected);
    }

    #[test]
    fn err() {
        let mut buf = Vec::new();

        let input = "\
        $\n\
        #\n\
        ";

        Tokenizer::new(input, &mut io::sink(), &mut buf)
            .tokenize()
            .unwrap();

        let expected = {
            let mut buf = Vec::new();
            let errors = [
                scanner::Error::lexical(1, '$'),
                scanner::Error::lexical(2, '#'),
            ];
            for error in errors {
                writeln!(buf, "{error}").unwrap();
            }
            buf
        };
        assert_eq!(buf, expected);
    }

    #[test]
    fn out_err() {
        let mut out_buf = Vec::new();
        let mut err_buf = Vec::new();

        let input = "\
        ,\n\
        .\n\
        $\n\
        (\n\
        #\n\
        ";

        Tokenizer::new(input, &mut out_buf, &mut err_buf)
            .tokenize()
            .unwrap();

        {
            let expected_out = {
                let mut buf = Vec::new();
                let tokens = [Token::COMMA, Token::DOT, Token::LEFT_PAREN, Token::EOF];
                for token in tokens {
                    writeln!(buf, "{token}").unwrap();
                }
                buf
            };
            assert_eq!(out_buf, expected_out);
        }

        {
            let expected_err = {
                let mut buf = Vec::new();
                let errors = [
                    scanner::Error::lexical(3, '$'),
                    scanner::Error::lexical(5, '#'),
                ];
                for error in errors {
                    writeln!(buf, "{error}").unwrap();
                }
                buf
            };
            assert_eq!(err_buf, expected_err);
        }
    }
}

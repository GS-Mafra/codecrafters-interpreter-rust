#![allow(clippy::missing_errors_doc)]

mod scanner;
pub use scanner::Scanner;

mod token;
pub use token::Token;
pub(crate) use token::{Literal, Type};

mod tokenize;
pub use tokenize::Tokenizer;

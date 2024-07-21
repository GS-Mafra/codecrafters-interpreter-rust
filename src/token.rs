use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct Token<'a> {
    pub(crate) ty: Type,
    pub(crate) lexeme: &'a str,
    pub(crate) literal: Literal<'a>,
}

impl<'a> Token<'a> {
    pub const LEFT_PAREN: Self = Self::new_null(Type::LeftParen, "(");
    pub const RIGHT_PAREN: Self = Self::new_null(Type::RightParen, ")");
    pub const LEFT_BRACE: Self = Self::new_null(Type::LeftBrace, "{");
    pub const RIGHT_BRACE: Self = Self::new_null(Type::RightBrace, "}");
    pub const SEMICOLON: Self = Self::new_null(Type::Semicolon, ";");
    pub const COMMA: Self = Self::new_null(Type::Comma, ",");
    pub const PLUS: Self = Self::new_null(Type::Plus, "+");
    pub const MINUS: Self = Self::new_null(Type::Minus, "-");
    pub const STAR: Self = Self::new_null(Type::Star, "*");
    pub const BANG: Self = Self::new_null(Type::Bang, "!");
    pub const EQUAL: Self = Self::new_null(Type::Equal, "=");
    pub const LESS: Self = Self::new_null(Type::Less, "<");
    pub const GREATER: Self = Self::new_null(Type::Greater, ">");
    pub const SLASH: Self = Self::new_null(Type::Slash, "/");
    pub const DOT: Self = Self::new_null(Type::Dot, ".");
    pub const BANG_EQUAL: Self = Self::new_null(Type::BangEqual, "!=");
    pub const EQUAL_EQUAL: Self = Self::new_null(Type::EqualEqual, "==");
    pub const LESS_EQUAL: Self = Self::new_null(Type::LessEqual, "<=");
    pub const GREATER_EQUAL: Self = Self::new_null(Type::GreaterEqual, ">=");
    pub const EOF: Self = Self::new_null(Type::Eof, "");

    #[inline]
    pub(crate) const fn new(ty: Type, lexeme: &'a str, literal: Literal<'a>) -> Self {
        Self {
            ty,
            lexeme,
            literal,
        }
    }

    #[inline]
    pub(crate) const fn new_null(ty: Type, lexeme: &'a str) -> Self {
        Self::new(ty, lexeme, Literal::Null)
    }
}

#[derive(Debug, PartialEq, Eq, strum_macros::Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Type {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    Comma,
    Plus,
    Minus,
    Star,
    Bang,
    Equal,
    Less,
    Greater,
    Slash,
    Dot,

    BangEqual,
    EqualEqual,
    LessEqual,
    GreaterEqual,

    Identifier,
    String,

    Eof,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Literal<'a> {
    Null,
    String(&'a str),
    Number(i64), // FIXME
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.ty, self.lexeme, self.literal)
    }
}

impl<'a> Display for Literal<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Null => write!(f, "null"),
            Self::String(x) => x.fmt(f),
            Self::Number(x) => x.fmt(f),
        }
    }
}

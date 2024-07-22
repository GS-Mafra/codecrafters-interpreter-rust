use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy)]
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

    pub const AND: Self = Self::new_null(Type::And, "AND");
    pub const CLASS: Self = Self::new_null(Type::Class, "CLASS");
    pub const ELSE: Self = Self::new_null(Type::Else, "ELSE");
    pub const FALSE: Self = Self::new_null(Type::False, "FALSE");
    pub const FUN: Self = Self::new_null(Type::Fun, "FUN");
    pub const FOR: Self = Self::new_null(Type::For, "FOR");
    pub const IF: Self = Self::new_null(Type::If, "IF");
    pub const NIL: Self = Self::new_null(Type::Nil, "NIL");
    pub const OR: Self = Self::new_null(Type::Or, "OR");
    pub const PRINT: Self = Self::new_null(Type::Print, "PRINT");
    pub const RETURN: Self = Self::new_null(Type::Return, "RETURN");
    pub const SUPER: Self = Self::new_null(Type::Super, "SUPER");
    pub const THIS: Self = Self::new_null(Type::This, "THIS");
    pub const TRUE: Self = Self::new_null(Type::True, "TRUE");
    pub const VAR: Self = Self::new_null(Type::Var, "VAR");
    pub const WHILE: Self = Self::new_null(Type::While, "WHILE");

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

    #[inline]
    pub(crate) fn reserved() -> &'static phf::Map<&'static str, Self> {
        static RESERVED: phf::Map<&'static str, Token> = phf::phf_map! {
            "and" => Token::AND,
            "class" => Token::CLASS,
            "else" => Token::ELSE,
            "false" => Token::FALSE,
            "for" => Token::FOR,
            "fun" => Token::FUN,
            "if" => Token::IF,
            "nil" => Token::NIL,
            "or" => Token::OR,
            "print" => Token::PRINT,
            "return" => Token::RETURN,
            "super" => Token::SUPER,
            "this" => Token::THIS,
            "true" => Token::TRUE,
            "var" => Token::VAR,
            "while" => Token::WHILE,
        };
        &RESERVED
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, strum_macros::Display)]
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
    Number,

    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Literal<'a> {
    Null,
    String(&'a str),
    Number(f64),
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
            Self::Number(x) => std::fmt::Debug::fmt(x, f),
        }
    }
}

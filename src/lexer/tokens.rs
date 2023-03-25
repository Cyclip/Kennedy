use super::Span;
use std::fmt;

/// A lexical unit, representing a single token in the source code.
#[derive(Debug, Clone, PartialEq)]
#[allow(unused)]
pub enum TokenType {
    // Single-character tokens
    LeftParen, RightParen, LeftBrace, RightBrace,       // ( ) { }
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,    // , . - + ; / *
    Colon,                                              // :
    // One or two character tokens
    Bang, BangEqual,                                  // ! !=
    Equal, EqualEqual,                                // = ==
    Greater, GreaterEqual,                            // > >=
    Less, LessEqual,                                  // < <=
    PlusPlus, MinusMinus,                             // ++ --
    StarStar, SlashSlash,                             // ** //
    PlusEqual, MinusEqual,                            // += -=
    StarEqual, SlashEqual,                            // *= /=
    // Literals
    StringLiteral(String),                            // "..."
    IntegerLiteral(i64),                              // 123
    FloatLiteral(f64),                                // 123.456
    Ident(String),                               // ...
    // Keywords
    Function, Let, If, Else, While, Return,            // function let if else while return
    True, False,                                      // true false
    For, Do, Until,                                   // for do until
    Or, And, Not,                                     // or and not
    // Types
    Int, Float, Bool, String, Null,                   // int float bool string null
    // End of file
    Eof,
}

/// Lexical unit with a span
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub span: Span,
}

impl Token {
    pub fn new(token_type: TokenType, span: Span) -> Self {
        Self { token_type, span }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {:?}", self.token_type, self.span)
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::LeftParen => write!(f, "("),
            TokenType::RightParen => write!(f, ")"),
            TokenType::LeftBrace => write!(f, "{{"),
            TokenType::RightBrace => write!(f, "}}"),
            TokenType::Comma => write!(f, ","),
            TokenType::Dot => write!(f, "."),
            TokenType::Minus => write!(f, "-"),
            TokenType::Plus => write!(f, "+"),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::Slash => write!(f, "/"),
            TokenType::Star => write!(f, "*"),
            TokenType::Colon => write!(f, ":"),
            TokenType::Bang => write!(f, "!"),
            TokenType::BangEqual => write!(f, "!="),
            TokenType::Equal => write!(f, "="),
            TokenType::EqualEqual => write!(f, "=="),
            TokenType::Greater => write!(f, ">"),
            TokenType::GreaterEqual => write!(f, ">="),
            TokenType::Less => write!(f, "<"),
            TokenType::LessEqual => write!(f, "<="),
            TokenType::PlusPlus => write!(f, "++"),
            TokenType::MinusMinus => write!(f, "--"),
            TokenType::StarStar => write!(f, "**"),
            TokenType::SlashSlash => write!(f, "//"),
            TokenType::PlusEqual => write!(f, "+="),
            TokenType::MinusEqual => write!(f, "-="),
            TokenType::StarEqual => write!(f, "*="),
            TokenType::SlashEqual => write!(f, "/="),
            TokenType::StringLiteral(string) => write!(f, "{:?}", string),
            TokenType::IntegerLiteral(int) => write!(f, "{}", int),
            TokenType::FloatLiteral(float) => write!(f, "{}", float),
            TokenType::Ident(ident) => write!(f, "{}", ident),
            TokenType::Function => write!(f, "function"),
            TokenType::Let => write!(f, "let"),
            TokenType::If => write!(f, "if"),
            TokenType::Else => write!(f, "else"),
            TokenType::While => write!(f, "while"),
            TokenType::Return => write!(f, "return"),
            TokenType::True => write!(f, "true"),
            TokenType::False => write!(f, "false"),
            TokenType::For => write!(f, "for"),
            TokenType::Do => write!(f, "do"),
            TokenType::Until => write!(f, "until"),
            TokenType::Or => write!(f, "or"),
            TokenType::And => write!(f, "and"),
            TokenType::Not => write!(f, "not"),
            TokenType::Int => write!(f, "int"),
            TokenType::Float => write!(f, "float"),
            TokenType::Bool => write!(f, "bool"),
            TokenType::String => write!(f, "string"),
            TokenType::Null => write!(f, "null"),
            TokenType::Eof => write!(f, "EOF"),
        }
    }
}
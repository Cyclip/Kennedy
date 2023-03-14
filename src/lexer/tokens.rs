use super::Span;

/// A lexical unit, representing a single token in the source code.
#[derive(Debug, Clone, PartialEq)]
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
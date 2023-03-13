use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub enum CompileError {
    SyntaxError(String, Span),
    SemanticError(String, Span),
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CompileError::SyntaxError(msg, span) => write!(f, "Syntax error at {:?}: {}", span, msg),
            CompileError::SemanticError(msg, span) => write!(f, "Semantic error at {:?}: {}", span, msg),
        }
    }
}

pub type CompileResult<T> = Result<T, CompileError>;
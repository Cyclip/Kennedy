mod lexer;
mod parser;
pub mod ast;
pub mod precedence;
// mod codegen;
mod error;

pub use error::{CompileError, CompileResult};


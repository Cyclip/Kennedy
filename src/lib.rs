#![allow(non_snake_case)] // let Kennedy be non-snake case 😭

extern crate cranelift;
extern crate cranelift_codegen;
extern crate cranelift_jit;
extern crate cranelift_native;
extern crate target_lexicon;
extern crate memmap2;

mod lexer;
mod parser;
pub mod ast;
pub mod precedence;
pub mod compiler;
mod error;

pub use error::{CompileError, CompileResult};


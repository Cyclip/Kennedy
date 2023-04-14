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
    CompileError(String),
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CompileError::SyntaxError(msg, span) => write!(f, "Syntax error at {:?}: {}", span, msg),
            CompileError::SemanticError(msg, span) => write!(f, "Semantic error at {:?}: {}", span, msg),
        }
    }
}

impl CompileError {
    /// To string with source.
    pub fn to_string_with_source(&self, source: &str) -> String {
        let mut line = 0;
        let mut col = 0;
        let mut start = 0;
        let mut end = 0;

        let span = match self {
            CompileError::SyntaxError(_, span) => span,
            CompileError::SemanticError(_, span) => span,
        };

        for (i, c) in source.chars().enumerate() {
            if c == '\n' {
                line += 1;
                col = 0;
                start = 0;
                end = 0;
                continue;
            }

            col += 1;

            if i == span.start {
                start = col;
                end = col;
            }

            if span.start <= i && i < span.end {
                end = col;
            }

            if i == span.end {
                break;
            }
        }

        // closure to add a line
        let add_line = |rv: &mut String, line_number: usize| {
            rv.push_str(&format!("\t{} | {}\n",
                line_number + 1,
                source.lines().nth(line_number).expect("Line not found"),
            ));
        };

        // let mut rv = format!("Syntax error at {}:{}: Unexpected token `return`\n",
        //     line + 1,
        //     start + 1,
        // );

        let mut rv = format!("{} at {}:{}: {}\n",
            match self {
                CompileError::SyntaxError(_, _) => "Syntax error",
                CompileError::SemanticError(_, _) => "Semantic error",
            },
            line + 1,
            start + 1,
            match self {
                CompileError::SyntaxError(msg, _) => msg,
                CompileError::SemanticError(msg, _) => msg,
            },
        );

        // add 3 lines before
        for i in 1..3 {
            // if we're at the start of the file, don't add a line
            if line >= i {
                add_line(&mut rv, line - i);
            }
        }

        rv.push_str(&format!("\t{} | {}\n\t{}{}",
            line + 1,
            source.lines().nth(line).expect("Line not found"),
            " ".repeat(start + 3),
            "^".repeat(end - start + 1)
        ));

        rv
    }
}

pub type CompileResult<T> = Result<T, CompileError>;
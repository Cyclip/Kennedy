use crate::ast::{
    Program, Function, Parameters, Parameter, Block, Statement, Expression, Type,
};

use crate::lexer::tokens::{Token, TokenType};

use crate::error::{CompileError, CompileResult};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    /// Match the current token with the given token type
    fn match_tok(&mut self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.peek().token_type != token_type {
            return false;
        }

        self.current += 1;
        true
    }

    /// Match the current token with the given token type
    /// If the current token is not the given token type, return an error
    /// Otherwise, return the current token
    fn consume(&mut self, token_type: TokenType) -> CompileResult<Token> {
        if self.match_tok(token_type) {
            Ok(self.previous().clone())
        } else {
            Err(
                CompileError::SyntaxError(
                    format!("Expected token type: {:?}", token_type),
                    self.peek().span.clone(),
                )
            )
        }
    }

    /// Return the previous token
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    /// Parse a program
    pub fn parse(&mut self) -> CompileResult<Program> {
        let functions: Vec<Function> = Vec::new();

        while !self.is_at_end() {
            let function = self.parse_function()?;
            functions.push(function);
        }

        Ok(Program { functions })
    }

    /// Parse a function
    /// i.e. `func add(a: int, b: int): int { return a + b; }`
    fn parse_function(&mut self) -> CompileResult<Function> {
        // func
        self.consume(TokenType::Function)?;

        // ident
        let ident = self.parse_ident()?;

        // (
        self.consume(TokenType::LeftParen)?;

        // params
        let params = self.parse_parameters()?;

        // )
        self.consume(TokenType::RightParen)?;

        // :
        self.consume(TokenType::Colon)?;

        // type
        let return_type = self.parse_type()?;

        // {
        self.consume(TokenType::LeftBrace)?;

        // body
        let body = self.parse_block()?;

        // }
        self.consume(TokenType::RightBrace)?;

        Ok(Function {
            ident,
            params,
            return_type,
            body,
        })
    }
}
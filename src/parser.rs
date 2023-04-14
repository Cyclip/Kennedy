use crate::ast::{
    Program, Function, Parameters, Parameter, Block, Statement, Expression, Type,
    BinaryOperator, UnaryOperator,
};

use crate::lexer::tokens::{Token, TokenType};
use crate::error::{CompileError, CompileResult};
use crate::precedence::Precedence;

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
    /// Advances the current token if the match is successful
    fn match_advance(&mut self, token_type: TokenType) -> bool {
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
    /// Does not advance
    fn match_peek(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.peek().token_type != token_type {
            return false;
        }

        true
    }

    /// Match the current token with the given token type
    /// If the current token is not the given token type, return an error
    /// Otherwise, return the current token
    fn consume(&mut self, token_type: TokenType) -> CompileResult<Token> {
        let token = self.peek().clone();

        if self.match_advance(token_type.clone()) {
            Ok(self.previous().clone())
        } else {
            Err(
                CompileError::SyntaxError(
                    format!(
                        "Expected token {}, got {}",
                        &token_type,
                        self.peek()
                    ),
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
        let mut functions: Vec<Function> = Vec::new();

        while !self.is_at_end() {
            let function = self.parse_function()?;
            
            println!("Function: {:#?}", function);
            
            functions.push(function);
            
            // print current token
            println!("Current token: {:?} (pos {})", self.peek(), self.current);
        }

        Ok(Program { functions })
    }

    /// Parse a function
    /// i.e. `func add(a: int, b: int): int { return a + b; }`
    fn parse_function(&mut self) -> CompileResult<Function> {
        println!("Parsing function, current token: {:?} (pos {})", self.peek(), self.current);

        // func
        self.consume(TokenType::Function)?;

        // ident
        let ident = self.parse_ident()?;

        println!("Ident: {:?}", ident);

        // params
        let params = self.parse_parameters()?;

        println!("Params: {:?}", params);

        // :
        self.consume(TokenType::Colon)?;

        // type
        let return_type = self.parse_type()?;

        println!("Return type: {:?}", return_type);

        // body
        let body = self.parse_block()?;

        println!("Body: {:?}", body);

        Ok(Function {
            ident,
            params,
            return_type,
            body,
        })
    }

    /// Parse a list of parameters
    /// i.e. `a: int, b: int`
    fn parse_parameters(&mut self) -> CompileResult<Parameters> {
        println!("Parsing parameters, current token: {:?} (pos {})", self.peek(), self.current);

        // (
        self.consume(TokenType::LeftParen)?;

        let mut parameters: Vec<Parameter> = Vec::new();

        while !self.match_peek(TokenType::RightParen) {
            let ident = self.parse_ident()?;
            self.consume(TokenType::Colon)?;
            let param_type = self.parse_type()?;
            parameters.push(Parameter { ident, param_type });

            if !self.match_peek(TokenType::RightParen) {
                self.consume(TokenType::Comma)?;
            }
        };

        // )
        self.consume(TokenType::RightParen)?;

        Ok(Parameters { params: parameters })
    }

    /// Parse a block
    /// i.e. `{ a += 1; return a; }`
    fn parse_block(&mut self) -> CompileResult<Block> {
        println!("Parsing block, current token: {:?} (pos {})", self.peek(), self.current);

        // {
        self.consume(TokenType::LeftBrace)?;

        let mut statements: Vec<Statement> = Vec::new();

        while !self.match_peek(TokenType::RightBrace) {
            statements.push(self.parse_statement()?);
        }

        // }
        self.consume(TokenType::RightBrace)?;

        Ok(Block { statements })
    }

    /// Parse a block statement
    /// i.e. `{ a += 1; return a; }`
    fn parse_block_statement(&mut self) -> CompileResult<Statement> {
        println!("Parsing block statement, current token: {:?} (pos {})", self.peek(), self.current);

        let block = self.parse_block()?;
        Ok(Statement::Block{
            block: Box::new(block),
        })
    }

    /// Parse a statement
    /// Can be any of the following:
    /// - Variable declaration
    /// - Assignment
    /// - Print
    /// - If statement
    /// - For statement
    /// - While statement
    /// - Do until statement
    /// - Return statement
    /// - Block
    /// - Expression
    fn parse_statement(&mut self) -> CompileResult<Statement> {
        println!("Parsing statement, current token: {:?} (pos {})", self.peek(), self.current);

        if self.match_peek(TokenType::Let) {
            self.parse_variable_declaration()
        // } else if self.match_peek(TokenType::Print) {
        //     self.parse_print_statement()
        } else if self.match_peek(TokenType::If) {
            self.parse_if_statement()
        } else if self.match_peek(TokenType::For) {
            self.parse_for_statement()
        } else if self.match_peek(TokenType::While) {
            self.parse_while_statement()
        } else if self.match_peek(TokenType::Do) {
            self.parse_do_until_statement()
        } else if self.match_peek(TokenType::Return) {
            self.parse_return_statement()
        } else if self.match_peek(TokenType::LeftBrace) {
            self.parse_block_statement()
        } else {
            self.parse_expression_statement()
        }
    }

    /// Parse a variable declaration
    /// i.e. `let a: int = 1;`
    /// i.e. `let a = 1;` (type is inferred)
    /// Value must be assigned
    fn parse_variable_declaration(&mut self) -> CompileResult<Statement> {
        println!("Parsing variable declaration, current token: {:?} (pos {})", self.peek(), self.current);

        // let
        self.consume(TokenType::Let)?;

        // ident
        let ident = self.parse_ident()?;

        // is type specified?
        let var_type = if self.match_peek(TokenType::Colon) {
            // :
            self.consume(TokenType::Colon)?;

            // type
            Some(self.parse_type()?)
        } else {
            None
        };

        // =
        self.consume(TokenType::Equal)?;

        let value = self.parse_expression()?;

        // ;
        self.consume(TokenType::Semicolon)?;

        Ok(Statement::VariableDeclaration { 
            ident,
            var_type,
            value,
        })
    }

    /// Parse an if statement
    /// May contain nested if/else statements
    fn parse_if_statement(&mut self) -> CompileResult<Statement> {
        println!("Parsing if statement, current token: {:?} (pos {})", self.peek(), self.current);

        unimplemented!()

        // if
        // self.consume(TokenType::If)?;

        // // condition
        // let condition = self.parse_expression()?;

        // // body
        // let body = self.parse_block()?;

        // // else?
        // let else_body = if self.match_peek(TokenType::Else) {
        //     // else
        //     self.consume(TokenType::Else)?;

        //     // if?
        //     if self.match_peek(TokenType::If) {
        //         Some(Box::new(self.parse_if_statement()?))
        //     } else {
        //         let block = Box::new(self.parse_block()?);
        //         Some(Box::new(
        //             Statement::Block { block: block }
        //         ))
        //     }
        // } else {
        //     None
        // };
    }

    /// Parse a for statement
    /// for (let i: int = 0; i < 10; i += 1) { ... }
    fn parse_for_statement(&mut self) -> CompileResult<Statement> {
        println!("Parsing for statement, current token: {:?} (pos {})", self.peek(), self.current);

        unimplemented!()

        // // for
        // self.consume(TokenType::For)?;

        // // (
        // self.consume(TokenType::LeftParen)?;

        // // init
        // let init = if self.match_peek(TokenType::Let) {
        //     Some(self.parse_variable_declaration()?)
        // } else if self.match_peek(TokenType::Semicolon) {
        //     None
        // } else {
        //     Some(self.parse_expression_statement()?)
        // };

        // // condition
        // let condition = if self.match_peek(TokenType::Semicolon) {
        //     None
        // } else {
        //     let expr = self.parse_expression()?;
        //     self.consume(TokenType::Semicolon)?;
        //     Some(expr)
        // };

        // // increment
        // let increment = if self.match_peek(TokenType::RightParen) {
        //     None
        // } else {
        //     let expr = self.parse_expression()?;
        //     self.consume(TokenType::RightParen)?;
        //     Some(expr)
        // };

        // // body
        // let body = self.parse_block()?;

        // Ok(Statement::For {
        //     init,
        //     condition,
        //     increment,
        //     body,
        // })
    }

    /// Parse a while statement
    /// while (i < 10) { ... }
    fn parse_while_statement(&mut self) -> CompileResult<Statement> {
        println!("Parsing while statement, current token: {:?} (pos {})", self.peek(), self.current);

        unimplemented!()

        // // while
        // self.consume(TokenType::While)?;

        // // (
        // self.consume(TokenType::LeftParen)?;

        // // condition
        // let condition = self.parse_expression()?;

        // // )
        // self.consume(TokenType::RightParen)?;

        // // body
        // let body = self.parse_block()?;

        // Ok(Statement::While {
        //     condition,
        //     body,
        // })
    }

    /// Parse a do until statement
    /// do { ... } until (i >= 10)
    fn parse_do_until_statement(&mut self) -> CompileResult<Statement> {
        println!("Parsing do until statement, current token: {:?} (pos {})", self.peek(), self.current);

        unimplemented!()
    }

    /// Parse a return statement
    /// return 1;
    fn parse_return_statement(&mut self) -> CompileResult<Statement> {
        // return
        self.consume(TokenType::Return)?;

        // value
        let value = if self.match_peek(TokenType::Semicolon) {
            None
        } else {
            Some(self.parse_expression()?)
        };

        // ;
        self.consume(TokenType::Semicolon)?;

        Ok(Statement::Return { value })
    }
    
    /// Parse an expression statement
    /// i.e. `1 + 1;`
    fn parse_expression_statement(&mut self) -> CompileResult<Statement> {
        println!("Parsing expression statement, current token: {:?} (pos {})", self.peek(), self.current);

        let expr = self.parse_expression()?;

        // ;
        self.consume(TokenType::Semicolon)?;

        Ok(Statement::Expression { expression: expr })
    }

    /// Parse an ident
    /// i.e. `foo`
    fn parse_ident(&mut self) -> CompileResult<String> {
        println!("Parsing ident, current token: {:?} (pos {})", self.peek(), self.current);

        match &self.peek().token_type {
            TokenType::Ident(x) => {
                let token = self.consume(TokenType::Ident(x.clone()))?;
                Ok(match token.token_type {
                    TokenType::Ident(x) => x,
                    _ => unreachable!(),
                })
            }
            _ => Err(CompileError::SyntaxError(
                format!("Expected ident, got {:?}", self.peek().token_type),
                self.peek().span.clone(),
            )),
        }
    }

    /// Parse a type
    /// i.e. `int`
    fn parse_type(&mut self) -> CompileResult<Type> {
        println!("Parsing type, current token: {:?} (pos {})", self.peek(), self.current);

        match self.peek().token_type {
            TokenType::Int => {
                self.consume(TokenType::Int)?;
                Ok(Type::Int)
            }
            TokenType::Float => {
                self.consume(TokenType::Float)?;
                Ok(Type::Float)
            }
            TokenType::String => {
                self.consume(TokenType::String)?;
                Ok(Type::String)
            }
            TokenType::Bool => {
                self.consume(TokenType::Bool)?;
                Ok(Type::Bool)
            }
            TokenType::Null => {
                self.consume(TokenType::Null)?;
                Ok(Type::Null)
            }

            _ => Err(CompileError::SyntaxError(
                format!("Expected type, got {:?}", self.peek().token_type),
                self.peek().span.clone(),
            )),
        }
    }

    /// Parse an expression
    /// May be a binary expression, unary expression, or a literal
    /// i.e. `1 + 1`
    fn parse_expression(&mut self) -> CompileResult<Expression> {
        println!("Parsing expression, current token: {:?} (pos {})", self.peek(), self.current);

        // first we parse the left hand side
        let lhs = self.parse_term()?;

        // if the next token is an operator, we parse the right hand side
        // and return a binary expression
        if self.match_peek(TokenType::Plus)
            || self.match_peek(TokenType::Minus)
            || self.match_peek(TokenType::Star)
            || self.match_peek(TokenType::Slash)
            || self.match_peek(TokenType::EqualEqual)
            || self.match_peek(TokenType::BangEqual)
            || self.match_peek(TokenType::Greater)
            || self.match_peek(TokenType::GreaterEqual)
            || self.match_peek(TokenType::Less)
            || self.match_peek(TokenType::LessEqual)
        {
            let op = self.consume(self.peek().token_type.clone())?;
            let op_btoken = match op.token_type {
                TokenType::Plus => BinaryOperator::Plus,
                TokenType::Minus => BinaryOperator::Minus,
                TokenType::Star => BinaryOperator::Star,
                TokenType::Slash => BinaryOperator::Slash,
                TokenType::EqualEqual => BinaryOperator::EqualEqual,
                TokenType::BangEqual => BinaryOperator::BangEqual,
                TokenType::Greater => BinaryOperator::Greater,
                TokenType::GreaterEqual => BinaryOperator::GreaterEqual,
                TokenType::Less => BinaryOperator::Less,
                TokenType::LessEqual => BinaryOperator::LessEqual,
                _ => unreachable!(),
            };

            let rhs = self.parse_expression()?;

            Ok(Expression::Binary {
                left: Box::new(lhs),
                operator: op_btoken,
                right: Box::new(rhs),
            })
        } else {
            Ok(lhs)
        }
    }

    /// Parse a term
    /// May have + or - in front of it
    /// i.e. `1 + 1`
    fn parse_term(&mut self) -> CompileResult<Expression> {
        println!("Parsing term, current token: {:?} (pos {})", self.peek(), self.current);

        // first we parse the left hand side
        let lhs = self.parse_factor()?;

        // if the next token is an operator, we parse the right hand side
        // and return a binary expression
        if self.match_peek(TokenType::Plus) || self.match_peek(TokenType::Minus) {
            let op = self.consume(self.peek().token_type.clone())?;
            let op_btoken = match op.token_type {
                TokenType::Plus => BinaryOperator::Plus,
                TokenType::Minus => BinaryOperator::Minus,
                _ => unreachable!(),
            };

            let rhs = self.parse_term()?;

            Ok(Expression::Binary {
                left: Box::new(lhs),
                operator: op_btoken,
                right: Box::new(rhs),
            })
        } else {
            Ok(lhs)
        }
    }

    /// Parse a factor
    /// May have * or / in front of it
    /// i.e. `1 * 1`
    fn parse_factor(&mut self) -> CompileResult<Expression> {
        println!("Parsing factor, current token: {:?} (pos {})", self.peek(), self.current);

        // first we parse the left hand side
        let lhs = self.parse_unary()?;

        // if the next token is an operator, we parse the right hand side
        // and return a binary expression
        if self.match_peek(TokenType::Star) || self.match_peek(TokenType::Slash) {
            let op = self.consume(self.peek().token_type.clone())?;
            let op_btoken = match op.token_type {
                TokenType::Star => BinaryOperator::Star,
                TokenType::Slash => BinaryOperator::Slash,
                _ => unreachable!(),
            };

            let rhs = self.parse_factor()?;

            Ok(Expression::Binary {
                left: Box::new(lhs),
                operator: op_btoken,
                right: Box::new(rhs),
            })
        } else {
            Ok(lhs)
        }
    }

    /// Parse a unary expression
    /// May have +, -, or ! in front of it
    /// i.e. `-1`
    fn parse_unary(&mut self) -> CompileResult<Expression> {
        println!("Parsing unary, current token: {:?} (pos {})", self.peek(), self.current);

        if self.match_peek(TokenType::Bang) || self.match_peek(TokenType::Minus)
         {
            let op = self.consume(self.peek().token_type.clone())?;
            let op_utoken = match op.token_type {
                TokenType::Minus => UnaryOperator::Minus,
                TokenType::Bang => UnaryOperator::Bang,
                _ => unreachable!(),
            };

            let rhs = self.parse_unary()?;

            Ok(Expression::Unary {
                operator: op_utoken,
                right: Box::new(rhs),
            })
        } else {
            self.parse_primary()
        }
    }

    /// Parse a primary expression
    /// Can be a literal, a parenthesized expression, or a variable
    /// i.e. `1`, `(1 + 1)`, `foo`
    fn parse_primary(&mut self) -> CompileResult<Expression> {
        println!("Parsing primary, current token: {:?} (pos {})", self.peek(), self.current);

        match &self.peek().token_type {
            TokenType::IntegerLiteral(x) => {
                let token = self.consume(TokenType::IntegerLiteral(*x))?;
                Ok(Expression::IntegerLiteral {
                    value: match token.token_type {
                        TokenType::IntegerLiteral(x) => x,
                        _ => unreachable!(),
                    },
                })
            }

            TokenType::FloatLiteral(x) => {
                let token = self.consume(TokenType::FloatLiteral(*x))?;
                Ok(Expression::FloatLiteral {
                    value: match token.token_type {
                        TokenType::FloatLiteral(x) => x,
                        _ => unreachable!(),
                    },
                })
            }

            TokenType::StringLiteral(x) => {
                let token = self.consume(TokenType::StringLiteral(x.clone()))?;
                Ok(Expression::StringLiteral {
                    value: match token.token_type {
                        TokenType::StringLiteral(x) => x,
                        _ => unreachable!(),
                    },
                })
            }

            TokenType::Ident(x) => {
                let token = self.consume(TokenType::Ident(x.clone()))?;
                Ok(Expression::Identifier {
                    ident: match token.token_type {
                        TokenType::Ident(x) => x,
                        _ => unreachable!(),
                    },
                })
            }

            TokenType::LeftParen => {
                self.consume(TokenType::LeftParen)?;
                let expr = self.parse_expression()?;
                self.consume(TokenType::RightParen)?;
                Ok(expr)
            }

            _ => Err(CompileError::SyntaxError(
                format!("Expected primary expression, got {:?}", self.peek().token_type),
                self.peek().span.clone(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer;

    #[test]
    fn test_parse() {
        let source = r#"func main(): int {
    return 1 + 2 * 3;
}
        "#;

        // tokenize
        let lexer = lexer::lex(source.to_string());

        let lexer = match lexer {
            Ok(lexer) => Some(lexer),
            Err(e) => {
                println!("Error: {}", e);
                None
            }
        };

        // parse
        let mut parser = crate::parser::Parser::new(lexer.unwrap());
        let ast = parser.parse();

        match ast {
            Ok(ast) => println!("AST: {:#?}", ast),
            Err(e) => println!("Error: {}", e.to_string_with_source(source)),
        }
    }
}
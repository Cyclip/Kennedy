#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub functions: Vec<Function>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    // ident
    pub ident: String,
    // params
    pub params: Parameters,
    // type
    pub return_type: Type,
    // body
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameters {
    pub params: Vec<Parameter>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub ident: String,
    pub param_type: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    // variable decl
    VariableDeclaration {
        ident: String,
        var_type: Option<Type>,
        value: Expression,
    },
    // variable assign
    Assign {
        ident: String,
        value: Expression,
    },
    // return 1;
    Return {
        value: Option<Expression>,
    },
    /// { ... }
    Block {
        block: Box<Block>,
    },
    // if (x) { 1; } else { 2; }
    If {
        condition: Expression,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },
    // while (x) { 1; }
    While {
        condition: Expression,
        body: Block,
    },
    // 1;
    Expression {
        expression: Expression,
    },
    DoUntil {
        condition: Expression,
        body: Box<Statement>,
    },
    For {
        init: Box<Statement>,
        condition: Expression,
        increment: Box<Statement>,
        body: Block,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    // 1
    IntegerLiteral {
        value: i64,
    },
    // 1.0
    FloatLiteral {
        value: f64,
    },
    // "hello"
    StringLiteral {
        value: String,
    },
    // true
    BooleanLiteral {
        value: bool,
    },
    // null
    NullLiteral,
    // x
    Identifier {
        ident: String,
    },
    // x + 1
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    // -1
    Unary {
        operator: UnaryOperator,
        right: Box<Expression>,
    },
    // (1 + 2)
    Grouping {
        expression: Box<Expression>,
    },
    // function (x) { return x; }
    Function {
        params: Parameters,
        return_type: Type,
        body: Block,
    },
    // call()
    Call {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },
    // x++
    Postfix {
        left: Box<Expression>,
        operator: PostfixOperator,
    },
    // x--
    Prefix {
        operator: PrefixOperator,
        right: Box<Expression>,
    },
    // x += 1
    Assign {
        left: Box<Expression>,
        operator: AssignOperator,
        right: Box<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Star,
    Slash,
    StarStar,
    SlashSlash,
    EqualEqual,
    BangEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Or,
    And,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Minus,
    Bang,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PostfixOperator {
    PlusPlus,
    MinusMinus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrefixOperator {
    PlusPlus,
    MinusMinus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssignOperator {
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
}

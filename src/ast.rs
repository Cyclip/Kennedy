pub struct Program {
    pub functions: Vec<Function>,
}

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

pub struct Parameters {
    pub params: Vec<Parameter>,
}

pub struct Parameter {
    pub ident: String,
    pub param_type: Type,
}

pub struct Block {
    pub statements: Vec<Statement>,
}

pub enum Statement {
    // variable decl
    Let {
        ident: String,
        value: Expression,
    },
    // variable assign
    Assign {
        ident: String,
        value: Expression,
    },
    // return 1;
    Return {
        value: Expression,
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
        body: Box<Statement>,
    },
    // { 1; 2; }
    Block {
        block: Block,
    },
    // 1;
    Expression {
        expression: Expression,
    },
    DoUntil {
        condition: Expression,
        body: Box<Statement>,
    },
}

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

pub enum Type {
    Int,
    Float,
    Bool,
    String,
    Null,
    Void,
}

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
}

pub enum UnaryOperator {
    Minus,
    Bang,
}

pub enum PostfixOperator {
    PlusPlus,
    MinusMinus,
}

pub enum PrefixOperator {
    PlusPlus,
    MinusMinus,
}

pub enum AssignOperator {
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
}

//! Module for precedence of operators

use crate::ast::{
    BinaryOperator,
    UnaryOperator,
    PostfixOperator,
    PrefixOperator,
    AssignOperator,
};

/// Precedence of operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Precedence {
    /// Lowest precedence.
    Lowest,
    /// Assignment operators.
    Assign,
    /// Logical OR.
    Or,
    /// Logical AND.
    And,
    /// Equality operators.
    Equal,
    /// Comparison operators.
    Compare,
    /// Addition and subtraction.
    Sum,
    /// Multiplication, division, and modulo.
    Product,
    /// Exponentiation.
    Exponent,
    /// Unary operators.
    Unary,
    /// Postfix operators.
    Postfix,
    /// Highest precedence.
    Highest,
}

/// Precedence of a binary operator
pub fn precedence_of_binary_operator(operator: BinaryOperator) -> Precedence {
    match operator {
        BinaryOperator::Or => Precedence::Or,
        BinaryOperator::And => Precedence::And,
        BinaryOperator::EqualEqual => Precedence::Equal,
        BinaryOperator::BangEqual => Precedence::Equal,
        BinaryOperator::Greater => Precedence::Compare,
        BinaryOperator::GreaterEqual => Precedence::Compare,
        BinaryOperator::Less => Precedence::Compare,
        BinaryOperator::LessEqual => Precedence::Compare,
        BinaryOperator::Plus => Precedence::Sum,
        BinaryOperator::Minus => Precedence::Sum,
        BinaryOperator::Star => Precedence::Product,
        BinaryOperator::Slash => Precedence::Product,
        BinaryOperator::StarStar => Precedence::Exponent,
        BinaryOperator::SlashSlash => Precedence::Exponent,
    }
}

/// Precedence of a unary operator
pub fn precedence_of_unary_operator(operator: UnaryOperator) -> Precedence {
    match operator {
        UnaryOperator::Minus => Precedence::Unary,
        UnaryOperator::Bang => Precedence::Unary,
    }
}

/// Precedence of a postfix operator
pub fn precedence_of_postfix_operator(operator: PostfixOperator) -> Precedence {
    match operator {
        PostfixOperator::PlusPlus => Precedence::Postfix,
        PostfixOperator::MinusMinus => Precedence::Postfix,
    }
}

/// Precedence of a prefix operator
pub fn precedence_of_prefix_operator(operator: PrefixOperator) -> Precedence {
    match operator {
        PrefixOperator::PlusPlus => Precedence::Postfix,
        PrefixOperator::MinusMinus => Precedence::Postfix,
    }
}

/// Precedence of an assignment operator
pub fn precedence_of_assign_operator(operator: AssignOperator) -> Precedence {
    match operator {
        AssignOperator::PlusEqual => Precedence::Assign,
        AssignOperator::MinusEqual => Precedence::Assign,
        AssignOperator::StarEqual => Precedence::Assign,
        AssignOperator::SlashEqual => Precedence::Assign,
    }
}

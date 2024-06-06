use core::fmt;
use std::fmt::{Display, Formatter};

use crate::interpreter::functions::Reference;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Literal(ExpressionLiteral),
    Operation(Operation),
    Grouping(Box<Expression>),
    Variable(ExpressionVariable),
    Assign(ExpressionVariable, Box<Expression>),
    Call(usize, Box<Expression>, Vec<Expression>),
    Get(usize, Box<Expression>, String),
    Set(usize, Box<Expression>, String, Box<Expression>),
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct ExpressionVariable {
    pub line_number: usize,
    pub identifier_name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExpressionLiteral {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
    Reference(Reference),
}

impl ExpressionLiteral {
    pub fn is_same_type(&self, other: &ExpressionLiteral) -> bool {
        match self {
            ExpressionLiteral::Number(_) => match other {
                ExpressionLiteral::Number(_) => true,
                _ => false,
            },
            ExpressionLiteral::String(_) => match other {
                ExpressionLiteral::String(_) => true,
                _ => false,
            },
            ExpressionLiteral::True | ExpressionLiteral::False => {
                *other == ExpressionLiteral::True || *other == ExpressionLiteral::False
            }
            ExpressionLiteral::Nil => match other {
                ExpressionLiteral::Nil => true,
                _ => false,
            },
            ExpressionLiteral::Reference(_) => false,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EqualityOperation {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub line_number: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ComparisonOperation {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub line_number: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FactorOperation {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub line_number: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TermOperation {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub line_number: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnaryOperation {
    pub operand: Box<Expression>,
    pub line_number: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LogicalOperation {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub line_number: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operation {
    Negate(UnaryOperation),
    Not(UnaryOperation),

    Equal(EqualityOperation),
    NotEqual(EqualityOperation),
    Less(ComparisonOperation),
    LessEqual(ComparisonOperation),
    Greater(ComparisonOperation),
    GreaterEqual(ComparisonOperation),

    Plus(TermOperation),
    Minus(TermOperation),
    Multiply(FactorOperation),
    Divide(FactorOperation),

    And(LogicalOperation),
    Or(LogicalOperation),
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Expression::Literal(literal) => write!(f, "{}", *literal),
            Expression::Operation(operation) => write!(f, "{}", *operation),
            Expression::Grouping(expression) => write!(f, "( {} )", *expression),
            Expression::Variable(name) => write!(f, "( *{} )", name.identifier_name),
            Expression::Assign(name, value) => {
                write!(f, "( {} <-- {} )", name.identifier_name, value)
            }
            Expression::Call(_line_number, callee, arguments) => write!(
                f,
                "( {callee} <-call-with- ( {} ))",
                arguments
                    .iter()
                    .map(|argument_expression| argument_expression.to_string())
                    .fold(String::new(), |prev, cur_argument| prev
                        + ", "
                        + &cur_argument)
            ),
            Expression::Get(_line_number, expression, identifier) => {
                write!(f, "( {}.{} )", *expression, identifier)
            }
            Expression::Set(_line_number, expression, identifier, value) => {
                write!(f, "( {}.{} <-- {} )", *expression, identifier, *value)
            }
        }
    }
}

impl Display for ExpressionLiteral {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ExpressionLiteral::Number(number) => write!(f, "{:.2}", number),
            ExpressionLiteral::String(string_literal) => write!(f, "\"{}\"", string_literal),
            ExpressionLiteral::True => write!(f, "true"),
            ExpressionLiteral::False => write!(f, "false"),
            ExpressionLiteral::Nil => write!(f, "nil"),
            ExpressionLiteral::Reference(reference) => match reference {
                Reference::CallableReference(callable_reference) => {
                    write!(f, "@Callable<Arity = {}>", callable_reference.arity)
                }
                Reference::ClassReference(class_reference) => {
                    write!(f, "@Class<Name = \"{}\">", class_reference.name)
                }
                Reference::InstanceReference(instance_reference) => {
                    write!(f, "@Instance<Name = \"{}\">", instance_reference.class.name)
                }
            },
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Operation::Negate(unary_operation) => {
                write!(f, "( - {} )", (*unary_operation.operand))
            }
            Operation::Not(unary_operation) => {
                write!(f, "( not {} )", (*unary_operation.operand))
            }
            Operation::Equal(binary_operation) => {
                write!(
                    f,
                    "( == {} {} )",
                    *binary_operation.left, *binary_operation.right
                )
            }
            Operation::NotEqual(binary_operation) => {
                write!(
                    f,
                    "( != {} {} )",
                    *binary_operation.left, *binary_operation.right
                )
            }
            Operation::Less(binary_operation) => {
                write!(
                    f,
                    "( < {} {} )",
                    *binary_operation.left, *binary_operation.right
                )
            }
            Operation::LessEqual(binary_operation) => {
                write!(
                    f,
                    "( <= {} {} )",
                    *binary_operation.left, *binary_operation.right
                )
            }
            Operation::Greater(binary_operation) => {
                write!(
                    f,
                    "( > {} {} )",
                    *binary_operation.left, *binary_operation.right
                )
            }
            Operation::GreaterEqual(binary_operation) => {
                write!(
                    f,
                    "( >= {} {} )",
                    *binary_operation.left, *binary_operation.right
                )
            }
            Operation::Plus(binary_operation) => {
                write!(
                    f,
                    "( + {} {} )",
                    *binary_operation.left, *binary_operation.right
                )
            }
            Operation::Minus(binary_operation) => {
                write!(
                    f,
                    "( - {} {} )",
                    *binary_operation.left, *binary_operation.right
                )
            }
            Operation::Multiply(binary_operation) => {
                write!(
                    f,
                    "( * {} {} )",
                    *binary_operation.left, *binary_operation.right
                )
            }
            Operation::Divide(binary_operation) => {
                write!(
                    f,
                    "( / {} {} )",
                    *binary_operation.left, *binary_operation.right
                )
            }
            Operation::And(logical_operation) => write!(
                f,
                "( and {} {} )",
                *logical_operation.left, *logical_operation.right
            ),
            Operation::Or(logical_operation) => write!(
                f,
                "( or {} {} )",
                *logical_operation.left, *logical_operation.right
            ),
        }
    }
}

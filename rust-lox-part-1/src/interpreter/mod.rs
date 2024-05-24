mod tests;

use crate::{
    errors::RuntimeError,
    tree::expression::{
        ComparisonOperation, EqualityOperation, Expression, ExpressionLiteral, FactorOperation,
        Operation, TermOperation, UnaryOperation,
    },
};

pub fn interpret_tree(tree: Expression) -> Result<ExpressionLiteral, RuntimeError> {
    let literal: Result<ExpressionLiteral, RuntimeError> = match tree {
        Expression::Grouping(grouped_expression) => interpret_tree(*grouped_expression),
        Expression::Literal(literal) => Ok(literal),
        Expression::Operation(operation) => match operation {
            Operation::Negate(UnaryOperation {
                operand,
                line_number,
            }) => match *operand {
                Expression::Literal(literal) => match literal {
                    ExpressionLiteral::Number(number) => Ok(ExpressionLiteral::Number(-number)),
                    ExpressionLiteral::Nil => Err(RuntimeError {
                        line_number,
                        message: "Tried to Negate Nil value".to_string(),
                    }),
                    literal => Err(RuntimeError {
                        line_number,
                        message: format!("Tried to Negate invalid literal: {literal}"),
                    }),
                },
                expression => interpret_tree(expression),
            },
            Operation::Not(UnaryOperation {
                operand,
                line_number: _,
            }) => {
                if is_truthy(*operand)? {
                    Ok(ExpressionLiteral::False)
                } else {
                    Ok(ExpressionLiteral::True)
                }
            }
            Operation::Equal(EqualityOperation {
                left,
                right,
                line_number,
            }) => {
                let left_parsed = interpret_tree(*left)?;
                let right_parsed = interpret_tree(*right)?;

                if !left_parsed.is_same_type(&right_parsed) {
                    return Err(RuntimeError {
                        message: format!(
                            "Tried to compare invalid types to each other: {} and {}",
                            left_parsed, right_parsed
                        ),
                        line_number,
                    });
                }

                if left_parsed == right_parsed {
                    Ok(ExpressionLiteral::True)
                } else {
                    Ok(ExpressionLiteral::False)
                }
            }
            Operation::NotEqual(EqualityOperation {
                left,
                right,
                line_number,
            }) => {
                let left_parsed = interpret_tree(*left)?;
                let right_parsed = interpret_tree(*right)?;

                if !left_parsed.is_same_type(&right_parsed) {
                    return Err(RuntimeError {
                        message: format!(
                            "Tried to compare invalid types to each other: {} and {}",
                            left_parsed, right_parsed
                        ),
                        line_number,
                    });
                }

                if left_parsed != right_parsed {
                    Ok(ExpressionLiteral::True)
                } else {
                    Ok(ExpressionLiteral::False)
                }
            }
            Operation::Less(ComparisonOperation {
                left,
                right,
                line_number,
            }) => {
                let left_parsed = interpret_tree(*left)?;
                let right_parsed = interpret_tree(*right)?;

                match (&left_parsed, &right_parsed) {
                    (
                        ExpressionLiteral::Number(left_number),
                        ExpressionLiteral::Number(right_number),
                    ) => Ok(if left_number < right_number {
                        ExpressionLiteral::True
                    } else {
                        ExpressionLiteral::False
                    }),
                    _ => Err(RuntimeError {
                        message: format!("Cannot compare types {left_parsed} and {right_parsed}"),
                        line_number,
                    }),
                }
            }
            Operation::LessEqual(ComparisonOperation {
                left,
                right,
                line_number,
            }) => {
                let left_parsed = interpret_tree(*left)?;
                let right_parsed = interpret_tree(*right)?;

                match (&left_parsed, &right_parsed) {
                    (
                        ExpressionLiteral::Number(left_number),
                        ExpressionLiteral::Number(right_number),
                    ) => Ok(if left_number <= right_number {
                        ExpressionLiteral::True
                    } else {
                        ExpressionLiteral::False
                    }),
                    _ => Err(RuntimeError {
                        message: format!("Cannot compare types {left_parsed} and {right_parsed}"),
                        line_number,
                    }),
                }
            }
            Operation::Greater(ComparisonOperation {
                left,
                right,
                line_number,
            }) => {
                let left_parsed = interpret_tree(*left)?;
                let right_parsed = interpret_tree(*right)?;

                match (&left_parsed, &right_parsed) {
                    (
                        ExpressionLiteral::Number(left_number),
                        ExpressionLiteral::Number(right_number),
                    ) => Ok(if left_number > right_number {
                        ExpressionLiteral::True
                    } else {
                        ExpressionLiteral::False
                    }),
                    _ => Err(RuntimeError {
                        message: format!("Cannot compare types {left_parsed} and {right_parsed}"),
                        line_number,
                    }),
                }
            }
            Operation::GreaterEqual(ComparisonOperation {
                left,
                right,
                line_number,
            }) => {
                let left_parsed = interpret_tree(*left)?;
                let right_parsed = interpret_tree(*right)?;

                match (&left_parsed, &right_parsed) {
                    (
                        ExpressionLiteral::Number(left_number),
                        ExpressionLiteral::Number(right_number),
                    ) => Ok(if left_number >= right_number {
                        ExpressionLiteral::True
                    } else {
                        ExpressionLiteral::False
                    }),
                    _ => Err(RuntimeError {
                        message: format!("Cannot compare types {left_parsed} and {right_parsed}"),
                        line_number,
                    }),
                }
            }
            Operation::Plus(TermOperation {
                left,
                right,
                line_number,
            }) => {
                let left_parsed = interpret_tree(*left)?;
                let right_parsed = interpret_tree(*right)?;

                match (&left_parsed, &right_parsed) {
                    (
                        ExpressionLiteral::Number(left_number),
                        ExpressionLiteral::Number(right_number),
                    ) => Ok(ExpressionLiteral::Number(left_number + right_number)),
                    (
                        ExpressionLiteral::String(left_string),
                        ExpressionLiteral::String(right_string),
                    ) => Ok(ExpressionLiteral::String(
                        left_string.to_owned() + right_string,
                    )),
                    _ => Err(RuntimeError {
                        message: format!("Cannot add types {left_parsed} and {right_parsed}"),
                        line_number,
                    }),
                }
            }
            Operation::Minus(TermOperation {
                left,
                right,
                line_number,
            }) => {
                let left_parsed = interpret_tree(*left)?;
                let right_parsed = interpret_tree(*right)?;

                match (&left_parsed, &right_parsed) {
                    (
                        ExpressionLiteral::Number(left_number),
                        ExpressionLiteral::Number(right_number),
                    ) => Ok(ExpressionLiteral::Number(left_number - right_number)),
                    _ => Err(RuntimeError {
                        message: format!("Cannot subtract types {left_parsed} and {right_parsed}"),
                        line_number,
                    }),
                }
            }
            Operation::Multiply(FactorOperation {
                left,
                right,
                line_number,
            }) => {
                let left_parsed = interpret_tree(*left)?;
                let right_parsed = interpret_tree(*right)?;

                match (&left_parsed, &right_parsed) {
                    (
                        ExpressionLiteral::Number(left_number),
                        ExpressionLiteral::Number(right_number),
                    ) => Ok(ExpressionLiteral::Number(left_number * right_number)),
                    _ => Err(RuntimeError {
                        message: format!("Cannot multiply types {left_parsed} and {right_parsed}"),
                        line_number,
                    }),
                }
            }
            Operation::Divide(FactorOperation {
                left,
                right,
                line_number,
            }) => {
                let left_parsed = interpret_tree(*left)?;
                let right_parsed = interpret_tree(*right)?;

                // TODO: Handle divide by zero behavior

                match (&left_parsed, &right_parsed) {
                    (
                        ExpressionLiteral::Number(left_number),
                        ExpressionLiteral::Number(right_number),
                    ) => Ok(ExpressionLiteral::Number(left_number / right_number)),
                    _ => Err(RuntimeError {
                        message: format!("Cannot divide types {left_parsed} and {right_parsed}"),
                        line_number,
                    }),
                }
            }
        },
    };

    return Ok(literal?);
}

pub fn is_truthy(expr: Expression) -> Result<bool, RuntimeError> {
    match expr {
        Expression::Literal(literal) => match literal {
            ExpressionLiteral::Number(number) => Ok(number != 0.0),
            ExpressionLiteral::String(str) => Ok(str.len() > 0),
            ExpressionLiteral::True => Ok(true),
            ExpressionLiteral::False => Ok(false),
            ExpressionLiteral::Nil => Ok(false),
        },
        tree => is_truthy(Expression::Literal(interpret_tree(tree)?)),
    }
}

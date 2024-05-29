mod environment;
mod statements;
mod tests;

use environment::Environment;
use statements::interpret_variable_definition;

use crate::{
    errors::RuntimeError,
    parser::{statements::Statement, ParsedStep, ParsingResult},
    tree::expression::{
        ComparisonOperation, EqualityOperation, Expression, ExpressionLiteral, ExpressionVariable,
        FactorOperation, Operation, TermOperation, UnaryOperation,
    },
};

use self::statements::interpret_print;

// TODO: Test
pub fn interpret(steps: Vec<ParsingResult>) -> Result<(), RuntimeError> {
    let global_environment = &mut Environment::new();

    interpret_steps(global_environment, steps)
}

pub fn interpret_steps(
    environment: &Environment,
    steps: Vec<ParsingResult>,
) -> Result<(), RuntimeError> {
    for step in steps {
        match step.unwrap() {
            ParsedStep::Expression(expr) => {
                interpret_expression_tree(environment, expr)?;
            }
            ParsedStep::Statement(statement) => {
                // TODO: Find some way to get the line number of a statement
                interpret_statement(environment, statement, 0)?;
            }
            ParsedStep::Block(steps) => {
                let block_environment = Environment::with_parent(environment);

                interpret_steps(&block_environment, steps)?;
            }
        }
    }

    Ok(())
}

pub fn interpret_statement(
    environment: &Environment,
    statement: Statement,
    line_number: usize,
) -> Result<(), RuntimeError> {
    match statement {
        Statement::Print(enclosed_expression) => interpret_print(environment, enclosed_expression)?,
        Statement::Variable(name, value) => {
            interpret_variable_definition(environment, line_number, name, value)?
        }
    }

    Ok(())
}

pub fn interpret_expression_tree(
    environment: &Environment,
    tree: Expression,
) -> Result<ExpressionLiteral, RuntimeError> {
    let literal: Result<ExpressionLiteral, RuntimeError> = match tree {
        Expression::Grouping(grouped_expression) => {
            interpret_expression_tree(environment, *grouped_expression)
        }
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
                expression => interpret_expression_tree(environment, expression),
            },
            Operation::Not(UnaryOperation {
                operand,
                line_number: _,
            }) => {
                if is_truthy(environment, *operand)? {
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
                let left_parsed = interpret_expression_tree(environment, *left)?;
                let right_parsed = interpret_expression_tree(environment, *right)?;

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
                let left_parsed = interpret_expression_tree(environment, *left)?;
                let right_parsed = interpret_expression_tree(environment, *right)?;

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
                let left_parsed = interpret_expression_tree(environment, *left)?;
                let right_parsed = interpret_expression_tree(environment, *right)?;

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
                let left_parsed = interpret_expression_tree(environment, *left)?;
                let right_parsed = interpret_expression_tree(environment, *right)?;

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
                let left_parsed = interpret_expression_tree(environment, *left)?;
                let right_parsed = interpret_expression_tree(environment, *right)?;

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
                let left_parsed = interpret_expression_tree(environment, *left)?;
                let right_parsed = interpret_expression_tree(environment, *right)?;

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
                let left_parsed = interpret_expression_tree(environment, *left)?;
                let right_parsed = interpret_expression_tree(environment, *right)?;

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
                        message: format!("Cannot add values {left_parsed} and {right_parsed}"),
                        line_number,
                    }),
                }
            }
            Operation::Minus(TermOperation {
                left,
                right,
                line_number,
            }) => {
                let left_parsed = interpret_expression_tree(environment, *left)?;
                let right_parsed = interpret_expression_tree(environment, *right)?;

                match (&left_parsed, &right_parsed) {
                    (
                        ExpressionLiteral::Number(left_number),
                        ExpressionLiteral::Number(right_number),
                    ) => Ok(ExpressionLiteral::Number(left_number - right_number)),
                    _ => Err(RuntimeError {
                        message: format!("Cannot subtract values {left_parsed} and {right_parsed}"),
                        line_number,
                    }),
                }
            }
            Operation::Multiply(FactorOperation {
                left,
                right,
                line_number,
            }) => {
                let left_parsed = interpret_expression_tree(environment, *left)?;
                let right_parsed = interpret_expression_tree(environment, *right)?;

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
                let left_parsed = interpret_expression_tree(environment, *left)?;
                let right_parsed = interpret_expression_tree(environment, *right)?;

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
        Expression::Variable(ExpressionVariable {
            line_number,
            identifier_name,
        }) => environment.get_variable(line_number, identifier_name),

        Expression::Assign(expression_variable, right_side_tree) => {
            let expression_value = interpret_expression_tree(environment, *right_side_tree)?;

            environment.set_variable(
                expression_variable.line_number,
                expression_variable.identifier_name,
                expression_value,
            )
        }
    };

    return Ok(literal?);
}

pub fn is_truthy(environment: &Environment, expr: Expression) -> Result<bool, RuntimeError> {
    match expr {
        Expression::Literal(literal) => match literal {
            ExpressionLiteral::Number(number) => Ok(number != 0.0),
            ExpressionLiteral::String(str) => Ok(str.len() > 0),
            ExpressionLiteral::True => Ok(true),
            ExpressionLiteral::False => Ok(false),
            ExpressionLiteral::Nil => Ok(false),
        },
        tree => {
            let evaluated_tree = interpret_expression_tree(environment, tree)?;

            is_truthy(environment, Expression::Literal(evaluated_tree))
        }
    }
}

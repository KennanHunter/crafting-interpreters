mod environment;
pub mod functions;
mod statements;
mod tests;
mod types;

use std::{borrow::Borrow, cell::RefCell, iter::zip, ops::Deref, rc::Rc};

use environment::{Environment, EnvironmentRef};
use functions::{
    native::create_native_now, CallableReference, ClassReference, InstanceReference, Reference,
};
use statements::interpret_variable_definition;
use types::BlockReturn;

use crate::{
    errors::RuntimeError,
    parser::{
        statements::{IfStatement, Statement, WhileStatement},
        ParsedStep, ParsingResult,
    },
    resolver::VariableMap,
    tree::expression::{
        ComparisonOperation, EqualityOperation, Expression, ExpressionLiteral, FactorOperation,
        LogicalOperation, Operation, TermOperation, UnaryOperation,
    },
};

use self::statements::interpret_print;

// TODO: Test
pub fn interpret(variable_map: VariableMap, steps: Vec<ParsingResult>) -> Result<(), RuntimeError> {
    let global_environment = Environment::with_resolved_variable_map(variable_map);

    global_environment.define_variable(
        0,
        "now".to_owned(),
        ExpressionLiteral::Reference(Reference::CallableReference(create_native_now())),
    )?;

    interpret_steps(Rc::new(RefCell::new(global_environment)), steps)?;

    Ok(())
}

pub fn interpret_steps(
    environment: EnvironmentRef,
    steps: Vec<ParsingResult>,
) -> Result<BlockReturn, RuntimeError> {
    for step in steps {
        match interpret_step(environment.clone(), step.unwrap())? {
            BlockReturn::Returned(Some(returned)) => {
                return Ok(BlockReturn::Returned(Some(returned)))
            }
            BlockReturn::Returned(None) => return Ok(BlockReturn::Returned(None)),
            BlockReturn::NoReturn => continue,
        }
    }

    Ok(BlockReturn::NoReturn)
}

fn interpret_step(
    environment: EnvironmentRef,
    step: ParsedStep,
) -> Result<BlockReturn, RuntimeError> {
    Ok(match step {
        ParsedStep::Expression(expr) => {
            interpret_expression_tree(environment.clone(), expr)?;

            BlockReturn::NoReturn
        }
        ParsedStep::Statement(statement) => {
            // TODO: Find some way to get the line number of a statement
            interpret_statement(environment, statement, 0)?
        }
        ParsedStep::Block(steps) => {
            let block_environment = Rc::new(RefCell::new(Environment::with_parent(environment)));

            interpret_steps(block_environment, steps)?
        }
    })
}

pub fn interpret_statement(
    environment: EnvironmentRef,
    statement: Statement,
    line_number: usize,
) -> Result<BlockReturn, RuntimeError> {
    match statement {
        Statement::Print(enclosed_expression) => interpret_print(environment, enclosed_expression)?,
        Statement::Variable(name, value) => {
            interpret_variable_definition(environment.clone(), line_number, name, value)?
        }
        Statement::If(IfStatement {
            condition,
            then_statement,
            else_statement,
        }) => {
            if is_truthy(environment.clone(), condition)? {
                interpret_step(environment.clone(), *then_statement)?;
            } else if else_statement.is_some() {
                interpret_step(environment, *else_statement.unwrap())?;
            }
        }
        Statement::While(WhileStatement { condition, body }) => {
            while is_truthy(environment.clone(), condition.clone())? {
                interpret_step(environment.clone(), *body.clone())?;
            }
        }
        Statement::Fun(function_definition) => {
            let function_parent_environment = environment.clone();
            let function_body = function_definition.clone().body;

            let name = function_definition.name.clone();

            let func =
                ExpressionLiteral::Reference(Reference::CallableReference(CallableReference {
                    arity: function_definition.parameters.len(),
                    subroutine: Rc::new(
                        move |call_line_number, args| -> Result<BlockReturn, RuntimeError> {
                            let env = Environment::with_parent(function_parent_environment.clone());
                            let function_environment = Rc::new(RefCell::new(env.clone()));

                            for (name, value) in zip(function_definition.clone().parameters, args) {
                                (function_environment).borrow_mut().define_variable(
                                    call_line_number,
                                    name,
                                    value,
                                )?;
                            }

                            return interpret_step(function_environment, *function_body.clone());
                        },
                    ),
                }));

            let env: &RefCell<Environment> = environment.borrow();
            env.borrow().define_variable(line_number, name, func)?;
        }
        Statement::Return(optional_expression) => match optional_expression {
            Some(expression) => {
                let returned = interpret_expression_tree(environment, expression)?;

                return Ok(BlockReturn::Returned(Some(returned)));
            }
            None => return Ok(BlockReturn::Returned(None)),
        },
        Statement::Class(class) => {
            let env: &RefCell<Environment> = environment.borrow();

            env.borrow().define_variable(
                line_number,
                class.name.clone(),
                ExpressionLiteral::Reference(Reference::ClassReference(ClassReference {
                    name: class.name,
                })),
            )?;
        }
    }

    Ok(BlockReturn::NoReturn)
}

pub fn interpret_expression_tree(
    environment: EnvironmentRef,
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
                let left_parsed = interpret_expression_tree(environment.clone(), *left)?;
                let right_parsed = interpret_expression_tree(environment.clone(), *right)?;

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
                let left_parsed = interpret_expression_tree(environment.clone(), *left)?;
                let right_parsed = interpret_expression_tree(environment.clone(), *right)?;

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
                let left_parsed = interpret_expression_tree(environment.clone(), *left)?;
                let right_parsed = interpret_expression_tree(environment.clone(), *right)?;

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
                let left_parsed = interpret_expression_tree(environment.clone(), *left)?;
                let right_parsed = interpret_expression_tree(environment.clone(), *right)?;

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
                let left_parsed = interpret_expression_tree(environment.clone(), *left)?;
                let right_parsed = interpret_expression_tree(environment.clone(), *right)?;

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
                let left_parsed = interpret_expression_tree(environment.clone(), *left)?;
                let right_parsed = interpret_expression_tree(environment.clone(), *right)?;

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
                let left_parsed = interpret_expression_tree(environment.clone(), *left)?;
                let right_parsed = interpret_expression_tree(environment.clone(), *right)?;

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
                let left_parsed = interpret_expression_tree(environment.clone(), *left)?;
                let right_parsed = interpret_expression_tree(environment.clone(), *right)?;

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
                let left_parsed = interpret_expression_tree(environment.clone(), *left)?;
                let right_parsed = interpret_expression_tree(environment.clone(), *right)?;

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
                let left_parsed = interpret_expression_tree(environment.clone(), *left)?;
                let right_parsed = interpret_expression_tree(environment.clone(), *right)?;

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

            Operation::And(LogicalOperation {
                left,
                right,
                line_number: _,
            }) => {
                let left = interpret_expression_tree(environment.clone(), *left)?;

                if !is_truthy(environment.clone(), Expression::Literal(left.clone()))? {
                    return Ok(left);
                }

                return Ok(interpret_expression_tree(environment, *right)?);
            }

            Operation::Or(LogicalOperation {
                left,
                right,
                line_number: _,
            }) => {
                let left = interpret_expression_tree(environment.clone(), *left)?;

                if is_truthy(environment.clone(), Expression::Literal(left.clone()))? {
                    return Ok(left);
                }

                return Ok(interpret_expression_tree(environment, *right)?);
            }
        },
        Expression::Variable(var) => {
            let env: &RefCell<Environment> = environment.borrow();

            env.borrow().get_variable_with_depth(var)
        }

        Expression::Assign(expression_variable, right_side_tree) => {
            let expression_value =
                interpret_expression_tree(environment.clone(), *right_side_tree)?;

            let env: &RefCell<Environment> = environment.borrow();
            env.borrow().set_variable(
                expression_variable.line_number,
                expression_variable.identifier_name,
                expression_value,
            )
        }
        Expression::Call(line_number, callable, arguments) => {
            match interpret_expression_tree(environment.clone(), *callable)? {
                ExpressionLiteral::Reference(reference) => match reference {
                    Reference::CallableReference(callable_reference) => {
                        evaluate_callable_reference(
                            environment,
                            callable_reference,
                            arguments,
                            line_number,
                        )
                    }
                    Reference::ClassReference(class) => {
                        let instance = InstanceReference::instantiate(class);

                        let reference = Reference::InstanceReference(instance);

                        Ok(ExpressionLiteral::Reference(reference))
                    }
                    Reference::InstanceReference(_) => Err(RuntimeError {
                        line_number,
                        message: format!("Can't call a class instance, only a class type"),
                    }),
                },
                invalid_type => Err(RuntimeError {
                    line_number,
                    message: format!(
                        "Expected function or method reference, found {}",
                        invalid_type
                    ),
                }),
            }
        }
        Expression::Get(line_number, object_expression, identifier) => {
            let object = interpret_expression_tree(environment, *object_expression)?;

            match object {
                ExpressionLiteral::Reference(reference) => match reference {
                    Reference::InstanceReference(instance) => {
                        instance.get_property(line_number, &identifier)
                    }
                    Reference::ClassReference(_) => Err(RuntimeError {
                        line_number,
                        message: format!("Can't access properties on a class, only an instance"),
                    }),
                    _ => Err(RuntimeError {
                        line_number,
                        message: format!("Can only access properties on a instance"),
                    }),
                },
                _ => Err(RuntimeError {
                    line_number,
                    message: format!("Can only access properties on a instance"),
                }),
            }
        }
        Expression::Set(line_number, object_expression, identifier, value) => {
            let object = interpret_expression_tree(environment.clone(), *object_expression)?;

            match object {
                ExpressionLiteral::Reference(reference) => match reference {
                    Reference::InstanceReference(instance) => instance
                        .set_property(identifier, interpret_expression_tree(environment, *value)?),
                    Reference::ClassReference(_) => Err(RuntimeError {
                        line_number,
                        message: format!("Can't access properties on a class, only an instance"),
                    }),
                    _ => Err(RuntimeError {
                        line_number,
                        message: format!("Can only access properties on a instance"),
                    }),
                },
                _ => Err(RuntimeError {
                    line_number,
                    message: format!("Can only access properties on a instance"),
                }),
            }
        }
    };

    return Ok(literal?);
}

fn evaluate_callable_reference(
    environment: EnvironmentRef,
    reference: CallableReference,
    arguments: Vec<Expression>,
    line_number: usize,
) -> Result<ExpressionLiteral, RuntimeError> {
    let provided_arity = arguments.len();

    if provided_arity != reference.arity {
        return Err(RuntimeError {
            line_number,
            message: format!(
                "Expected {} arguments, received {}",
                reference.arity, provided_arity
            ),
        });
    };

    let evaluated_args = arguments
        .into_iter()
        .map(|expr| interpret_expression_tree(environment.clone(), expr))
        .collect::<Result<Vec<ExpressionLiteral>, RuntimeError>>()?;

    let ret = Fn::call(reference.subroutine.deref(), (line_number, evaluated_args))?;

    match ret {
        BlockReturn::Returned(Some(value)) => Ok(value),
        BlockReturn::Returned(None) | BlockReturn::NoReturn => Ok(ExpressionLiteral::Nil),
    }
}

pub fn is_truthy(environment: EnvironmentRef, expr: Expression) -> Result<bool, RuntimeError> {
    match expr {
        Expression::Literal(literal) => match literal {
            ExpressionLiteral::Number(number) => Ok(number != 0.0),
            ExpressionLiteral::String(str) => Ok(str.len() > 0),
            ExpressionLiteral::True => Ok(true),
            ExpressionLiteral::False => Ok(false),
            ExpressionLiteral::Nil => Ok(false),
            ExpressionLiteral::Reference(_) => Ok(true),
        },
        tree => {
            let evaluated_tree = interpret_expression_tree(environment.clone(), tree)?;

            is_truthy(environment, Expression::Literal(evaluated_tree))
        }
    }
}

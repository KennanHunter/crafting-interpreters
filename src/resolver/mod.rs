mod scope_stack;

use std::collections::HashMap;

use scope_stack::ScopeStack;

use crate::{
    errors::ResolvingError,
    parser::{
        statements::{FunStatement, Statement},
        ParsedStep, ParsingResult,
    },
    tree::expression::{Expression, ExpressionVariable, Operation},
};

type ResolveResult = Result<(), ResolvingError>;
pub type VariableMap = HashMap<ExpressionVariable, usize>;

pub fn resolve(steps: Vec<ParsingResult>) -> Result<VariableMap, ResolvingError> {
    let mut scopes = ScopeStack::new();

    scopes.begin_scope();

    resolve_steps(&mut scopes, steps)?;

    scopes.end_scope();

    Ok(scopes.locals)
}

fn resolve_steps(scopes: &mut ScopeStack, steps: Vec<ParsingResult>) -> ResolveResult {
    for step in steps {
        resolve_step(scopes, step.unwrap())?;
    }

    Ok(())
}

fn resolve_step(scope_stack: &mut ScopeStack, step: ParsedStep) -> ResolveResult {
    match step {
        ParsedStep::Expression(expr) => resolve_expression(scope_stack, expr)?,
        ParsedStep::Statement(stmt) => resolve_statement(scope_stack, stmt)?,
        ParsedStep::Block(block) => {
            scope_stack.begin_scope();

            resolve_steps(scope_stack, block)?;

            scope_stack.end_scope();
        }
    }

    Ok(())
}

fn resolve_expression(scope_stack: &mut ScopeStack, expr: Expression) -> ResolveResult {
    match expr {
        Expression::Literal(_) => (),
        Expression::Operation(operation) => resolve_operation(scope_stack, operation)?,
        Expression::Grouping(group) => resolve_expression(scope_stack, *group)?,
        Expression::Variable(var) => {
            if scope_stack.is_locally_declared(&var.identifier_name) {
                return Err(ResolvingError {
                    line_number: var.line_number,
                    message: format!(
                        "Can't read local variable ({}) in its own initializer",
                        var.identifier_name
                    ),
                });
            }

            scope_stack.encode_resolved_variable(var);
        }
        Expression::Assign(var, value) => {
            resolve_expression(scope_stack, *value)?;
            scope_stack.encode_resolved_variable(var);
        }
        Expression::Call(_, callee, arguments) => {
            resolve_expression(scope_stack, *callee)?;

            for arg in arguments {
                resolve_expression(scope_stack, arg)?;
            }
        }
        Expression::Get(_, expr, _) => {
            resolve_expression(scope_stack, *expr)?;
        }
        Expression::Set(_, expr, _, value) => {
            resolve_expression(scope_stack, *expr)?;

            resolve_expression(scope_stack, *value)?;
        }
    }

    Ok(())
}

fn resolve_operation(
    scope_stack: &mut ScopeStack,
    operation: Operation,
) -> Result<(), ResolvingError> {
    Ok(match operation {
        Operation::Not(operation) | Operation::Negate(operation) => {
            resolve_expression(scope_stack, *operation.operand)?
        }
        Operation::Equal(op) | Operation::NotEqual(op) => {
            resolve_expression(scope_stack, *op.left)?;
            resolve_expression(scope_stack, *op.right)?;
        }
        Operation::Less(op)
        | Operation::LessEqual(op)
        | Operation::Greater(op)
        | Operation::GreaterEqual(op) => {
            resolve_expression(scope_stack, *op.left)?;
            resolve_expression(scope_stack, *op.right)?;
        }
        Operation::Plus(op) | Operation::Minus(op) => {
            resolve_expression(scope_stack, *op.left)?;
            resolve_expression(scope_stack, *op.right)?;
        }
        Operation::Divide(op) | Operation::Multiply(op) => {
            resolve_expression(scope_stack, *op.left)?;
            resolve_expression(scope_stack, *op.right)?;
        }
        Operation::And(op) | Operation::Or(op) => {
            resolve_expression(scope_stack, *op.left)?;
            resolve_expression(scope_stack, *op.right)?;
        }
    })
}

fn resolve_statement(scope_stack: &mut ScopeStack, stmt: Statement) -> ResolveResult {
    match stmt {
        Statement::Print(expr) => {
            resolve_expression(scope_stack, expr)?;
        }
        Statement::Variable(name, expr) => {
            if scope_stack.is_locally_declared(&name) || scope_stack.is_locally_defined(&name) {
                return Err(ResolvingError {
                    // TODO: Get this line number somehow
                    line_number: 0,
                    message: format!("Variable {name} already exists in this scope"),
                });
            }

            scope_stack.declare(name.clone());

            resolve_expression(scope_stack, expr)?;

            scope_stack.define(name);
        }
        Statement::If(stmt) => {
            resolve_expression(scope_stack, stmt.condition)?;
            resolve_step(scope_stack, *stmt.then_statement)?;

            if let Some(else_stmt) = stmt.else_statement {
                resolve_step(scope_stack, *else_stmt)?;
            }
        }
        Statement::While(while_statement) => {
            resolve_expression(scope_stack, while_statement.condition)?;
            resolve_step(scope_stack, *while_statement.body)?;
        }
        Statement::Fun(function_statement) => {
            scope_stack.declare(function_statement.name.clone());
            scope_stack.define(function_statement.name.clone());

            resolve_function(scope_stack, function_statement)?;
        }
        Statement::Return(expr) => {
            if expr.is_some() {
                resolve_expression(scope_stack, expr.unwrap())?;
            }
        }
        Statement::Class(class) => {
            scope_stack.declare(class.name.clone());
            scope_stack.define(class.name);
        }
    }

    Ok(())
}

// TODO: refactor for methods
fn resolve_function(
    scope_stack: &mut ScopeStack,
    function_statement: FunStatement,
) -> ResolveResult {
    scope_stack.declare(function_statement.name.clone());

    scope_stack.define(function_statement.name);

    for param in function_statement.parameters {
        scope_stack.declare(param.clone());
        scope_stack.define(param);
    }

    scope_stack.begin_scope();

    // NOTE: The block generated by resolve_step handles it's own scoping
    resolve_step(scope_stack, *function_statement.body)?;

    scope_stack.end_scope();

    Ok(())
}

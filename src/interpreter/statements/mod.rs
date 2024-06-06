use std::io::Write;

use crate::{errors::RuntimeError, tree::expression::Expression};

use super::{environment::EnvironmentRef, interpret_expression_tree};

pub fn interpret_print(
    environment: EnvironmentRef,
    enclosed_expression: Expression,
) -> Result<(), RuntimeError> {
    let mut evaluated_string =
        interpret_expression_tree(environment, enclosed_expression)?.to_string();

    evaluated_string.push('\n');

    match std::io::stdout().write(evaluated_string.as_bytes()) {
        Err(_) => {
            return Err(RuntimeError {
                line_number: 0,
                message: "Failed to access stdout".to_owned(),
            });
        }
        Ok(_) => Ok(()),
    }
}

pub fn interpret_variable_definition(
    environment: EnvironmentRef,
    line_number: usize,
    name: String,
    value: Expression,
) -> Result<(), RuntimeError> {
    let evaluated_value = interpret_expression_tree(environment.clone(), value)?;

    environment
        .borrow()
        .define_variable(line_number, name, evaluated_value)?;

    Ok(())
}
use crate::{errors::RuntimeError, tree::expression::Expression};

use super::{environment::EnvironmentRef, interpret_expression_tree};

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

use std::io::Write;

use crate::{errors::RuntimeError, tree::expression::Expression};

use super::interpret_expression_tree;

pub fn interpret_print(enclosed_expression: Expression) -> Result<(), RuntimeError> {
    let mut evaluated_string = interpret_expression_tree(enclosed_expression)?.to_string();

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

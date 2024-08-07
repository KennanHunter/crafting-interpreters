use std::{rc::Rc, time::UNIX_EPOCH};

use crate::{
    errors::RuntimeError, interpreter::types::BlockReturn, tree::expression::ExpressionLiteral,
};

use super::CallableReference;

pub fn create_native_now() -> CallableReference {
    CallableReference {
        arity: 0,
        subroutine: Rc::new(|_line_number, _args| -> Result<BlockReturn, RuntimeError> {
            let timestamp = UNIX_EPOCH.elapsed().unwrap().as_millis() as f64;
            let seconds = timestamp / 1000f64;

            Ok(BlockReturn::from(ExpressionLiteral::Number(seconds)))
        }),
    }
}

pub fn create_native_print() -> CallableReference {
    CallableReference {
        arity: 1,
        subroutine: Rc::new(|line_number, args| -> Result<BlockReturn, RuntimeError> {
            match args.first() {
                Some(expr) => {
                    let mut evaluated_string = expr.to_string();

                    evaluated_string.push('\n');

                    #[cfg(not(target_family = "wasm"))]
                    {
                        if std::io::Write::write(
                            &mut std::io::stdout(),
                            evaluated_string.as_bytes(),
                        )
                        .is_err()
                        {
                            return Err(RuntimeError {
                                line_number,
                                message: "Failed to access stdout".to_owned(),
                            });
                        }
                    }

                    #[cfg(target_family = "wasm")]
                    {
                        crate::logging::report!("{}", evaluated_string);
                    }

                    Ok(BlockReturn::NoReturn)
                }
                None => Err(RuntimeError {
                    line_number,
                    message: "Must provide value to print, perhaps you meant print(\"\")?"
                        .to_owned(),
                }),
            }
        }),
    }
}

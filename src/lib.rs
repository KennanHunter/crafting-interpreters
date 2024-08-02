#![feature(fn_traits)]
#![feature(duration_millis_float)]

#[macro_use]
pub mod logging;

pub mod errors;
pub mod interpreter;
pub mod parser;
pub mod resolver;
pub mod scanner;
pub mod tokens;
pub mod tree;
pub mod tests;

use std::time;

use interpreter::interpret;
use parser::{parse, ParsingResult};
use resolver::{resolve, VariableMap};
use scanner::scan_tokens;

use wasm_bindgen::prelude::*;

/// Core function that takes in the raw source code and does stuff
///
// TODO: Rework this whole function
#[wasm_bindgen]
pub fn run(source: &str) {
    report!(
        "Parsing {} characters: \n {}",
        source.len(),
        source.escape_default()
    );

    let tokens = scan_tokens(source).unwrap();

    report_progress!("Scanned {} tokens", tokens.len());

    let syntax_tree: Vec<ParsingResult> = parse(tokens);

    report_progress!("Parsed tokens into {} blocks", syntax_tree.len());

    // TODO: Check for errors in sub blocks
    match syntax_tree
        .iter()
        .filter_map(|parsed_block| parsed_block.clone().err())
        .collect::<Vec<_>>()
        .as_slice()
    {
        [] => {}
        errors => {
            for error in errors {
                report_error!(
                    "Parsing error appeared at line number {} with issue: {}",
                    error.line_number,
                    error.message
                );
            }

            return;
        }
    }

    let resolved_variable_map: VariableMap = match resolve(syntax_tree.clone()) {
        Ok(map) => map,
        Err(err) => {
            report_error!(
                "Failed to resolve at line {} with message {}",
                err.line_number,
                err.message
            );

            return;
        }
    };

    let starting_time = time::Instant::now();

    report!("\n---- output ----");

    match interpret(resolved_variable_map, syntax_tree) {
        Ok(_) => report!(
            "---- program finished ----\n\nExecuted in {}μs",
            starting_time.elapsed().as_micros()
        ),
        Err(err) => {
            report!(
                "---- program errored ----\n\nExperienced runtime error at line {} with message:\n {}",
                err.line_number, err.message
            )
        }
    }
}

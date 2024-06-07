#![feature(fn_traits)]
#![feature(duration_millis_float)]

pub mod errors;
pub mod interpreter;
pub mod parser;
pub mod resolver;
pub mod scanner;
pub mod tokens;
pub mod tree;

use std::time;

use interpreter::interpret;
use parser::{parse, ParsingResult};
use resolver::{resolve, VariableMap};
use scanner::scan_tokens;

use wasm_bindgen::prelude::*;

/// Core function that takes in the raw source code and does stuff
///
// #[warn(unused_variables)]
// TODO: Rework this whole function
#[wasm_bindgen]
pub fn run(source: &str) {
    let tokens = scan_tokens(source).unwrap(); 

    eprintln!("Scanned {} tokens", tokens.len());

    let syntax_tree: Vec<ParsingResult> = parse(tokens);

    eprintln!("Parsed tokens into {} blocks", syntax_tree.len());

    // TODO: Check for errors in sub blocks
    let parsing_errors = syntax_tree
        .iter()
        .filter(|&parsed_block| parsed_block.is_err());

    let mut has_parsing_error = false;

    for error_res in parsing_errors {
        let error = error_res.clone().unwrap_err();

        eprintln!(
            "Parsing error appeared at line number {} with issue: {}",
            error.line_number, error.message
        );

        has_parsing_error = true
    }

    if has_parsing_error {
        return;
    }

    let resolved_variable_map: VariableMap = match resolve(syntax_tree.clone()) {
        Ok(map) => map,
        Err(err) => {
            eprintln!(
                "Failed to resolve at line {} with message {}",
                err.line_number, err.message
            );

            return;
        }
    };

    let starting_time = time::Instant::now();

    eprintln!("\n---- output ----");

    match interpret(resolved_variable_map, syntax_tree) {
        Ok(_) => eprintln!(
            "---- program finished ----\n\nExecuted in {}μs",
            starting_time.elapsed().as_micros()
        ),
        Err(err) => {
            eprintln!(
                "---- program errored ----\n\nExperienced runtime error at line {} with message:\n {}",
                err.line_number, err.message
            )
        }
    }
}

#[wasm_bindgen]
pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

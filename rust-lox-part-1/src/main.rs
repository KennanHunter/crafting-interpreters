#![feature(fn_traits)]
#![feature(duration_millis_float)]

pub mod errors;
pub mod interpreter;
pub mod parser;
pub mod scanner;
pub mod tokens;
pub mod tree;

use std::{env, fs, time};

use interpreter::interpret;
use parser::{parse, ParsingResult};
use scanner::scan_tokens;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_name: Result<&str, &str> = match args.len() {
        3.. => Err(""),
        2 => {
            let file_name = args.get(1).unwrap();

            println!("Target file: {}", file_name);

            Ok(file_name.as_str())
        }
        // TODO: Use the run function to create a REPL
        1 => Ok("./demo.lox"),
        0 => unreachable!("Will always have at least the path of the executable"),
    };

    let source_file = read_file(file_name.unwrap());

    run(source_file);
}

fn read_file(file_name: &str) -> String {
    let file_contents = fs::read_to_string(file_name).expect("File name is invalid");

    return file_contents;
}

/// Core function that takes in the raw source code and does stuff
///
// #[warn(unused_variables)]
fn run(source: String) {
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
            "Error appeared at line number {} with issue: {}",
            error.line_number, error.message
        );

        has_parsing_error = true
    }

    if has_parsing_error {
        return;
    }

    let starting_time = time::Instant::now();

    eprintln!("\n---- output ----");

    match interpret(syntax_tree) {
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

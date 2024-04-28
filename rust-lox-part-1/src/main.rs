#![feature(anonymous_lifetime_in_impl_trait)]
#![feature(iter_collect_into)]

pub mod errors;
pub mod parser;
pub mod scanner;
pub mod tokens;
pub mod tree;

use std::{env, fs};

use parser::parse_tokens;
use scanner::scan_tokens;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_name: Result<&str, &str> = match args.len() {
        3.. => Err(""),
        2 => {
            let file_name = args.get(1).unwrap();

            println!("Target file: {:?}", file_name);

            Ok(file_name.as_str())
        }
        // TODO: Use the run function to create a REPL
        1 => Ok("./demo.lox"),
        0 => unreachable!("Will always have at least the path of the executable"),
    };

    let source_file = run_file(file_name.unwrap());

    run(source_file);
}

fn run_file(file_name: &str) -> String {
    let file_contents = fs::read_to_string(file_name).expect("File name is invalid");

    if file_contents.lines().count() >= u64::MAX.try_into().unwrap() {
        panic!("Why the fuck is this file so large")
    };

    return file_contents;
}

/// Core function that takes in the raw source code and does stuff
///
fn run(source: String) {
    let tokens = scan_tokens(source).unwrap();

    let syntax_tree = parse_tokens(tokens);

    println!("{:#?}", syntax_tree)
}

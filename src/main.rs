use std::{env, fs};

use rust_lox::run;

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

    run(&source_file);
}

fn read_file(file_name: &str) -> String {
    let file_contents = fs::read_to_string(file_name).expect("File name is invalid");

    return file_contents;
}

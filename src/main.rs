#[allow(unused_imports)]
use rust_lox;

#[cfg(not(target_family = "wasm"))]
fn main() {
    use rust_lox::run;

    let args: Vec<String> = std::env::args().collect();

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

    let source_file: String = read_file(file_name.unwrap());

    run(&source_file);
}

#[cfg(target_family = "wasm")]
fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[cfg(not(target_family = "wasm"))]
fn read_file(file_name: &str) -> String {
    std::fs::read_to_string(file_name).expect("File name is invalid")
}

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct ParsingError {
    pub line_number: usize,
    pub message: String,
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

pub fn error(line_number: u64, message: &str) {
    report(line_number, "", message)
}

fn report(line_number: u64, location: &str, message: &str) {
    println!(
        "There was an error found in line {},\n at \"{}\",\n {}",
        line_number, location, message
    )
}

use std::fmt;

/// Error to report issues at the raw scanning stage
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ScanningError {
    pub line_number: usize,
    pub message: String,
}

impl fmt::Display for ScanningError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "There was an error found in line {},\n \n {}",
            self.line_number, self.message
        )
    }
}

/// Error to report issues at the raw scanning stage
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RuntimeError {
    pub line_number: usize,
    pub message: String,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Runtime error at {},\n \n {}",
            self.line_number, self.message
        )
    }
}

/// Error to report issues at the parsing stage
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ParsingError {
    pub line_number: usize,
    pub message: String,
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Parsing failed at line {}, \n {}",
            self.line_number, self.message
        )
    }
}

/// Error to report issues at the parsing stage
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ResolvingError {
    pub line_number: usize,
    pub message: String,
}

impl fmt::Display for ResolvingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Resolution failed at line {}, \n {}",
            self.line_number, self.message
        )
    }
}

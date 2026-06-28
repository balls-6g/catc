use crate::error::err_trait::CompilerError;
use colored::Colorize;

pub struct SyntaxError {
    message: String,
    line: usize,
    column: usize,
}

impl CompilerError for SyntaxError {
    fn new(message: String, line: usize, column: usize) -> Self {
        SyntaxError {
            message,
            line,
            column,
        }
    }

    fn msg(&self) {
        println!("{}: SyntaxError:", "Error".red());
        println!("  line: {} column: {}", self.line, self.column);
        println!("{}  message: \n    {}", "Error".red(), self.message);
    }
}

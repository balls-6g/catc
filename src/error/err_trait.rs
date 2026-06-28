pub trait CompilerError {
    fn new(message: String, line: usize, column: usize) -> Self;
    fn msg(&self);
}

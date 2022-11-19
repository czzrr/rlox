pub struct ErrorHandler {
    had_error: bool,
}

impl ErrorHandler {
    pub fn new() -> ErrorHandler {
        ErrorHandler { had_error: false }
    }

    pub fn line_error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    pub fn report(&mut self, line: usize, location: &str, message: &str) {
        println!("[line {}] Error{}: {}", line, location, message);
        self.had_error = true;
    }

    pub fn had_error(&self) -> bool {
        self.had_error
    }

    pub fn reset_error(&mut self) {
        self.had_error = false;
    }
}

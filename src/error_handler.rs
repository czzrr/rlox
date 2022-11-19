use crate::{parser::ParseErr, token::Token, token_type::TokenType, interpreter::InterpErr};

pub struct ErrorHandler {
    had_error: bool,
    had_runtime_error: bool,
}

impl ErrorHandler {
    pub fn new() -> ErrorHandler {
        ErrorHandler { had_error: false, had_runtime_error: false }
    }

    pub fn line_error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    pub fn report(&mut self, line: usize, location: &str, message: &str) {
        println!("[line {}] Error {}: {}", line, location, message);
        self.had_error = true;
    }

    pub fn had_error(&self) -> bool {
        self.had_error
    }

    pub fn had_runtime_error(&self) -> bool {
        self.had_runtime_error
    }

    pub fn reset_error(&mut self) {
        self.had_error = false;
    }

    pub fn reset_runtime_error(&mut self) {
        self.had_runtime_error = false;
    }

    pub fn parse_error(&mut self, token: &Token, parse_err: ParseErr) {
        if token.ty == TokenType::Eof {
            self.report(token.line, "at end", &parse_err.to_string());
        } else {
            self.report(token.line, &format!("at '{}'", token.lexeme), &parse_err.to_string());
        }
    }

    pub fn runtime_error(&mut self, token: &Token, interp_err: InterpErr) {
        println!("{}\n[line {}]", interp_err, token.line);
        self.had_runtime_error = true;
    }
}

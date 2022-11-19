use crate::{parser::ParseErr, token::Token, token_type::TokenType};

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
        println!("[line {}] Error {}: {}", line, location, message);
        self.had_error = true;
    }

    pub fn had_error(&self) -> bool {
        self.had_error
    }

    pub fn reset_error(&mut self) {
        self.had_error = false;
    }

    pub fn parse_error(&mut self, token: &Token, parse_err: ParseErr) {
        let message = match parse_err {
            ParseErr::ExpectMissingRightParen => "Expect ')' after expression.",
            ParseErr::ExpectExpr => "Expect expression.",
            ParseErr::MissingSemicolonAfterExprStmt => "Expect ';' after value.",
            ParseErr::Sync => "Sync error.",
            ParseErr::ExpectVarName => "Expect variable name.",
            ParseErr::InvalidAssignmentTarget => "Invalid assignment target.",
            ParseErr::ExpectRightBraceAfterBlock => "Expect '}' after block.",
            //_ => todo!()
        };

        if token.ty == TokenType::Eof {
            self.report(token.line, " at end", message);
        } else {
            self.report(token.line, &format!(" at '{}'", token.lexeme), message);
        }
    }
}

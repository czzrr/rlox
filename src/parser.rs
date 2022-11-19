use crate::{expr::Expr, token::{Token, Literal}, token_type::TokenType, error_handler::ErrorHandler};
use thiserror::Error;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[derive(Debug, Clone, Copy, Error)]
pub enum ParseErr {
    #[error("Expect ')' after expression.")]
    ExpectMissingRightParen,
    #[error("Expect expression.")]
    ExpectExpr,
    #[error("Expect ';' after value.")]
    MissingSemicolonAfterExprStmt,
    #[error("Sync error.")]
    Sync,
    //#[error("Expect variable name.")]
    //ExpectVarName,
    //#[error("Invalid assignment target.")]
    //InvalidAssignmentTarget,
    //#[error("Expect '}' after block.")]
    //ExpectRightBraceAfterBlock,
}

type ParseResult<T> = Result<T, (Token, ParseErr)>;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self, error_handler: &mut ErrorHandler) -> Option<Box<Expr>> {
        let expr = self.expression();
        match expr {
            Ok(expr) => Some(expr),
            Err((token, parse_err)) => {
                error_handler.parse_error(&token, parse_err);
                None
            }
        }
    }

    fn expression(&mut self) -> ParseResult<Box<Expr>> {
        self.equality()
    }

    fn equality(&mut self) -> ParseResult<Box<Expr>> {
        let mut expr = self.comparison()?;
        while self.matches(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Box::new(Expr::Binary(expr, operator, right));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> ParseResult<Box<Expr>> {
        let mut expr = self.term()?;
        while self.matches(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Box::new(Expr::Binary(expr, operator, right));
        }

        Ok(expr)
    }

    fn term(&mut self) -> ParseResult<Box<Expr>> {
        let mut expr = self.factor()?;
        while self.matches(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Box::new(Expr::Binary(expr, operator, right));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> ParseResult<Box<Expr>> {
        let mut expr = self.unary()?;
        loop {
            if !self.matches(vec![TokenType::Slash, TokenType::Star]) {
                break;
            }
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Box::new(Expr::Binary(expr, operator, right));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> ParseResult<Box<Expr>> {
        if self.matches(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            Ok(Box::new(Expr::Unary(operator, right)))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> ParseResult<Box<Expr>> {
        if self.matches(vec![TokenType::False]) {
            return Ok(Box::new(Expr::Literal(Literal::Bool(false))));
        }
        if self.matches(vec![TokenType::True]) {
            return Ok(Box::new(Expr::Literal(Literal::Bool(true))));
        }
        if self.matches(vec![TokenType::Nil]) {
            return Ok(Box::new(Expr::Literal(Literal::Nil)));
        }
        if self.matches(vec![TokenType::Number, TokenType::String]) {
            return Ok(Box::new(Expr::Literal(
                self.previous().literal.as_ref().unwrap().clone(),
            )));
        }
        // if self.matches(vec![TokenType::Identifier]) {
        //     return Ok(Box::new(Expr::Variable(self.previous().clone())));
        // }
        if self.matches(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            if self.consume(TokenType::RightParen).is_some() {
                return Ok(Box::new(Expr::Grouping(expr)));
            }
            return self.error(ParseErr::ExpectMissingRightParen);
        }
        self.error(ParseErr::ExpectExpr)
    }

    fn matches(&mut self, token_types: impl IntoIterator<Item = TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, token_type: TokenType) -> Option<&Token> {
        if self.check(token_type) {
            Some(self.advance())
        } else {
            None
        }
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().ty == token_type
        }
    }

    /* Return the current token and advance the cursor. */
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().ty == TokenType::Eof
    }

    /* Return the current token. */
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn error<T>(&self, err: ParseErr) -> ParseResult<T> {
        Err((self.peek().clone(), err))
    }

}

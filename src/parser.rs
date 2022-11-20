use crate::{
    error_handler::ErrorHandler,
    expr::Expr,
    stmt::Stmt,
    token::{Literal, Token},
    token_type::TokenType,
};
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
    #[error("Expect variable name.")]
    ExpectVarName,
    #[error("Invalid assignment target.")]
    InvalidAssignmentTarget,
    #[error("Expect '}}' after block.")]
    ExpectRightBraceAfterBlock,
}

type ParseResult<T> = Result<T, (Token, ParseErr)>;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self, error_handler: &mut ErrorHandler) -> Option<Vec<Stmt>> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            let stmt = self.declaration();
            match stmt {
                Ok(stmt) => statements.push(stmt),
                Err((token, parse_err)) => {
                    error_handler.parse_error(&token, parse_err);
                    return None;
                }
            }
        }
        Some(statements)
    }

    fn expression(&mut self) -> ParseResult<Box<Expr>> {
        self.assignment()
    }

    fn declaration(&mut self) -> ParseResult<Stmt> {
        let res = {
            if self.matches(vec![TokenType::Var]) {
                self.var_declaration()
            } else {
                self.statement()
            }
        };

        if res.is_err() {
            self.synchronize();
            self.error(ParseErr::Sync)
        } else {
            res
        }
    }

    fn statement(&mut self) -> ParseResult<Stmt> {
        if self.matches(vec![TokenType::Print]) {
            self.print_statement()
        } else if self.matches(vec![TokenType::LeftBrace]) {
            self.block().map(|stmts| Stmt::Block(stmts))
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> ParseResult<Stmt> {
        let value = self.expression().map_err(|e| e)?;
        if self.consume(TokenType::Semicolon).is_some() {
            Ok(Stmt::PrintStmt(*value))
        } else {
            self.error(ParseErr::MissingSemicolonAfterExprStmt)
        }
    }

    fn var_declaration(&mut self) -> ParseResult<Stmt> {
        let name = self.consume(TokenType::Identifier);
        match name {
            Some(name) => {
                let name = name.clone();
                let mut initializer = Box::new(Expr::Literal(Literal::Nil));
                if self.matches(vec![TokenType::Equal]) {
                    initializer = self.expression()?;
                }
                if self.consume(TokenType::Semicolon).is_some() {
                    Ok(Stmt::Var(name, Some(*initializer)))
                } else {
                    self.error(ParseErr::MissingSemicolonAfterExprStmt)
                }
            }
            _ => self.error(ParseErr::ExpectVarName),
        }
    }

    fn expression_statement(&mut self) -> ParseResult<Stmt> {
        let value = self.expression().map_err(|e| e)?;
        if self.consume(TokenType::Semicolon).is_some() {
            Ok(Stmt::ExprStmt(*value))
        } else {
            self.error(ParseErr::MissingSemicolonAfterExprStmt)
        }
    }

    fn block(&mut self) -> ParseResult<Vec<Stmt>> {
        let mut statements = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        if self.consume(TokenType::RightBrace).is_none() {
            self.error(ParseErr::ExpectRightBraceAfterBlock)
        } else {
            Ok(statements)
        }
    }

    fn assignment(&mut self) -> ParseResult<Box<Expr>> {
        let expr = self.equality()?;

        if self.matches(vec![TokenType::Equal]) {
            let equals = self.previous().to_owned();
            if let Expr::Variable(name) = *expr {
                let value = self.assignment()?;
                Ok(Box::new(Expr::Assign(name, value)))
            } else {
                Err((equals, ParseErr::InvalidAssignmentTarget))
            }
        } else {
            Ok(expr)
        }
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
        if self.matches(vec![TokenType::Identifier]) {
            return Ok(Box::new(Expr::Variable(self.previous().clone())));
        }
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

    fn synchronize(&mut self) {
        while !self.is_at_end() {
            match self.peek().ty {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => break,
                _ => {
                    self.advance();
                }
            }
        }
    }
}

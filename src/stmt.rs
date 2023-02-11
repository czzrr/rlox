use crate::{expr::Expr, token::Token};

#[derive(Debug, Clone)]
pub enum Stmt {
    ExprStmt(Expr),
    PrintStmt(Expr),
    Var(Token, Option<Expr>),
    While(Expr, Box<Stmt>),
    Block(Vec<Stmt>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
}

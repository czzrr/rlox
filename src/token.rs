use crate::token_type::TokenType;
use std::fmt;

pub struct Token {
    ty: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: usize,
}

impl Token {
    pub fn new(ty: TokenType, lexeme: String, literal: Option<Literal>, line: usize) -> Token {
        Token {
            ty,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {} {:?}", self.ty, self.lexeme, self.literal)
    }
}

#[derive(Debug)]
pub enum Literal {
    Id(String),
    Str(String),
    Bool(bool),
    Double(f64),
    Nil,
}

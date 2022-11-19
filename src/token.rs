use crate::token_type::TokenType;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub ty: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
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

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Id(String),
    Str(String),
    Bool(bool),
    Double(f64),
    Nil,
}

impl ToString for Literal {
    fn to_string(&self) -> String {
        use Literal::*;
        match self {
            Id(s) | Str(s) => s.to_owned(),
            Bool(b) => b.to_string(),
            Double(n) => n.to_string(),
            Nil => "nil".to_owned(),
        }
    }
}

use std::io::Cursor;

pub struct Scanner<'a> {
    start: Cursor<&'a [u8]>,
    current: Cursor<&'a [u8]>,
    line: usize
}

impl Scanner<'_> {
    pub fn new(source: &[u8]) -> Scanner<'_> {
        Scanner {
            start: Cursor::new(source),
            current: Cursor::new(source),
            line: 1
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();

        self.start = self.current.clone();

        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }

        let c = self.advance();
        match c {
            '(' => return self.make_token(TokenType::LeftParen),
            ')' => return self.make_token(TokenType::RightParen),
            '{' => return self.make_token(TokenType::LeftBrace),
            '}' => return self.make_token(TokenType::RightBrace),
            ',' => return self.make_token(TokenType::Comma),
            '.' => return self.make_token(TokenType::Dot),
            '-' => return self.make_token(TokenType::Minus),
            '+' => return self.make_token(TokenType::Plus),
            ';' => return self.make_token(TokenType::Semicolon),
            '*' => return self.make_token(TokenType::Star),
            '!' => {
                if self.matches('=') {
                    self.make_token(TokenType::BangEqual);
                } else {
                    self.make_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.matches('=') {
                    self.make_token(TokenType::EqualEqual);
                } else {
                    self.make_token(TokenType::Equal);
                }
            }
            '<' => {
                if self.matches('=') {
                    self.make_token(TokenType::LessEqual);
                } else {
                    self.make_token(TokenType::Less);
                }
            }
            '>' => {
                if self.matches('=') {
                    self.make_token(TokenType::GreaterEqual);
                } else {
                    self.make_token(TokenType::Greater);
                }
            }
            '\n' => self.line += 1,
            '/' => {
                if self.matches('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.make_token(TokenType::Slash);
                }
            }
            '"' => self.string(),
            c if self.is_digit(c) => self.number(),
            c if self.is_alpha(c) => self.identifier(),
        }

        self.error_token("Unexpected character")
    }

    fn advance(&mut self) -> char {
        let c = self.current.get_ref()[self.current.position() as usize];
        self.current.set_position(self.current.position() - 1);
        c as char
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.current.get_ref()[self.current.position() as usize] as char != expected {
            return false;
        }

        self.current.set_position(self.current.position() + 1);

        true
    }

    pub fn make_token(&self, ty: TokenType) -> Token<'_> {
       Token::new(ty, self.start.clone(), (self.current.position() - self.start.position()) as usize, self.line) 
    }

    pub fn error_token(&self, message: &'static str) -> Token<'_> {
       Token::new(TokenType::Error, self.start.clone(), message.len(), self.line) 
    }
    
    fn is_at_end(&self) -> bool {
        self.current.position() as usize == self.current.get_ref().len()
    }
    
    fn skip_whitespace(&mut self) {
        while self.current.get_ref()[self.current.position() as usize].is_ascii_whitespace() {
            self.advance();
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.current.get_ref()[self.current.position() as usize] as char
        }
    }

    fn identifier(&mut self) {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        if let Some(token_type) = self.keywords.get(text) {
            self.add_token(token_type.clone());
        } else {
            self.add_token(TokenType::Identifier);
        }
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        self.add_token_lit(
            TokenType::Number,
            Literal::Double(self.source[self.start..self.current].parse().unwrap()),
        );
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error_token("Unterminated string");
        } else {
            self.advance();
            let value = self.source[self.start + 1..self.current - 1].to_owned();
            self.add_token_lit(TokenType::String, Literal::Str(value));
        }
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.as_bytes()[self.current + 1] as char
        }
    }

}

#[derive(Debug, Clone)]
pub struct Token<'a> {
    ty:  TokenType,
    start: Cursor<&'a [u8]>,
    length: usize,
    line: usize
}

impl<'a> Token<'a> {
    pub fn ty(&self) -> TokenType {
        self.ty    
    }
    
    pub fn start(&self) -> Cursor<&[u8]> {
        self.start.clone()
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn ident(&self) -> &[u8] {
        &self.start.get_ref()[..self.length]
    }
    
    fn new(ty: TokenType, start: Cursor<&'a [u8]>, length: usize, line: usize) -> Self {
        Self { ty, start, length, line }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Error,
    Eof,
}



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
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            ';' => self.make_token(TokenType::Semicolon),
            '*' => self.make_token(TokenType::Star),
            '!' => {
                if self.matches('=') {
                    self.make_token(TokenType::BangEqual)
                } else {
                    self.make_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.matches('=') {
                    self.make_token(TokenType::EqualEqual)
                } else {
                    self.make_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.matches('=') {
                    self.make_token(TokenType::LessEqual)
                } else {
                    self.make_token(TokenType::Less)
                }
            }
            '>' => {
                if self.matches('=') {
                    self.make_token(TokenType::GreaterEqual)
                } else {
                    self.make_token(TokenType::Greater)
                }
            }
            '"' => self.string(),
            c if self.is_digit(c) => self.number(),
            c if self.is_alpha(c) => self.identifier(),
            _ => self.error_token("Unexpected character")
        }

    }

    fn is_digit(&self, c: char) -> bool {
        ('0'..='9').contains(&c)
    }


    fn is_alpha(&self, c: char) -> bool {
        ('a'..='z').contains(&c) || ('A'..='Z').contains(&c) || c == '_'
    }

    fn is_alphanumeric(&self, c: char) -> bool {
        self.is_digit(c) || self.is_alpha(c)
    }

    fn advance(&mut self) -> char {
        let c = self.current.get_ref()[self.current.position() as usize];
        self.current.set_position(self.current.position() - 1);
        c as char
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.peek() != expected {
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
        loop {
            match self.peek() {
                ' ' | '\r' | '\t' => { self.advance(); },
                '\n' => { self.line += 1; self.advance(); }
                '/' => {
                if self.peek_next() == '/' {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.make_token(TokenType::Slash);
                }
            }
                _ => break
            }
       }
    }

    fn peek(&self) -> char {
        self.current.get_ref()[self.current.position() as usize] as char
    }

    fn identifier(&mut self) -> Token {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }

        let ty = self.identifier_type();

        self.make_token(ty)
    }

    fn identifier_type(&self) -> TokenType {
        match self.start.get_ref()[self.start.position() as usize] as char {
            'a' => self.check_keyword(1, "nd", TokenType::And),
            'c' => self.check_keyword(1, "lass", TokenType::Class),
            'e' => self.check_keyword(1, "lse", TokenType::Else),
            'i' => self.check_keyword(1, "f", TokenType::If),
            'n' => self.check_keyword(1, "il", TokenType::Nil),
            'o' => self.check_keyword(1, "r", TokenType::Or),
            'p' => self.check_keyword(1, "print", TokenType::Print),
            'r' => self.check_keyword(1, "eturn", TokenType::Return),
            's' => self.check_keyword(1, "uper", TokenType::Super),
            'v' => self.check_keyword(1, "ar", TokenType::Var),
            'w' => self.check_keyword(1, "hile", TokenType::While),
            'f' if self.current.position() - self.start.position() > 1 =>
                match self.start.get_ref()[self.start.position() as usize] as char {
                     'a' => self.check_keyword(2, "lse", TokenType::False),
                     'o' => self.check_keyword(2, "r", TokenType::For),
                     'u' => self.check_keyword(2, "n", TokenType::Fun),
                     _ => TokenType::Identifier
                }
            't' if self.current.position() - self.start.position() > 1 =>
                match self.start.get_ref()[self.start.position() as usize] as char {
                     'h' => self.check_keyword(2, "is", TokenType::False),
                     'r' => self.check_keyword(2, "ue", TokenType::For),
                     _ => TokenType::Identifier
                }
            _ => TokenType::Identifier
        }
    }

    fn check_keyword(&self, start: usize, rest: &str, ty: TokenType) -> TokenType {
        if self.current.position() - self.start.position() == rest.len() as u64 && &self.start.get_ref()[start..start + rest.len()] == rest.as_bytes() {
            ty
        } else {
            TokenType::Identifier
        }
    }

    fn number(&mut self) -> Token {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        self.make_token(TokenType::Number)
    }

    fn string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error_token("Unterminated string")
        } else {
            self.advance();
            self.make_token(TokenType::String) 
        }
    }

    fn peek_next(&mut self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.current.get_ref()[self.current.position() as usize] as char
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



use anyhow::bail;

use crate::{
    chunk::{Chunk, OpCode},
    scanner::{Scanner, Token, TokenType},
};

pub struct Compiler<'a> {
    current: Option<Token<'a>>,
    previous: Option<Token<'a>>,
    scanner: Scanner<'a>,
    had_error: bool,
    panic_mode: bool,
    compiling_chunk: Option<Chunk>,
}

impl<'a> Compiler<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Compiler {
            current: None,
            previous: None,
            scanner: Scanner::new(source),
            had_error: false,
            panic_mode: false,
            compiling_chunk: None,
        }
    }

    pub fn compile(&mut self) -> anyhow::Result<Chunk> {
        self.compiling_chunk = Some(Chunk::new());
        self.advance()?;

        // parser.expression();
        // let mut line = 0;
        // loop {
        //     let token = scanner.scan_token();

        //     if token.line() != line {
        //         print!("{:4} ", line);
        //     } else {
        //         print!("   | ");
        //     }

        //     println!("{:?} {:?}", token.ty(), token.ident());

        //     if token.ty() == TokenType::Eof {
        //         break;
        //     }
        // }

        if self.had_error {
            bail!("Parser had error");
        }

        Ok(self.compiling_chunk.take().unwrap())
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        self.compiling_chunk.as_mut().unwrap()
    }

    fn emit_byte(&mut self, byte: impl Into<u8>) {
        let line = self.previous().line();
        self.current_chunk().write(byte, line);
    }
    pub fn current(&self) -> &Token {
        self.current.as_ref().unwrap()
    }

    pub fn previous(&self) -> &Token {
        self.previous.as_ref().unwrap()
    }

    pub fn advance(&mut self) -> anyhow::Result<()> {
        self.previous = self.current.take();
        loop {
            let token = self.scanner.scan_token();
            self.current = Some(token);
            let ty = self.current().ty();
            if ty != TokenType::Error {
                break;
            }
            self.error_at_current(&String::from_utf8_lossy(
                self.current.clone().unwrap().ident(),
            ));
        }

        Ok(())
    }

    fn error_at_current(&mut self, message: &str) {
        self.error_at(self.current.clone().unwrap(), message);
    }

    fn error(&mut self, message: &str) {
        self.error_at(self.previous.clone().unwrap(), message);
    }

    fn error_at(&mut self, token: Token, message: &str) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;

        eprint!("[line {}] Error", token.line());

        if token.ty() == TokenType::Eof {
            eprint!(" at end");
        } else if token.ty() == TokenType::Error {
        } else {
            eprint!(" at {:?}", token.ident());
        }

        eprintln!(": {}", message);

        self.had_error = true;
    }

    pub fn consume(&mut self, ty: TokenType, message: &str) -> anyhow::Result<()> {
        if self.current.clone().unwrap().ty() == ty {
            self.advance()?;
        } else {
            self.error_at_current(message);
        }

        Ok(())
    }
    pub fn expression() {}

    fn end_compiler(&mut self) {
        self.emit_return();
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::Return);
    }
}

use crate::scanner::{Scanner, TokenType};

pub fn compile(source: &[u8]) {
    let mut scanner = Scanner::new(source);
    let mut line = 0;
    loop {
        let token = scanner.scan_token();

        if token.line() != line {
            print!("{:4} ", line);
        } else {
            print!("   | ");
        }

        println!("{:?} {:?}", token.ty(), token.ident());

        if token.ty() == TokenType::Eof {
            break;
        }
    }
}
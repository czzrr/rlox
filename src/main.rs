use std::{io::Write, path::Path};

use rlox::{error_handler::ErrorHandler, interpreter:: Interpreter, parser::Parser, scanner::Scanner};

const USAGE_ERROR: i32 = 64;
const DATA_ERROR: i32 = 65;
const SOFTWARE_ERROR: i32 = 70;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    match args.len() {
        1 => run_prompt().unwrap(),
        2 => run_file(&args[1]).unwrap(),
        _ => {
            println!("Usage: rlox [script]");
            std::process::exit(USAGE_ERROR)
        }
    }
}

fn run_file(path: impl AsRef<Path>) -> std::io::Result<()> {
    let script = std::fs::read_to_string(path)?;

    let mut interpreter = Interpreter::new();
    let mut error_handler = ErrorHandler::new();

    run(script, &mut interpreter, &mut error_handler);

    if error_handler.had_error() {
        std::process::exit(DATA_ERROR);
    }
    if error_handler.had_runtime_error() {
        std::process::exit(SOFTWARE_ERROR);
    }

    Ok(())
}

fn run_prompt() -> std::io::Result<()> {
    let mut interpreter = Interpreter::new();
    let mut error_handler = ErrorHandler::new();
    
    loop {
        print!("> ");
        std::io::stdout().flush()?;
        let mut line = String::new();
        std::io::stdin().read_line(&mut line)?;
        if line.is_empty() {
            break;
        }
        run(line, &mut interpreter, &mut error_handler);
    }

    Ok(())
}

fn run(source: String, interpreter: &mut Interpreter, error_handler: &mut ErrorHandler) {
    let mut scanner = Scanner::new(source, error_handler);
    let tokens = scanner.scan_tokens().to_owned();
    let mut parser = Parser::new(tokens.to_owned());
    let statements = parser.parse(error_handler);

    if let Some(statements) = statements {
        interpreter.interpret(statements, error_handler);
    }
}

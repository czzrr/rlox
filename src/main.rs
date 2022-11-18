use std::path::Path;

use rlox::{scanner::Scanner, error_handler::ErrorHandler};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]).unwrap(),
        _ => {
            println!("Usage: rlox [script]");
            std::process::exit(64)
        }
    }
}

fn run_file(path: impl AsRef<Path>) -> std::io::Result<()> {
    let script = std::fs::read_to_string(path)?;
    if run(script).is_err() {
        std::process::exit(65)
    }
    Ok(())
}

fn run_prompt() {
    loop {
        print!("> ");
        let mut line = String::new();
        std::io::stdin().read_line(&mut line);
        if line.is_empty() {
            break;
        }
        run(line);
    }
}

fn run(source: String) -> Result<(), ()> {
    let mut error_handler = ErrorHandler::new();
    let mut scanner = Scanner::new(source, &mut error_handler);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{}", token);
    }
    
    Ok(())
}

fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, location: &str, message: &str) {
    println!("[line {line}] Error {location}: {message}");
    // hadError = true; ?
}
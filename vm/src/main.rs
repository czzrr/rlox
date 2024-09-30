use std::{env::args, io::stdin, path::Path};

use vm::{
    chunk::{Chunk, OpCode}, compiler::compile, value::Value, vm::Vm
};

fn main() {
    let args: Vec<_> = args().collect();
    match args.len() {
        1 => repl(),
        2 => run_file(Path::new(&args[1])),
        _ => {
            eprintln!("Usage: clox [path]");
            std::process::exit(64);
        }
    }
    // let mut chunk = Chunk::new();

    // let constant = chunk.add_constant(Value::Double(1.2));
    // chunk.write(OpCode::Constant, 123);
    // chunk.write(constant as u8, 123);

    // let constant = chunk.add_constant(Value::Double(3.4));
    // chunk.write(OpCode::Constant, 123);
    // chunk.write(constant as u8, 123);

    // chunk.write(OpCode::Add, 123);

    // let constant = chunk.add_constant(Value::Double(5.6));
    // chunk.write(OpCode::Constant, 123);
    // chunk.write(constant as u8, 123);

    // chunk.write(OpCode::Divide, 123);

    // chunk.write(OpCode::Negate, 123);
    // chunk.write(OpCode::Return, 123);

    // let mut vm = Vm::new();
    // vm.interpret(&chunk).unwrap();
}

fn repl() {
    loop {
        let mut line = String::new();
        if stdin().read_line(&mut line).unwrap() == 0 {
            break;
        }
        interpret(line.as_bytes()); 
    }
}

fn interpret(source: &[u8]) {
    compile(source);
}

fn run_file(path: impl AsRef<Path>) {
    let source = std::fs::read_to_string(path).unwrap();
    interpret(source.as_bytes()); 
}
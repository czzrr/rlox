use std::{env::args, io::stdin, path::Path};

use vm::{compiler::Compiler, vm::Vm};

fn main() -> anyhow::Result<()> {
    let args: Vec<_> = args().collect();
    match args.len() {
        1 => repl()?,
        2 => run_file(Path::new(&args[1])).map(|_| ())?,
        _ => {
            eprintln!("Usage: clox [path]");
            std::process::exit(64);
        }
    }

    Ok(())

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

fn repl() -> anyhow::Result<()> {
    loop {
        let mut line = String::new();
        if stdin().read_line(&mut line).unwrap() == 0 {
            break Ok(());
        }
        interpret(line.as_bytes())?;
    }
}

struct InterpretResult;

fn interpret(source: &[u8]) -> anyhow::Result<InterpretResult> {
    let mut compiler = Compiler::new(source);
    let chunk = compiler.compile()?;
    let mut vm = Vm::new();
    vm.interpret(&chunk)?;

    Ok(InterpretResult)
}

fn run_file(path: impl AsRef<Path>) -> anyhow::Result<InterpretResult> {
    let source = std::fs::read_to_string(path).unwrap();
    interpret(source.as_bytes())
}

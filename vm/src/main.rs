use vm::{
    chunk::{Chunk, OpCode},
    value::Value,
    vm::Vm,
};

fn main() {
    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(Value::Double(1.2));
    chunk.write(OpCode::Constant, 123);
    chunk.write(constant as u8, 123);

    let constant = chunk.add_constant(Value::Double(3.4));
    chunk.write(OpCode::Constant, 123);
    chunk.write(constant as u8, 123);

    chunk.write(OpCode::Add, 123);

    let constant = chunk.add_constant(Value::Double(5.6));
    chunk.write(OpCode::Constant, 123);
    chunk.write(constant as u8, 123);

    chunk.write(OpCode::Divide, 123);

    chunk.write(OpCode::Negate, 123);
    chunk.write(OpCode::Return, 123);

    let mut vm = Vm::new();
    vm.interpret(&chunk).unwrap();
}

use vm::{
    chunk::{Chunk, OpCode},
    debug::Disassembler,
    value::Value,
};

fn main() {
    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(Value::Double(1.2));
    chunk.write(OpCode::Constant, 123);
    chunk.write(constant as u8, 123);
    chunk.write(OpCode::Return, 123);

    Disassembler::disassemble(&chunk, "test chunk");
}

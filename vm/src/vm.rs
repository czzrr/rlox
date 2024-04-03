use std::collections::VecDeque;

use crate::{
    chunk::{Chunk, OpCode},
    debug::Disassembler,
    value::Value,
};

pub struct Vm<'a> {
    chunk: Option<&'a Chunk>,
    index: usize,
    stack: VecDeque<Value>,
}

#[derive(Debug, Clone, Copy)]
pub enum InterpretError {
    Compile,
    Runtime,
}

impl<'a> Vm<'a> {
    pub fn new() -> Self {
        Vm {
            chunk: None,
            index: 0,
            stack: VecDeque::new(),
        }
    }
    pub fn interpret(&'a mut self, chunk: &'a Chunk) -> Result<(), InterpretError> {
        self.chunk = Some(chunk);
        self.index = 0;
        self.run()
    }

    pub fn run(&mut self) -> Result<(), InterpretError> {
        loop {
            for value in &self.stack {
                print!("[ {} ]", value);
            }
            Disassembler::disassemble_instruction(self.chunk.expect("chunk"), self.index);
            let instruction = self.read_byte();
            match instruction.try_into().expect("valid opcode") {
                OpCode::Constant => {
                    let constant = *self.read_constant();
                    self.stack.push_back(constant);
                }
                OpCode::Negate => {
                    let Value::Double(n) = self.stack.pop_back().expect("value");
                    self.stack.push_back(Value::Double(-n));
                }
                OpCode::Add => self.binary_op(|a, b| a + b),
                OpCode::Subtract => self.binary_op(|a, b| a - b),
                OpCode::Multiply => self.binary_op(|a, b| a * b),
                OpCode::Divide => self.binary_op(|a, b| a / b),
                OpCode::Return => {
                    println!("{}", self.stack.pop_back().expect("non-empty stack"));
                    break Ok(());
                }
            }
        }
    }

    fn binary_op(&mut self, op: impl Fn(f64, f64) -> f64) {
        let Value::Double(b) = self.stack.pop_back().expect("value");
        let Value::Double(a) = self.stack.pop_back().expect("value");
        self.stack.push_back(Value::Double(op(a, b)));
    }
    pub fn read_byte(&mut self) -> u8 {
        let instruction = self.chunk.expect("chunk").code[self.index];
        self.index += 1;
        instruction
    }

    pub fn read_constant(&mut self) -> &Value {
        let value = &self.chunk.expect("chunk").constants[self.read_byte() as usize];
        value
    }
}

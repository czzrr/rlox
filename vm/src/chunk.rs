use derive_try_from_primitive::TryFromPrimitive;

use crate::value::Value;
#[derive(TryFromPrimitive, Debug, Clone, Copy)]
#[repr(u8)]
pub enum OpCode {
    Constant,
    Return,
}

impl Into<u8> for OpCode {
    fn into(self) -> u8 {
        self as u8
    }
}

pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
    pub lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn write<B>(&mut self, byte: B, line: usize)
    where
        B: Into<u8>,
    {
        self.code.push(byte.into());
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}

use crate::chunk::{Chunk, OpCode};

pub struct Disassembler;

impl Disassembler {
    pub fn disassemble(chunk: &Chunk, name: &str) {
        println!("== {} ==", name);

        let mut offset = 0;
        while offset < chunk.code.len() {
            offset = Self::disassemble_instruction(chunk, offset);
        }
    }

    pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
        print!("{:04} ", offset);
        if offset != 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{:4} ", chunk.lines[offset]);
        }
        let instruction = chunk.code[offset];
        if let Ok(opcode) = instruction.try_into() {
            match opcode {
                OpCode::Constant => Self::constant_instruction("CONSTANT", chunk, offset),
                OpCode::Add => Self::simple_instruction("ADD", offset),
                OpCode::Subtract => Self::simple_instruction("SUBTRACT", offset),
                OpCode::Multiply => Self::simple_instruction("MULTIPLY", offset),
                OpCode::Divide => Self::simple_instruction("DIVIDE", offset),
                OpCode::Negate => Self::simple_instruction("NEGATE", offset),
                OpCode::Return => Self::simple_instruction("RETURN", offset),
            }
        } else {
            println!("Unknown opcode {}", instruction);
            offset + 1
        }
    }

    pub fn simple_instruction(name: &str, offset: usize) -> usize {
        println!("{}", name);
        offset + 1
    }

    pub fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
        let constant = chunk.code[offset + 1];
        println!(
            "{:16} {:4} '{}'",
            name, constant as u8, chunk.constants[constant as usize]
        );
        offset + 2
    }
}

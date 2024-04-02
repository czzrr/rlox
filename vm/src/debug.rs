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
        if let Ok(opcode) = OpCode::try_from(instruction) {
            match opcode {
                OpCode::Return => Self::simple_instruction("RETURN", offset),
                OpCode::Constant => Self::constant_instruction("CONSTANT", chunk, offset),
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

use crate::chunk::{Chunk, OpCode};
use crate::value::{Value, print_value};
pub fn disassemble_chunk(chunk: Chunk, name: String) -> () {
    println!("== {} ==\n", name);
    let list = chunk.code.iter();
    let mut offset = 0;

    for (idx, opcode) in list.enumerate() {
        disassemble_instruction(opcode, idx);
    }
}

fn disassemble_instruction(instruction: &OpCode, offset: usize) -> () {
    match instruction {
        OpCode::Return => simple_instruction("OP_RETURN".to_string(), offset),
        OpCode::Constant(constant) => constant_instruction("OP_CONSTANT".to_string(), *constant, offset),
        _ => {
            println!("Unknown opcode {:?}\n", instruction);
        }
    }
}

fn simple_instruction(name: String, offset: usize) -> () {
    println!("{:04} {}", offset, name);
}

fn constant_instruction(name: String, constant: Value, offset: usize) -> () {
    println!("{:04} {} {:16}", offset, constant, name);
} 

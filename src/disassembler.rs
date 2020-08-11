use crate::chunk::{Chunk, OpCode};

pub fn disassemble_chunk(chunk_arg: Chunk, name: String) -> () {
    println!("== {} ==\n", name);
    let list = chunk_arg.code.iter();
    let mut offset = 0;

    for opcode in list {
        offset = disassemble_instruction(opcode, offset);
    }
}

fn disassemble_instruction(instruction: &OpCode, offset: i32) -> i32 {
    match instruction {
        OpCode::Return => simple_instruction("OP_RETURN".to_string(), offset),
        _ => {
            println!("Unknown opcode {:?}\n", instruction);
            offset + 1
        }
    }
}

fn simple_instruction(name: String, offset: i32) -> i32 {
    print!("{:04} ", offset);
    println!("{}", name);
    offset + 1
}

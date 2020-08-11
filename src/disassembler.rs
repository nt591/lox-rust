use crate::chunk::{Chunk, OpCode};
use crate::value::{Value};
pub fn disassemble_chunk(chunk: Chunk, name: String) -> () {
    println!("== {} ==\n", name);
    let list = chunk.code.iter();

    for (idx, _codeline) in list.enumerate() {
        disassemble_instruction(&chunk, idx);
    }
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> () {
    let code = &chunk.code[offset];
    print!("{:04} ", offset);
    let instruction = &code.code;
    let line = code.line;
    if offset > 0 && line == chunk.code[offset-1].line {
        print!("   | ");
    } else {
        print!("{:04} ", line);
    } 
    match instruction {
        OpCode::Return => simple_instruction("OP_RETURN".to_string()),
        OpCode::Constant(constant) => constant_instruction("OP_CONSTANT".to_string(), *constant),
        _ => {
            println!("Unknown opcode {:?}\n", instruction);
        }
    }
}

fn simple_instruction(name: String) -> () {
    println!("{}", name);
}

fn constant_instruction(name: String, constant: Value) -> () {
    println!("{} {:16}",name, constant);
} 

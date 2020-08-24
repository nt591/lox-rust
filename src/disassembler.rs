use crate::chunk::{Chunk, OpCode};
use crate::value::{Value};

#[allow(dead_code)]
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
        OpCode::Return => simple_instruction("OP_RETURN"),
        OpCode::Constant(constant) => constant_instruction("OP_CONSTANT", constant),
        OpCode::Negate => simple_instruction("OP_NEGATE"),
        OpCode::Add => simple_instruction("OP_ADD"),
        OpCode::Subtract => simple_instruction("OP_SUBTRACT"),
        OpCode::Multiply => simple_instruction("OP_MULTIPLY"),
        OpCode::Divide => simple_instruction("OP_DIVIDE"),
        OpCode::Nil => simple_instruction("OP_NIL"),
        OpCode::True => simple_instruction("OP_TRUE"),
        OpCode::False => simple_instruction("OP_FALSE"),
        OpCode::Not => simple_instruction("OP_NOT"),
        OpCode::Equal => simple_instruction("OP_EQUAL"),
        OpCode::Greater => simple_instruction("OP_GREATER"),
        OpCode::Less => simple_instruction("OP_LESS"),
    }
}

fn simple_instruction(name: &str) -> () {
    println!("{}", name);
}

fn constant_instruction(name: &str, constant: &Value) -> () {
    println!("{} {}",name, constant);
} 

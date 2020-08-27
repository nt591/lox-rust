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
        OpCode::DefineGlobal(val) => global_instruction("OP_DEFINE_GLOBAL", val),
        OpCode::GetGlobal(val) => global_instruction("OP_GET_GLOBAL", val),
        OpCode::SetGlobal(val) => global_instruction("OP_SET_GLOBAL", val),
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
        OpCode::Print => simple_instruction("OP_PRINT"),
        OpCode::Pop => simple_instruction("OP_POP"),
    }
}

fn simple_instruction(name: &str) -> () {
    println!("{}", name);
}

fn global_instruction(name: &str, constant: &String) -> () {
    println!("{} {}",name, constant);
}

fn constant_instruction(name: &str, constant: &Value) -> () {
    println!("{} {}",name, constant);
} 

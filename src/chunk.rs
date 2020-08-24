use std::fmt;
use crate::value::Value;

#[derive(Clone, Copy)]
pub enum OpCode {
    Constant(Value),
    True,
    False,
    Nil,
    Add,
    Subtract,
    Multiply,
    Divide,
    Return,
    Not,
    Negate,
    Equal,
    Greater,
    Less,
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpCode::Constant(val) => write!(f, "{}", val),
            OpCode::True => write!(f, "OpCode::True"),
            OpCode::False => write!(f, "OpCode::False"),
            OpCode::Nil => write!(f, "OpCode::Nil"),
            OpCode::Add => write!(f, "OpCode::Add"),
            OpCode::Subtract => write!(f, "OpCode::Subtract"),
            OpCode::Multiply => write!(f, "OpCode::Multiply"),
            OpCode::Divide => write!(f, "OpCode::Divide"),
            OpCode::Return => write!(f, "OpCode::Return"),
            OpCode::Negate => write!(f, "OpCode::Negate"),
            OpCode::Not => write!(f, "OpCode::Not"),
            OpCode::Equal => write!(f, "OpCode::Equal"),
            OpCode::Less => write!(f, "OpCode::Less"),
            OpCode::Greater => write!(f, "OpCode::Greater"),
        }
    }
}

#[derive(Clone)]
pub struct CodeLine {
    pub code: OpCode,
    pub line: i32,
}

#[derive(Clone)]
pub struct Chunk {
    pub code: Vec<CodeLine>,
    pub constants: Vec<Value>,
}

impl Chunk {
    pub fn new_chunk() -> Chunk {
        Chunk {
            code: Vec::new(), 
            constants: Vec::new(), 
        }
    }
    
    pub fn write(&mut self, code: OpCode, line: i32) -> () {
        self.code.push(CodeLine {code, line})
    }
}

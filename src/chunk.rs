use std::fmt;
use crate::value::Value;

#[derive(Clone)]
pub enum OpCode {
    Constant(Value),
    DefineGlobal(String),
    GetGlobal(String),
    SetGlobal(String),
    SetLocal(usize),
    GetLocal(usize),
    JumpIfFalse(usize),
    Jump(usize),
    Loop(usize),
    True,
    False,
    Pop,
    Nil,
    Add,
    Subtract,
    Multiply,
    Divide,
    Return,
    Not,
    Negate,
    Print,
    Equal,
    Greater,
    Less,
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpCode::Constant(val) => write!(f, "Constant {}", val),
            OpCode::DefineGlobal(val) => write!(f, "DefineGlobal {}", val),
            OpCode::GetGlobal(val) => write!(f, "GetGlobal {}", val),
            OpCode::SetGlobal(val) => write!(f, "SetGlobal {}", val),
            OpCode::GetLocal(val) => write!(f, "GetLocal {}", val),
            OpCode::SetLocal(val) => write!(f, "SetLocal {}", val),
            OpCode::JumpIfFalse(val) => write!(f, "JumpIfFalse: {}", val),
            OpCode::Jump(val) => write!(f, "Jump: {}", val),
            OpCode::Loop(val) => write!(f, "Loop: {}", val),
            OpCode::True => write!(f, "OpCode::True"),
            OpCode::False => write!(f, "OpCode::False"),
            OpCode::Pop => write!(f, "OpCode::Pop"),
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
            OpCode::Print => write!(f, "Print"),
        }
    }
}

#[derive(Clone)]
pub struct CodeLine {
    pub code: OpCode,
    pub line: i32,
}

impl fmt::Display for CodeLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ Code: {}, Line: {} }}", self.code, self.line)
    }
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

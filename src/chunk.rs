use crate::value::Value;

#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    Constant(Value),
    Add,
    Subtract,
    Multiply,
    Divide,
    Return,
    Negate,
}

#[derive(Clone, Debug)]
pub struct CodeLine {
    pub code: OpCode,
    pub line: i32,
}

#[derive(Clone, Debug)]
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

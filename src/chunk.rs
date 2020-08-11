use crate::value::{Value, ValueArray};

#[derive(Debug)]
pub enum OpCode {
    Constant(Value),
    Return,
}

pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: ValueArray,
}

impl Chunk {
    pub fn new_chunk() -> Chunk {
        Chunk {
            code: Vec::new(), 
            constants: ValueArray::new(), 
        }
    }
    
    pub fn write_chunk(&mut self, code: OpCode) -> () {
        self.code.push(code)
    }
}

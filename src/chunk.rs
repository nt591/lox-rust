#[derive(Debug)]
pub enum OpCode {
    Return,
}

pub struct Chunk {
    pub code: Vec<OpCode>,
}

impl Chunk {
    pub fn new_chunk() -> Chunk {
        Chunk {code: Vec::new()}
    }
    pub fn write_chunk(&mut self, code: OpCode) -> () {
        self.code.push(code)
    }
}

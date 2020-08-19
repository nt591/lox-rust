use crate::chunk::{Chunk, OpCode};
use crate::value::Value;
use crate::compiler::Compiler;

pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
}

pub enum InterpretResult {
    Ok, // Ok is reserved
    CompileError,
    RuntimeError,
}

impl VM {
    pub fn new() -> VM {
        VM {
            chunk: Chunk::new_chunk(),
            ip: 0, 
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self, source: &String) -> InterpretResult {
        self.chunk = Chunk::new_chunk();
        self.ip = 0; // we need to reset pointer to start of char vector
        let mut compiler = Compiler::new(&mut self.chunk);

        if !compiler.compile(&source) {
            return InterpretResult::CompileError
        }

        self.run()
    }
    
    pub fn run(&mut self) -> InterpretResult {
        loop {
            let ip = self.ip;
            self.ip += 1;
            let instruction = self.chunk.code[ip].code;

            match instruction {
                OpCode::Return => {
                    match self.stack.pop() {
                        Some(val) => {
                            println!("{:?}", val);
                            break InterpretResult::Ok
                        },
                        None => {
                            println!("Error: Nothing to return");
                            break InterpretResult::CompileError
                        }
                    }
                },
                OpCode::Constant(val) => self.stack.push(val),
                OpCode::Negate => {
                    match self.stack.pop() {
                        Some(val) => self.stack.push(val * -1.0),
                        None => eprintln!("Error popping"),
                    }
                }
                
                OpCode::Add | 
                    OpCode::Subtract |
                    OpCode:: Multiply | 
                    OpCode:: Divide => self.binary_operation(&instruction)

            }
        }
    }

    fn binary_operation(&mut self, operator: &OpCode) -> () {
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        match operator {
            OpCode::Add => self.stack.push(a + b),
            OpCode::Subtract => self.stack.push(a - b),
            OpCode::Multiply => self.stack.push(a * b),
            OpCode::Divide => self.stack.push(a / b),
            _ => panic!("{:?} is not a binary operation", operator),
        }
    } 
}

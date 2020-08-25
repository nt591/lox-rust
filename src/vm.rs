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
        self.reset_stack(); // we need to reset pointer to start of char vector
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
            let instruction = self.chunk.code[ip].code.clone();
            match instruction {
                OpCode::Return => {
                    match self.stack.pop() {
                        Some(val) => {
                            println!("{}", val);
                            break InterpretResult::Ok
                        },
                        None => {
                            println!("Error: Nothing to return");
                            break InterpretResult::CompileError
                        }
                    }
                },
                OpCode::Constant(val) => self.stack.push(val),
                OpCode::Nil => self.stack.push(Value::nil_val()),
                OpCode::True => self.stack.push(Value::bool_val(true)),
                OpCode::False => self.stack.push(Value::bool_val(false)),
                OpCode::Negate => {
                    match &self.peek(0) {
                        val if !Value::is_number(&val) => {
                            self.runtime_error("Operand must be a number");
                            break InterpretResult::RuntimeError
                        },
                        _ => match self.stack.pop() {
                                Some(val) => self.stack.push(Value::number_val(Value::as_number(val) * -1.0)),
                                None => panic!("Error - no value to pop from stack"),
                            }
                    }
                },
                OpCode::Not => match self.stack.pop() {
                    None => {
                        self.runtime_error("No value to pop from stack");
                        break InterpretResult::RuntimeError
                    },
                    Some(val) => self.stack.push(Value::bool_val(Value::is_falsey(&val))),
                },
                OpCode::Equal => {
                    let b: Value = self.stack.pop().unwrap();
                    let a: Value = self.stack.pop().unwrap();
                    self.stack.push(Value::bool_val(Value::values_equal(a, b)));
                },
                
                // todo - consolidate with binary_operation
                OpCode::Greater | OpCode::Less => {
                    match self.binary_comparison(&instruction) {
                        Err(error) => break error,
                        _ => ()
                    }
                }

                OpCode::Add | 
                    OpCode::Subtract |
                    OpCode:: Multiply | 
                    OpCode:: Divide => match self.binary_operation(&instruction) {
                        Err(error) => break error,
                        _ => (),
                    }

            }
        }
    }

    fn peek(&self, distance: usize) -> &Value {
        &self.stack[self.stack.len() - 1 - distance]
    }

    fn binary_operation(&mut self, operator: &OpCode) -> Result<(), InterpretResult> {
        if !Value::is_number(&self.peek(0)) || !Value::is_number(&self.peek(0)) {
            self.runtime_error("Operands must be numbers");
            Err(InterpretResult::RuntimeError)
        } else {
            let b = Value::as_number(self.stack.pop().unwrap());
            let a = Value::as_number(self.stack.pop().unwrap());
            match operator {
                OpCode::Add => self.stack.push(Value::number_val(a + b)),
                OpCode::Subtract => self.stack.push(Value::number_val(a - b)),
                OpCode::Multiply => self.stack.push(Value::number_val(a * b)),
                OpCode::Divide => self.stack.push(Value::number_val(a / b)),
                _ => panic!("{} is not a binary operation", operator),
            }
            Ok(())
        }
    }

    fn binary_comparison(&mut self, operator: &OpCode) -> Result<(), InterpretResult> {
        if !Value::is_number(&self.peek(0)) || !Value::is_number(&self.peek(0)) {
            self.runtime_error("Operands must be numbers");
            Err(InterpretResult::RuntimeError)
        } else {
            let b = Value::as_number(self.stack.pop().unwrap());
            let a = Value::as_number(self.stack.pop().unwrap());
            match operator {
                OpCode::Greater => self.stack.push(Value::bool_val(a > b)),
                OpCode::Less => self.stack.push(Value::bool_val(a < b)),
                _ => panic!("{} is not a binary comparator", operator),
            }
            Ok(())
        }
    }

    fn reset_stack(&mut self) -> () {
        self.ip = 0;
        self.stack = Vec::new();
    }

    fn runtime_error(&mut self, msg: &str) -> () {
        let line = self.chunk.code[self.ip - 1].line;
        eprintln!("{}", msg);
        eprintln!("[line {}] in script", line);
        self.reset_stack()
    }
}

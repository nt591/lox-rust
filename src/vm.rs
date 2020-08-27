use std::collections::HashMap;

use crate::chunk::{Chunk, OpCode};
use crate::value::Value;
use crate::compiler::Compiler;

pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
    globals: HashMap<String, Value>,
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
            globals: HashMap::new(),
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
            // handle overflow
            if ip >= self.chunk.code.len() {
                return InterpretResult::Ok
            }

            let instruction = self.chunk.code[ip].code.clone();
            match instruction {
                OpCode::Return => (),
                /*
                 * OpCode::Return => {
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
                */
                OpCode::Print => println!("{}", self.stack.pop().unwrap()),
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
                
                OpCode::Pop => {
                    self.stack.pop();
                },
                OpCode::DefineGlobal(val) => {
                    self.globals.insert(val, self.peek(0).clone());
                    self.stack.pop();
                }
                OpCode::GetGlobal(val) => {
                    let value = self.globals.get(&val);
                    if let Some(v) = value {
                        self.stack.push(v.clone());
                    } else {
                        self.runtime_error(&format!("Undefined variable {}", val));
                        return InterpretResult::RuntimeError;
                    }
                }

                OpCode::SetGlobal(val) => {
                    // setting a variable that's been previously declared
                    // first make sure the variable exists
                    if self.globals.contains_key(&val) {
                        self.globals.insert(val, self.peek(0).clone());
                    } else {
                        self.runtime_error(&format!("Undefined variable {}", val));
                        return InterpretResult::RuntimeError;
                    }
                }
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
        // handle string concat separately
        if Value::is_string(&self.peek(0)) && Value::is_string(&self.peek(1)) {
            match operator {
                OpCode::Add => {
                    self.concatenate();
                    Ok(())
                },
                _ => {
                    self.runtime_error("Invalid operator for strings");
                    Err(InterpretResult::RuntimeError)
                }
            }
        } else if Value::is_number(&self.peek(0)) && Value::is_number(&self.peek(0)) {
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
        } else {
            self.runtime_error("Operands must be both numbers or strings");
            Err(InterpretResult::RuntimeError)
        }
    }

    fn concatenate(&mut self) -> () {
        let b = Value::as_string(self.stack.pop().unwrap());
        let mut a = Value::as_string(self.stack.pop().unwrap());
        a.push_str(&b);
        self.stack.push(Value::string_val(a));
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

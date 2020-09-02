use std::fmt;

use crate::chunk::Chunk;

#[derive(Clone)]
pub enum ValueType {
	Bool(bool),
	Nil,
	Number(f64),
    String(String),
    Function(FunctionDef),
}

#[derive(Clone)]
enum FunctionType {
    Function,
    Script,
}

#[derive(Clone)]
pub struct FunctionDef {
    arity: usize,
    name: String,
    pub chunk: Chunk,
    fn_type: FunctionType,
}

impl FunctionDef {
    pub fn new() -> FunctionDef {
        FunctionDef {
            arity: 0,
            name: String::from(""),
            chunk: Chunk::new_chunk(),
            fn_type: FunctionType::Script,
        }
    }
}

#[derive(Clone)]
pub struct Value {
	value_type: ValueType,
}

impl Value {
	pub fn bool_val(val: bool) -> Value {
		Value { 
			value_type: ValueType::Bool(val)
		}
	}

	pub fn number_val(val: f64) -> Value {
		Value {
			value_type: ValueType::Number(val)
		}
	}

    pub fn string_val(val: String) -> Value {
        Value {
            value_type: ValueType::String(val)
        }
    }

    pub fn nil_val() -> Value {
        Value {
            value_type: ValueType::Nil
        }
    }

    pub fn new_function() -> Value {
        Value {
            value_type: ValueType::Function(FunctionDef::new().clone())
        }
    }
    
	pub fn as_bool(val: Value) -> bool {
		match val.value_type {
			ValueType::Bool(val) => val,
			_ => panic!("Value::as_bool should never be called on a non-bool type")
		}
	}
		
	pub fn as_number(val: Value) -> f64 {
		match val.value_type {
			ValueType::Number(val) => val,
			_ => panic!("Value::as_number should never be called on a non-f64 type")
		}
	}

    pub fn as_string(val: Value) -> String {
        match val.value_type {
            ValueType::String(string) => string,
            _ => panic!("Value::as_string should never be called on non-string"),
        }
    }

    pub fn as_function(val: Value) -> FunctionDef {
        match val.value_type {
            ValueType::Function(f) => f,
            _ => panic!("Value::as_function was called on a value not of ValueType::Function"),
        }    
    }

    pub fn is_number(val: &Value) -> bool {
        match val.value_type {
            ValueType::Number(_) => true,
            _ => false,
        }
    }

    pub fn is_bool(val: &Value) -> bool {
        match val.value_type {
            ValueType::Bool(_) => true,
            _ => false,
        }
    }

    pub fn is_string(val: &Value) -> bool {
        match val.value_type {
            ValueType::String(_) => true,
            _ => false,
        }
    }

    pub fn is_function(val: &Value) -> bool {
        match val.value_type {
            ValueType::Function(_) => true,
            _ => false,
        }
    }

    pub fn is_falsey(val: &Value) -> bool {
        match val.value_type {
            ValueType::Nil => true,
            ValueType::Bool(bool_val) => !bool_val,
            ValueType::Number(_) => false,
            ValueType::String(_) => false,
            ValueType::Function(_) => false,
        }
    }

    pub fn values_equal(a: Value, b: Value) -> bool {
        match (a.value_type, b.value_type) {
            (ValueType::Bool(a_val), ValueType::Bool(b_val)) => a_val == b_val,
            (ValueType::Nil, ValueType::Nil) => true,
            (ValueType::Number(a_val), ValueType::Number(b_val)) => a_val == b_val,
            (ValueType::String(s), ValueType::String(a)) => a == s,
            _ => false,
        }
    }

}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.value_type.clone();
        match val {
            ValueType::Number(v) => write!(f, "{}", v),
            ValueType::Bool(v) => write!(f, "{}", v),
            ValueType::Nil => write!(f, "Nil"),
            ValueType::String(s) => write!(f, "\"{}\"", s),
            ValueType::Function(func) => match func.fn_type {
                FunctionType::Script => write!(f, "<script>"),
                _ => write!(f, "<fn {}>", func.name),
            }
        }
    }
}

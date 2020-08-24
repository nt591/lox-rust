use std::fmt;

#[derive(Copy, Clone)]
pub enum ValueType {
	Bool(bool),
	Nil,
	Number(f64),
}

#[derive(Copy, Clone)]
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

        pub fn nil_val() -> Value {
            Value {
                value_type: ValueType::Nil
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
			_ => panic!("Value::as_bool should never be called on a non-bool type")
		}
	}

        pub fn is_number(val: Value) -> bool {
            match val.value_type {
                ValueType::Number(_) => true,
                _ => false,
            }
        }

        pub fn is_bool(val: Value) -> bool {
            match val.value_type {
                ValueType::Bool(_) => true,
                _ => false,
            }
        }

        pub fn is_falsey(val: Value) -> bool {
            match val.value_type {
                ValueType::Nil => true,
                ValueType::Bool(bool_val) => !bool_val,
                ValueType::Number(_) => false,
            }
        }

        pub fn values_equal(a: Value, b: Value) -> bool {
            match (a.value_type, b.value_type) {
                (ValueType::Bool(a_val), ValueType::Bool(b_val)) => a_val == b_val,
                (ValueType::Nil, ValueType::Nil) => true,
                (ValueType::Number(a_val), ValueType::Number(b_val)) => a_val == b_val,
                _ => false,
            }
        }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.value_type {
            ValueType::Number(v) => write!(f, "{}", v),
            ValueType::Bool(v) => write!(f, "{}", v),
            ValueType::Nil => write!(f, "Nil"),
        }
    }
}

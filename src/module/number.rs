use std::error::Error;

use super::value::Value;

#[derive(Debug, Clone, Copy)]
pub enum NumberType {
    Int32,
    Int64,
    Float32,
    Float64,
}
impl NumberType {
    pub fn from_byte(byte: u8) -> Option<NumberType> {
        match byte {
            0x7F => Some(NumberType::Int32),
            0x7E => Some(NumberType::Int64),
            0x7D => Some(NumberType::Float32),
            0x7C => Some(NumberType::Float64),
            // _ => panic!("Invalid ValueType {:x}", byte),
            _ => {
                println!("Invalid ValueType {:x}", byte);
                Some(NumberType::Int32)
            }
        }
    }

    pub fn decode_type(byte: u8) -> Result<NumberType, Box<dyn Error>> {
        Ok(match NumberType::from_byte(byte).unwrap() {
            NumberType::Int32 => NumberType::Int32,
            NumberType::Int64 => NumberType::Int64,
            NumberType::Float32 => NumberType::Float32,
            NumberType::Float64 => NumberType::Float64,
        })
    }

    pub fn inspect(&self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Debug)]
pub struct Number {
    bits: u8,
    pub num_type: NumberType,
    pub value: Value,
}
impl Number {
    fn new(bits: u8, num_type: NumberType, value: Value) -> Number {
        Number {
            bits: bits,
            num_type: num_type,
            value: value,
        }
    }

    pub fn i32(value: Option<i32>) -> Number {
        let v = value.unwrap_or(0);
        Number::new(32, NumberType::Int32, Value::Int32(v))
    }

    pub fn i64(value: Option<i64>) -> Number {
        let v = value.unwrap_or(0);
        Number::new(64, NumberType::Int64, Value::Int64(v))
    }

    pub fn f32(value: Option<f32>) -> Number {
        let v = value.unwrap_or(0.0);
        Number::new(32, NumberType::Float32, Value::Float32(v))
    }

    pub fn f64(value: Option<f64>) -> Number {
        let v = value.unwrap_or(0.0);
        Number::new(64, NumberType::Float64, Value::Float64(v))
    }

    pub fn inspect(&self) -> String {
        format!("{:?}", self.num_type)
    }
}

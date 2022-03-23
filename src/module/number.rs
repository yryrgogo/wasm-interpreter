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
            _ => panic!("Invalid ValueType {:x}", byte),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Number {
    bits: u8,
    num_type: NumberType,
}
impl Number {
    fn new(bits: u8, num_type: NumberType) -> Number {
        Number {
            bits: bits,
            num_type: num_type,
        }
    }

    pub fn i32() -> Number {
        Number::new(32, NumberType::Int32)
    }

    pub fn i64() -> Number {
        Number::new(64, NumberType::Int64)
    }

    pub fn f32() -> Number {
        Number::new(32, NumberType::Float32)
    }

    pub fn f64() -> Number {
        Number::new(64, NumberType::Float64)
    }

    pub fn inspect(&self) -> String {
        format!("{:?}", self.num_type)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Integer(i32),
    Binary(String),
    Hexadecimal(String),
}

impl Number {
    pub fn to_i32(&self) -> Option<i32> {
        match self {
            Number::Integer(value) => Some(*value),
            Number::Binary(value) => {
                match i32::from_str_radix(value, 2) {
                    Ok(num) => Some(num),
                    Err(_) => None,
                }
            }
            Number::Hexadecimal(value) => {
                match i32::from_str_radix(value, 16) {
                    Ok(num) => Some(num),
                    Err(_) => None,
                }
            }
        }
    }
    pub fn to_u16(&self) -> Option<u16> {
        match self.to_i32() {
            Some(value) => {
                match u16::try_from(value) {
                    Ok(num) => Some(num),
                    Err(_) => None,
                }
            }
            None => None,
        }
    }
}
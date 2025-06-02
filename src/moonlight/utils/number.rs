
use crate::moonlight::utils::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Integer(i32),
    Binary(String),
    Hexadecimal(String),
}

impl Number {
    
    pub fn to_u16(&self) -> Option<u16> {
        match self {
            Number::Integer(v) => {
                match u16::try_from(*v) {
                    Ok(val) => Some(val),
                    Err(_) => None,
                }
            }
            Number::Binary(bits) => {
                match bits.from_bin_to_u16() {
                    Ok(val) => Some(val),
                    Err(_) => None,
                }
            }
            Number::Hexadecimal(hex) => {
                match hex.from_hex_to_u16() {
                    Ok(val) => Some(val),
                    Err(_) => None,
                }
            }
        }
    }
}
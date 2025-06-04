
use crate::moonlight::utils::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Integer(i32),
    Binary(String),
    Hexadecimal(String),
}

impl Number {
    
    pub fn to_u16(&self) -> Result<u16, String> {
        match self {
            Number::Integer(v) => {
                match u16::try_from(*v) {
                    Ok(val) => Ok(val),
                    Err(_) => Err("Value out of range for unsigned 16-bit integer.".to_string()),
                }
            }
            Number::Binary(bits) => {
                match bits.from_bin_to_u16() {
                    Ok(val) => Ok(val),
                    Err(e) => Err(e),
                }
            }
            Number::Hexadecimal(hex) => {
                match hex.from_hex_to_u16() {
                    Ok(val) => Ok(val),
                    Err(e) => Err(e),
                }
            }
        }
    }


}
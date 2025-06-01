#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Integer(i32),
    Binary(String),
    Hexadecimal(String),
}
#[derive(Debug, Clone)]
pub enum Number {
    Integer(i32),
    Binary(String),
    Hexadecimal(String),
}
#[derive(Debug, Clone)]
pub struct Position {
    pub file: u32,
    pub line: u32,
    pub column: Option<u32>,
}

impl Position {
    pub fn new(file: u32, line: u32, column: Option<u32>) -> Self {
        Position {
            file,
            line,
            column,
        }
    }
}
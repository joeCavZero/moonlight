use std::collections::HashMap;
use crate::moonlight::parser::Parseable;
use crate::moonlight::utils::*;
use crate::moonlight::scanner::*;

pub struct Moonlight {
    pub file_table: HashMap<u32, String>,
    pub file_counter: u32,
    pub file_dependencies: HashMap<u32, Vec<u32>>,
}

impl Moonlight {
    pub fn new() -> Self {
        Moonlight {
            file_table: HashMap::new(),
            file_counter: 0,
            file_dependencies: HashMap::new(),
        }
    }

    pub fn get_file_name(&self, file_id: u32) -> String {
        match self.file_table.get(&file_id) {
            Some(name) => name.clone(),
            None => "Unknown".to_string(),
        }
    }

    pub fn run(&mut self, file_path: &str) {
        let tokens: Vec<PositionedToken> = self.scan(file_path);
        for token in &tokens {
            println!("{:?}", token);
        }
        println!("========================");
        let ast = self.parse(&tokens);
        println!("{:#?}", ast);
    }
}

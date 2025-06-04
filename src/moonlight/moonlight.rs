use std::collections::HashMap;
use crate::moonlight::data_memory_loadable::*;
use crate::moonlight::parseable::*;
use crate::moonlight::utils::*;
use crate::moonlight::scanneable::*;
use crate::moonlight::symbol_table_loadable::*;

pub const DATA_MEMORY_SIZE: usize = 32768;

pub struct Moonlight {
    pub file_table: HashMap<u32, String>,
    pub symbol_table: HashMap<String, u16>,
    pub data_memory: [u8; DATA_MEMORY_SIZE],
}

impl Moonlight {
    pub fn new() -> Self {
        Moonlight {
            file_table: HashMap::new(),
            symbol_table: HashMap::new(),

            data_memory: [7; DATA_MEMORY_SIZE],
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
        let ast = self.parse(&tokens);
        self.load_symbol_table_from(&ast);
        self.load_data_memory_from(&ast);
        //self.setup_instruction_memory_from(&ast);

    }
}

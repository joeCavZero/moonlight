use std::collections::HashMap;
use crate::moonlight::debug::*;
use super::front::*;

pub struct Moonlight {
    file_table: HashMap<String, u32>,
    file_counter: u32,
}

impl Moonlight {
    pub fn new() -> Self {
        Moonlight {
            file_table: HashMap::new(),
            file_counter: 0,
        }
    }

    pub fn get_file_name(&self, file_id: u32) -> String {
        for (fname, fid) in &self.file_table {
            if *fid == file_id {
                return fname.clone();
            }
        }
        return "Unknown".to_string();
    }

    pub fn run(&mut self, file_path: &str) {
        let all: Vec<PositionedToken> = self.resolve_includes(file_path, self.file_counter);
        for token in all {
            println!("{:?}", token);
        }
        for (fname, fid) in &self.file_table {
            println!("File: {} - ID: {}", fname, fid);
        }
    }

    pub fn resolve_includes(&mut self, file_path: &str, file_id: u32) -> Vec<PositionedToken> {
        /*
            Aqui vamos fazer um scan do arquivo file_path,
            verificar se existe alguma diretiva de include,
            e se existir, vamos chamar recursivamente a função
            resolve_includes para cada arquivo incluído via include.
         */
        self.file_table.insert(file_path.to_string(), file_id);
        let mut tokens = match scan_tokens_from_file(file_path, file_id) {
            Ok(tokens) => tokens,
            Err(_) => {
                self.exit_with_error(&format!("Error scanning file {}", file_path).as_str());
                return Vec::new();
            }
        };

        let mut token_counter = 0;
        let mut token_quantity = tokens.len();
        while token_counter < token_quantity {
            let tk = match tokens.get(token_counter) {
                Some(tk) => tk,
                None => break
            };
            if let Token::Directive(directive) = tk.token.clone() {
                match directive {
                    Directive::Include => {
                        if let Some(next_token) = tokens.get(token_counter + 1) {
                            if let Token::StringLiteral(path) = next_token.token.clone() {
                                /*
                                    path already haven't the quotes
                                    now the idea is to add the file to the file table
                                    and then call resolve_includes recursively and
                                    remove the include directive and the path from the tokens
                                    and add the tokens from the included file to the current tokens
                                    ( i must detect import cycles via file_table )
                                */
                                
                                if self.file_table.contains_key(&path) {
                                    self.exit_with_error(format!("Import cycle detected for file {}", path).as_str());
                                }
                                
                                self.file_counter += 1;
                                let included_tokens = self.resolve_includes(&path, self.file_counter);
                                

                                // remove the include directive and the path from the tokens
                                tokens.remove(token_counter);
                                tokens.remove(token_counter);
                                
                                // add the included tokens to the current tokens
                                for inc_tkn in included_tokens {
                                    tokens.insert( token_counter, inc_tkn);
                                    token_counter += 1;
                                    token_quantity += 1;
                                }

                                continue;
                            }
                        }
                    }
                    _ => {}
                }
            }
            token_counter += 1;
        }
        
        return tokens;
    }
}

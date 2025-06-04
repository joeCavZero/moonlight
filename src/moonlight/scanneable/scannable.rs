use std::collections::HashMap;
use std::collections::HashSet;

use crate::moonlight::debugable::*;
use crate::moonlight::utils::*;
use crate::moonlight::*;

use super::scanner::*;

pub trait Scannable {
    fn scan(&mut self, file_path: &str) -> Vec<PositionedToken>;
    fn resolve_includes(&mut self, file_path: &str, file_id: u32) -> Vec<PositionedToken>;
    fn has_dependency_cycle(&self, start: u32, target: u32) -> bool;
}

impl Scannable for Moonlight {
    fn scan(&mut self, file_path: &str) -> Vec<PositionedToken> {
        let raw_tokens = self.resolve_includes(file_path, self.file_counter);
        raw_tokens
    }

    fn resolve_includes(&mut self, file_path: &str, file_id: u32) -> Vec<PositionedToken> {
        /*
           Aqui vamos fazer um scan do arquivo file_path,
           verificar se existe alguma diretiva de include,
           e se existir, vamos chamar recursivamente a função
           resolve_includes para cada arquivo incluído via include.
        */
        self.file_table.insert(file_id, file_path.to_string());

        let mut tokens = match scan_tokens_from_file(file_path, file_id) {
            Ok(tokens) => tokens,
            Err((e, p)) => {
                match p {
                    Some(position) => {
                        self.exit_with_positional_error(e.as_str(), position);
                    }
                    None => {
                        self.exit_with_error(e.as_str());
                    }
                }
                return Vec::new();
            }
        };

        let mut token_counter = 0;
        let mut token_quantity = tokens.len();
        while token_counter < token_quantity {
            let tk = match tokens.get(token_counter) {
                Some(tk) => tk,
                None => break,
            };
            if let Token::Directive(directive) = tk.token.clone() {
                match directive {
                    Directive::Include => {
                        if let Some(next_token) = tokens.get(token_counter + 1) {
                            if let Token::StringLiteral(path) = next_token.token.clone() {
                                // Encontrar o id do arquivo incluído, se já existir no file_table
                                let included_file_id = if let Some((&id, _)) =
                                    self.file_table.iter().find(|(_, v)| **v == path)
                                {
                                    id
                                } else {
                                    self.file_counter += 1;
                                    self.file_counter
                                };

                                /*
                                    This part checks for dependency cycles.
                                    If the current file_id is already in the dependency chain
                                    of the included_file_id, it means we have a cycle.
                                 */
                                if self.has_dependency_cycle(file_id, included_file_id) {
                                    let included_name = self.get_file_name(included_file_id);
                                    let current_name = self.get_file_name(file_id);

                                    self.exit_with_error(&format!(
                                        "Include cycle detected [{} -> {}]",
                                        current_name, included_name
                                    ));
                                    return Vec::new();
                                }

                                // Atualizar file_table se necessário
                                if !self.file_table.contains_key(&included_file_id) {
                                    self.file_table.insert(included_file_id, path.clone());
                                }

                                // Atualizar file_dependencies
                                self.file_dependencies
                                    .entry(file_id)
                                    .or_insert_with(Vec::new)
                                    .push(included_file_id);

                                // Recursivamente processar includes
                                let included_tokens =
                                    self.resolve_includes(&path, included_file_id);

                                // Remover a diretiva de include e o path dos tokens
                                tokens.remove(token_counter);
                                tokens.remove(token_counter);

                                // Inserir tokens incluídos
                                for inc_tkn in included_tokens {
                                    tokens.insert(token_counter, inc_tkn);
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

    fn has_dependency_cycle(&self, start: u32, target: u32) -> bool {
        fn visit(
            deps: &HashMap<u32, Vec<u32>>,
            current: u32,
            target: u32,
            visited: &mut HashSet<u32>,
        ) -> bool {
            if current == target {
                return true;
            }
            if !visited.insert(current) {
                return false;
            }
            if let Some(children) = deps.get(&current) {
                for &child in children {
                    if visit(deps, child, target, visited) {
                        return true;
                    }
                }
            }
            false
        }
        let mut visited = std::collections::HashSet::new();
        visit(&self.file_dependencies, target, start, &mut visited)
    }
}
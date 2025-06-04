use crate::moonlight::{debug::Debugable, parser::ast::{Ast, DataArg}, utils::{Directive, Token}, Moonlight};

pub trait SymbolTableLoader {
    fn load_symbol_table_from(&mut self, ast: &Ast);
}

impl SymbolTableLoader for Moonlight {
    fn load_symbol_table_from(&mut self, ast: &Ast) {
        let mut stack_counter: u16 = 0;
        for data_camp in ast.data_field.iter() {
            for label in data_camp.label_declarations.iter() {
                match label.token {
                    Token::LabelDeclaration(ref label_string) => {
                        self.symbol_table.insert(
                            label_string.clone(),
                            stack_counter,
                        );
                    }
                    _ => unreachable!(),
                }
                
            }

            match data_camp.directive.token {
                ///////////////////////////
                // CASE BE BYTE OR WORD
                ///////////////////////////
                
                Token::Directive(Directive::Byte) 
                | Token::Directive(Directive::Word) 
                => {
                    let bytes_to_step = match data_camp.directive.token {
                        Token::Directive(Directive::Byte) => 1,
                        Token::Directive(Directive::Word) => 2,
                        _ => unreachable!(),
                    };

                    match data_camp.arg {
                        DataArg::Values(ref values) => {
                            for _ in values.iter() {
                                match stack_counter.checked_add(bytes_to_step) {
                                    Some(v) => {
                                        stack_counter = v;
                                    }
                                    None => {
                                        self.exit_with_error("Stack overflow while loading symbol table.");
                                    }
                                }
                            }
                        }
                        _ => unreachable!(),
                    }

                    
                }

                //////////////////////
                // CASE BE SPACE
                //////////////////////
                Token::Directive(Directive::Space) => {
                    match data_camp.arg {
                        DataArg::Number(ref ptk) => {
                            match ptk.token {
                                Token::Number(ref number_token) => {
                                    match number_token.to_u16() {
                                        Ok(num) => {
                                            match stack_counter.checked_add(num) {
                                                Some(v) => {
                                                    stack_counter = v;
                                                }
                                                None => {
                                                    self.exit_with_error("Stack overflow while loading symbol table.");
                                                }
                                            }
                                        }
                                        Err(e) => self.exit_with_positional_error(e.as_str(), ptk.position),
                                    }
                                }
                                _ => unreachable!(),
                            } 
                        }
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }


            
        }
    }

}
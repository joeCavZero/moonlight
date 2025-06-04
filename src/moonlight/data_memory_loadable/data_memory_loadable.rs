use std::mem::transmute;

use crate::moonlight::debugable::*;
use crate::moonlight::parseable::*;
use crate::moonlight::utils::*;
use crate::moonlight::Moonlight;

pub trait DataMemoryLoadable {
    fn load_data_memory_from(&mut self, ast: &Ast);
}

impl DataMemoryLoadable for Moonlight {
    fn load_data_memory_from(&mut self, ast: &Ast) {
        let mut stack_counter: usize = 0;
        for data_camp in ast.data_field.iter() {

            match data_camp.directive.token {
                
                ////////////////////////
                // CASE BE BYTE
                ////////////////////////
                Token::Directive(Directive::Byte) => {
                    if let DataArg::Values(ref values) = data_camp.arg {
                        for vptk in values.iter() {
                            if let Token::Number(ref num_token) = vptk.token {
                                match num_token.to_u8() {
                                    Ok(n) => {
                                        match self.data_memory.get_mut(stack_counter) {
                                            Some(memory_cell) => {
                                                *memory_cell = n;
                                            }
                                            None => self.exit_with_positional_error("Stack overflow while loading data memory.", vptk.position),
                                        }
                                        stack_counter += 1;
                                    }
                                    Err(e) => self.exit_with_positional_error(e.as_str(), vptk.position),
                                }
                            } else {unreachable!();}
                        } 
                    } else {unreachable!();}
                }

                ////////////////////////
                // CASE BE WORD
                ////////////////////////
                Token::Directive(Directive::Word) => {
                    if let DataArg::Values(ref values) = data_camp.arg {
                        for vptk in values.iter() {
                            if let Token::Number(ref num_token) = vptk.token {
                                match num_token.to_i16() {
                                    Ok(n) => {
                                        unsafe {
                                            let (a, b) = transmute::<i16, (u8, u8)>(n);
                                        
                                            match self.data_memory.get_mut(stack_counter) {
                                                Some(mem_cell_1) => {
                                                    *mem_cell_1 = b;
                                                }
                                                None => self.exit_with_positional_error("Stack overflow while loading data memory.", vptk.position),
                                            }

                                            match self.data_memory.get_mut(stack_counter + 1) {
                                                Some(mem_cell_2) => {
                                                    *mem_cell_2 = a;
                                                }
                                                None => self.exit_with_positional_error("Stack overflow while loading data memory.", vptk.position),
                                            }
                                        }
                                        stack_counter += 2;
                                    }
                                    Err(e) => self.exit_with_positional_error(e.as_str(), vptk.position),
                                }
                            } else {unreachable!();}
                        } 
                    } else {unreachable!();}
                }


                //////////////////////
                // CASE BE SPACE
                //////////////////////
                Token::Directive(Directive::Space) => {
                    if let DataArg::Number(ref ptk) = data_camp.arg {
                        if let Token::Number(ref number_token) = ptk.token {
                            match number_token.to_u16() {
                                Ok(num) => {
                                    match stack_counter.checked_add(num as usize) {
                                        Some(v) => stack_counter = v,
                                        None => self.exit_with_positional_error("Stack overflow while loading data memory.", ptk.position),
                                    }
                                    if stack_counter >= self.data_memory.len() {
                                        self.exit_with_positional_error("Stack overflow while loading data memory.", ptk.position);
                                    }
                                }
                                Err(e) => self.exit_with_positional_error(e.as_str(), ptk.position),
                            }
                        } else {unreachable!();}
                    } else {unreachable!();}
                }
                
                _ => unreachable!(),
            }

            println!("------> {}", stack_counter);
            
        }
    }
}
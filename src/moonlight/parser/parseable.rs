use crate::moonlight::{debug::Debugable, utils::*, Moonlight};
use super::ast::*;

enum Field {
    Data,
    Inst,
}
pub trait Parseable {
    fn parse(&mut self, tokens: &Vec<PositionedToken>) -> Ast;

    fn read_comma_separated_tokens(&self, tokens: &Vec<PositionedToken>, start_index: usize) -> Vec<PositionedToken>;
    fn read_lw_sw_format(&self, tokens: &Vec<PositionedToken>, start_index: usize) -> InstrArg;
}

impl Parseable for Moonlight {
    fn parse(&mut self, tokens: &Vec<PositionedToken>) -> Ast{
        let mut label_declarations_accumulator: Vec<PositionedToken> = Vec::new();
        let mut data_field: Vec<DataCamp> = Vec::new();
        let mut instr_field: Vec<InstrCamp> = Vec::new();
        let mut current_field: Field = Field::Inst;

        let mut ptk_index = 0;
        while ptk_index < tokens.len() {
            let ptk = match tokens.get(ptk_index).cloned() {
                Some(ptk) => ptk,
                None => break,
            };
            println!("======= Processing token: {:?} =======", ptk);
            match ptk.token {
                Token::Directive(Directive::Data) => {
                    current_field = Field::Data;
                    ptk_index += 1;
                    continue;
                }
                Token::Directive(Directive::Inst) => {
                    current_field = Field::Inst;
                    ptk_index += 1;
                    continue;
                }
                _ => {
                    match current_field {
                        Field::Data => {
                            match ptk.token {
                                Token::LabelDeclaration(_) => {
                                    label_declarations_accumulator.push(ptk.clone());
                                    ptk_index += 1;
                                    continue;
                                }
                                Token::Directive(Directive::Word) | Token::Directive(Directive::Byte) => {
                                    let data = self.read_comma_separated_tokens(tokens, ptk_index + 1);
                                    for d in &data {
                                        println!("----> data token: {:?}", d);
                                    }
                                    let data_len = data.len();
                                    data_field.push(
                                        DataCamp::new(
                                            label_declarations_accumulator.clone(),
                                            ptk.clone(),
                                            data
                                        )
                                    );
                                    ptk_index += data_len * 2 ;
                                    label_declarations_accumulator.clear();
                                    continue;
                                }
                                _ => self.exit_with_positional_error("Unexpected token in data field", ptk.position),
                                    
                            }
                        }
                        Field::Inst => {
                            match ptk.token {
                                Token::LabelDeclaration(_) => {
                                    label_declarations_accumulator.push(ptk.clone());
                                    ptk_index += 1;
                                    continue;
                                }
                                Token::PseudoInstruction(ref psinstr) => {
                                    match psinstr {
                                        PseudoInstruction::Lw | PseudoInstruction::Sw => {
                                            let lw_sw_tokens = self.read_lw_sw_format(&tokens, ptk_index + 1);
                                            instr_field.push(
                                                InstrCamp::new(
                                                    label_declarations_accumulator.clone(),
                                                    ptk.clone(),
                                                    lw_sw_tokens,
                                                ),
                                            );

                                            ptk_index += 7;
                                            label_declarations_accumulator.clear();
                                            continue;
                                        }
                                        _ => unimplemented!("Unexpected pseudo instruction: {:?}", psinstr),
                                    }
                                }
                                _ => unimplemented!("Unexpected token in instruction field: {:?}", ptk.token),
                            }
                        }
                    }
                }
            }
            //ptk_index += 1;
        }

        Ast {
            data_field,
            instr_field,
        }
    }


    fn read_comma_separated_tokens(&self, tokens: &Vec<PositionedToken>, start_index: usize) -> Vec<PositionedToken> {
        let mut result = Vec::new();
        let mut current_index = start_index;
        
        while let Some(ptk) = tokens.get(current_index) {
            match ptk.token {
                Token::Comma => return result,
                _ => result.push(ptk.clone()),
            }

            match tokens.get(current_index + 1) {
                Some(next_ptk) => {
                    match next_ptk.token {
                        Token::Comma => {
                            current_index += 2;
                        }
                        _ => return result,
                    }
                }
                None => return result,
            }
        }
        result
    }

    fn read_lw_sw_format(&self, tokens: &Vec<PositionedToken>, start_index: usize) -> InstrArg {
        /*
            PositionedToken { token: Accumulator(Ac0), position: Position { file: 0, line: 4, column: Some(12) } }
            PositionedToken { token: Comma, position: Position { file: 0, line: 4, column: Some(12) } }
            PositionedToken { token: LabelReference("_num"), position: Position { file: 0, line: 4, column: Some(16) } }
            PositionedToken { token: LeftSquareBracket, position: Position { file: 0, line: 4, column: Some(16) } }
            PositionedToken { token: Number(Integer(0)), position: Position { file: 0, line: 4, column: Some(21) } }
            PositionedToken { token: RightSquareBracket, position: Position { file: 0, line: 4, column: Some(21) } }     
        */
        let mut tokens_to_process = Vec::new();
        for i in 0..6 {
            if let Some(ptk) = tokens.get(start_index + i) {
                tokens_to_process.push(ptk.clone());
            } else {
                break;
            }
        } 

        match tokens_to_process.get(0) {
            Some(ptk0) => {
                match ptk0.token {
                    Token::Accumulator(_) => {
                        match tokens_to_process.get(1) {
                            Some(ptk1) => {
                                match ptk1.token {
                                    Token::Comma => {
                                        match tokens_to_process.get(2) {
                                            Some(ptk2) => {
                                                match ptk2.token {
                                                    Token::LabelReference(_) => {
                                                        match tokens_to_process.get(3) {
                                                            Some(ptk3) => {
                                                                match ptk3.token {
                                                                    Token::LeftSquareBracket => {
                                                                        match tokens_to_process.get(4) {
                                                                            Some(ptk4) => {
                                                                                match ptk4.token {
                                                                                    Token::Number(_) => {
                                                                                        match tokens_to_process.get(5) {
                                                                                            Some(ptk5) => {
                                                                                                match ptk5.token {
                                                                                                    Token::RightSquareBracket => {
                                                                                                        return InstrArg::LwSw {
                                                                                                            accumulator: ptk0.clone(),
                                                                                                            label_reference: ptk2.clone(),
                                                                                                            number: ptk4.clone(),
                                                                                                        };
                                                                                                    }
                                                                                                    _ => self.exit_with_positional_error("Expect a right square bracket after number in memory pseudo instruction format", ptk5.position),
                                                                                                }
                                                                                            }
                                                                                            None => self.exit_with_positional_error("Expect a right square bracket after number in memory pseudo instruction format", ptk4.position),
                                                                                        }
                                                                                    }
                                                                                    _ => self.exit_with_positional_error("Expect a number after left square bracket in memory pseudo instruction format", ptk4.position),
                                                                                }
                                                                            }
                                                                            None => self.exit_with_positional_error("Expect a number after left square bracket in memory pseudo instruction format", ptk3.position),
                                                                        }
                                                                    }
                                                                    _ => self.exit_with_positional_error("Expect a left square bracket after label reference in memory pseudo instruction format", ptk3.position),
                                                                }
                                                            }
                                                            None => self.exit_with_positional_error("Expect a left square bracket after label reference in memory pseudo instruction format", ptk2.position),
                                                        }
                                                    }
                                                    _ => self.exit_with_positional_error("Expect a label reference after comma in memory pseudo instruction format", ptk2.position),
                                                }
                                            }
                                            None => self.exit_with_positional_error("Expect a label reference after comma in memory pseudo instruction format", ptk1.position),
                                        }
                                    }
                                    _ => self.exit_with_positional_error("Expect a comma after accumulator in memory pseudo instruction format", ptk1.position),
                                }
                            }
                            None => self.exit_with_positional_error("Expect a comma after accumulator in memory pseudo instruction format", ptk0.position),
                        }
                    }
                    _ => self.exit_with_positional_error("Expect an accumulator for memory pseudo instruction format", ptk0.position),
                }
            }
            None => {
                match tokens.get(start_index-1) {
                    Some(bptk) => {
                        self.exit_with_positional_error("Expect an accumulator after pseudo instruction", bptk.position);
                    }
                    None => unreachable!(),
                }
            }
        }
        unreachable!();
    }

}


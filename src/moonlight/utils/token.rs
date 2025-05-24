use super::{directive::Directive, instruction::Instruction, number::Number, pseudo_instruction::PseudoInstruction};
use crate::moonlight::utils::*;

trait TokenStringTrait {
    fn processed_string(&self) -> String;
}

impl TokenStringTrait for String {
    fn processed_string(&self) -> String {
        let mut result = String::new();
        let mut chars = self.chars().peekable();
        
        while let Some(ch) = chars.next() {
            if ch == '\\' {
                match chars.next() {
                    Some('n') => result.push('\n'),
                    Some('t') => result.push('\t'),
                    Some('r') => result.push('\r'),
                    Some('"') => result.push('"'),
                    Some('\\') => result.push('\\'),
                    Some('0') => result.push('\0'),
                    Some(ch) => {
                        // Tratar sequência de escape inválida incluindo-a como está
                        result.push('\\');
                        result.push(ch);
                    },
                    None => {
                        // Sequência incompleta, adiciona a barra invertida
                        result.push('\\');
                    },
                }
            } else {
                result.push(ch);
            }
        }
        
        result
    }
}


#[derive(Debug, Clone)]
pub enum Token {
    Number(Number),
    StringLiteral(String),
    LabelDeclaration(String),
    LabelReference(String),
    Instruction(Instruction),
    PseudoInstruction(PseudoInstruction),
    Directive(Directive),
}

impl Token {
    pub fn from_string(token_string: &String) -> Self {
        match token_string.as_str() {
            "nope" => Token::Instruction(Instruction::Nope),

            "add" => Token::Instruction(Instruction::Add),
            "sub" => Token::Instruction(Instruction::Sub),
            "not" => Token::Instruction(Instruction::Not),
            "and" => Token::Instruction(Instruction::And),
            "or" => Token::Instruction(Instruction::Or),
            "xor" => Token::Instruction(Instruction::Xor),
            "nand" => Token::Instruction(Instruction::Nand),
            "nor" => Token::Instruction(Instruction::Nor),
            "xnor" => Token::Instruction(Instruction::Xnor),
            "slt" => Token::Instruction(Instruction::Slt),
            "tmul" => Token::Instruction(Instruction::Tmul),
            "tdiv" => Token::Instruction(Instruction::Tdiv),

            "sll" => Token::Instruction(Instruction::Sll),
            "srl" => Token::Instruction(Instruction::Srl),
            "sra" => Token::Instruction(Instruction::Sra),

            "mtl" => Token::Instruction(Instruction::Mtl),
            "mfl" => Token::Instruction(Instruction::Mfl),
            "mth" => Token::Instruction(Instruction::Mth),
            "mfh" => Token::Instruction(Instruction::Mfh),
            "mtac" => Token::Instruction(Instruction::Mtac),
            "mfac" => Token::Instruction(Instruction::Mfac),

            "addi" => Token::Instruction(Instruction::Addi),
            "subi" => Token::Instruction(Instruction::Subi),
            "andi" => Token::Instruction(Instruction::Andi),
            "ori" => Token::Instruction(Instruction::Ori),
            "xori" => Token::Instruction(Instruction::Xori),
            "nandi" => Token::Instruction(Instruction::Nandi),
            "nori" => Token::Instruction(Instruction::Nori),
            "xnori" => Token::Instruction(Instruction::Xnori),
            "lli" => Token::Instruction(Instruction::Lli),
            "lui" => Token::Instruction(Instruction::Lui),
            "lsi" => Token::Instruction(Instruction::Lsi),

            "lwr" => Token::Instruction(Instruction::Lwr),
            "swr" => Token::Instruction(Instruction::Swr),
            "push" => Token::Instruction(Instruction::Push),
            "pop" => Token::Instruction(Instruction::Pop),

            "jr" => Token::Instruction(Instruction::Jr),
            "jrl" => Token::Instruction(Instruction::Jrl),
            "ja" => Token::Instruction(Instruction::Ja),
            "jal" => Token::Instruction(Instruction::Jal),
            "bgtz" => Token::Instruction(Instruction::Bgtz),
            "bltz" => Token::Instruction(Instruction::Bltz),
            "beqz" => Token::Instruction(Instruction::Beqz),
            "bnez" => Token::Instruction(Instruction::Bnez),
            "bgtzr" => Token::Instruction(Instruction::Bgtzr),
            "bltzr" => Token::Instruction(Instruction::Bltzr),
            "beqzr" => Token::Instruction(Instruction::Beqzr),
            "bnezr" => Token::Instruction(Instruction::Bnezr),

            // Directives
            ".include" => Token::Directive(Directive::Include),
            ".data" => Token::Directive(Directive::Data),
            ".space" => Token::Directive(Directive::Space),
            ".word" => Token::Directive(Directive::Word),
            ".ascii" => Token::Directive(Directive::Ascii),
            ".byte" => Token::Directive(Directive::Byte),
            ".inst" => Token::Directive(Directive::Inst),

            // Pseudo instructions
            "jump" => Token::PseudoInstruction(PseudoInstruction::Jump),
            "lw" => Token::PseudoInstruction(PseudoInstruction::Lw),
            "sw" => Token::PseudoInstruction(PseudoInstruction::Sw),
            "mul" => Token::PseudoInstruction(PseudoInstruction::Mul),
            "div" => Token::PseudoInstruction(PseudoInstruction::Div),
            "swap" => Token::PseudoInstruction(PseudoInstruction::Swap),
            "call" => Token::PseudoInstruction(PseudoInstruction::Call),
            "ret" => Token::PseudoInstruction(PseudoInstruction::Ret),
            
            _ if token_string.starts_with("_") && token_string.ends_with(":") => {
                let label = token_string.trim_end_matches(":").to_string();
                Token::LabelDeclaration(label)
            }
            _ if token_string.starts_with("\"") && token_string.ends_with("\"") => {
                let string_literal = token_string.trim_matches('"').to_string().processed_string();
                Token::StringLiteral(string_literal)
            }
            _ if token_string.parse::<i32>().is_ok() => {
                Token::Number(Number::Integer(token_string.parse::<i32>().unwrap()))
            }
            _ if token_string.to_lowercase().starts_with("0b") => {
                Token::Number(Number::Binary(token_string.to_string()))
            }
            _ if token_string.to_lowercase().starts_with("0x") => {
                Token::Number(Number::Hexadecimal(token_string.to_string()))
            }
            _ => {
                Token::LabelReference(token_string.clone())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct PositionedToken {
    pub token: Token,
    pub position: Position,
}
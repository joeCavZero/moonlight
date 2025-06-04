use super::{directive::Directive, instruction::Instruction, number::Number, pseudo_instruction::PseudoInstruction};
use crate::moonlight::utils::*;



#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(Number),
    StringLiteral(String),
    LabelDeclaration(String),
    LabelReference(String),
    Instruction(Instruction),
    PseudoInstruction(PseudoInstruction),
    Directive(Directive),

    Accumulator(Accumulator),
    Register(Register),

    Comma,
    LeftSquareBracket,
    RightSquareBracket,
}

impl Token {
    pub fn from_string(token_string: &String) -> Result<Self, String> {
        match token_string.as_str() {
            "nope" => Ok(Token::Instruction(Instruction::Nope)),

            "add" => Ok(Token::Instruction(Instruction::Add)),
            "sub" => Ok(Token::Instruction(Instruction::Sub)),
            "not" => Ok(Token::Instruction(Instruction::Not)),
            "and" => Ok(Token::Instruction(Instruction::And)),
            "or" => Ok(Token::Instruction(Instruction::Or)),
            "xor" => Ok(Token::Instruction(Instruction::Xor)),
            "nand" => Ok(Token::Instruction(Instruction::Nand)),
            "nor" => Ok(Token::Instruction(Instruction::Nor)),
            "xnor" => Ok(Token::Instruction(Instruction::Xnor)),
            "slt" => Ok(Token::Instruction(Instruction::Slt)),
            "tmul" => Ok(Token::Instruction(Instruction::Tmul)),
            "tdiv" => Ok(Token::Instruction(Instruction::Tdiv)),

            "sll" => Ok(Token::Instruction(Instruction::Sll)),
            "srl" => Ok(Token::Instruction(Instruction::Srl)),
            "sra" => Ok(Token::Instruction(Instruction::Sra)),

            "mtl" => Ok(Token::Instruction(Instruction::Mtl)),
            "mfl" => Ok(Token::Instruction(Instruction::Mfl)),
            "mth" => Ok(Token::Instruction(Instruction::Mth)),
            "mfh" => Ok(Token::Instruction(Instruction::Mfh)),
            "mtac" => Ok(Token::Instruction(Instruction::Mtac)),
            "mfac" => Ok(Token::Instruction(Instruction::Mfac)),

            "addi" => Ok(Token::Instruction(Instruction::Addi)),
            "subi" => Ok(Token::Instruction(Instruction::Subi)),
            "andi" => Ok(Token::Instruction(Instruction::Andi)),
            "ori" => Ok(Token::Instruction(Instruction::Ori)),
            "xori" => Ok(Token::Instruction(Instruction::Xori)),
            "nandi" => Ok(Token::Instruction(Instruction::Nandi)),
            "nori" => Ok(Token::Instruction(Instruction::Nori)),
            "xnori" => Ok(Token::Instruction(Instruction::Xnori)),
            "lli" => Ok(Token::Instruction(Instruction::Lli)),
            "lui" => Ok(Token::Instruction(Instruction::Lui)),
            "lsi" => Ok(Token::Instruction(Instruction::Lsi)),

            "lwr" => Ok(Token::Instruction(Instruction::Lwr)),
            "swr" => Ok(Token::Instruction(Instruction::Swr)),
            "push" => Ok(Token::Instruction(Instruction::Push)),
            "pop" => Ok(Token::Instruction(Instruction::Pop)),

            "jr" => Ok(Token::Instruction(Instruction::Jr)),
            "jrl" => Ok(Token::Instruction(Instruction::Jrl)),
            "ja" => Ok(Token::Instruction(Instruction::Ja)),
            "jal" => Ok(Token::Instruction(Instruction::Jal)),
            "bgtz" => Ok(Token::Instruction(Instruction::Bgtz)),
            "bltz" => Ok(Token::Instruction(Instruction::Bltz)),
            "beqz" => Ok(Token::Instruction(Instruction::Beqz)),
            "bnez" => Ok(Token::Instruction(Instruction::Bnez)),
            "bgtzr" => Ok(Token::Instruction(Instruction::Bgtzr)),
            "bltzr" => Ok(Token::Instruction(Instruction::Bltzr)),
            "beqzr" => Ok(Token::Instruction(Instruction::Beqzr)),
            "bnezr" => Ok(Token::Instruction(Instruction::Bnezr)),

            // Directives
            ".include" => Ok(Token::Directive(Directive::Include)),
            ".data" => Ok(Token::Directive(Directive::Data)),
            ".space" => Ok(Token::Directive(Directive::Space)),
            ".word" => Ok(Token::Directive(Directive::Word)),
            ".ascii" => Ok(Token::Directive(Directive::Ascii)),
            ".byte" => Ok(Token::Directive(Directive::Byte)),
            ".inst" => Ok(Token::Directive(Directive::Inst)),

            // Pseudo instructions
            "jump" => Ok(Token::PseudoInstruction(PseudoInstruction::Jump)),
            "lw" => Ok(Token::PseudoInstruction(PseudoInstruction::Lw)),
            "sw" => Ok(Token::PseudoInstruction(PseudoInstruction::Sw)),
            "mul" => Ok(Token::PseudoInstruction(PseudoInstruction::Mul)),
            "div" => Ok(Token::PseudoInstruction(PseudoInstruction::Div)),
            "swap" => Ok(Token::PseudoInstruction(PseudoInstruction::Swap)),
            "call" => Ok(Token::PseudoInstruction(PseudoInstruction::Call)),
            "ret" => Ok(Token::PseudoInstruction(PseudoInstruction::Ret)),
            
            "," => Ok(Token::Comma),
            "[" => Ok(Token::LeftSquareBracket),
            "]" => Ok(Token::RightSquareBracket),

            _ if token_string.starts_with("&") => match token_string.as_str() {
                "&0" => Ok(Token::Accumulator(Accumulator::Ac0)),
                "&1" => Ok(Token::Accumulator(Accumulator::Ac1)),
                "&2" => Ok(Token::Accumulator(Accumulator::Ac2)),
                "&3" => Ok(Token::Accumulator(Accumulator::Ac3)),
                _ => Err(format!("Invalid accumulator token: {}", token_string)),
            }

            _ if token_string.starts_with("$") => match token_string.as_str() {
                "$0" => Ok(Token::Register(Register::Rf0)),
                "$1" => Ok(Token::Register(Register::Rf1)),
                "$2" => Ok(Token::Register(Register::Rf2)),
                "$3" => Ok(Token::Register(Register::Rf3)),
                "$4" => Ok(Token::Register(Register::Rf4)),
                "$5" => Ok(Token::Register(Register::Rf5)),
                "$6" => Ok(Token::Register(Register::Rf6)),
                "$7" => Ok(Token::Register(Register::Rf7)),
                "$8" => Ok(Token::Register(Register::Rf8)),
                "$9" => Ok(Token::Register(Register::Rf9)),
                "$10" => Ok(Token::Register(Register::Rf10)),
                "$11" => Ok(Token::Register(Register::Rf11)),
                "$12" => Ok(Token::Register(Register::Rf12)),
                "$13" => Ok(Token::Register(Register::Rf13)),
                "$14" => Ok(Token::Register(Register::Rf14)),
                "$15" => Ok(Token::Register(Register::Rf15)),
                _ => Err(format!("Invalid register token: {}", token_string)),
            }
            _ if token_string.starts_with("_") => {
                if token_string.ends_with(":") {
                    let label = token_string.trim_end_matches(":").to_string();
                    Ok(Token::LabelDeclaration(label))
                } else {
                    Ok(Token::LabelReference(token_string.clone()))
                }
            }
            _ if token_string.starts_with("\"") && token_string.ends_with("\"") => {
                let string_literal = token_string.trim_matches('"').to_string().processed_string();
                Ok(Token::StringLiteral(string_literal))
            }
            _ if token_string.parse::<i32>().is_ok() => {
                Ok(Token::Number(Number::Integer(token_string.parse::<i32>().unwrap())))
            }
            _ if token_string.to_lowercase().starts_with("0b") => {
                Ok(Token::Number(Number::Binary(token_string.to_string())))
            }
            _ if token_string.to_lowercase().starts_with("0x") => {
                Ok(Token::Number(Number::Hexadecimal(token_string.to_string())))
            }
            _ => {
                Err(format!("Invalid token: {}", token_string))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct PositionedToken {
    pub token: Token,
    pub position: Position,
}
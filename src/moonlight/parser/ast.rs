use crate::moonlight::utils::*;

pub enum DataValue {
    Number(Number),
    StringLiteral(String),
}

pub struct DataCamp {
    pub label_declarations: Vec<String>,
    pub directive: Directive,
    pub data: DataValue,
}

pub struct InstrCamp {
    pub label_declarations: Vec<String>,
    pub instruction: Instruction
}

pub struct Ast {
    pub data_field: Vec<DataCamp>,
    pub instr_field: Vec<InstrCamp>
}
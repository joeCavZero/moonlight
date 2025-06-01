use crate::moonlight::utils::*;

#[derive(Debug, Clone)]
pub struct DataCamp {
    pub label_declarations: Vec<PositionedToken>,
    pub directive: PositionedToken,
    pub data: Vec<PositionedToken>,
}

impl DataCamp {
    pub fn new(label_declarations: Vec<PositionedToken>, directive: PositionedToken, data: Vec<PositionedToken>) -> Self {
        DataCamp {
            label_declarations,
            directive,
            data,
        }
    }
}

#[derive(Debug, Clone)]
pub struct InstrCamp {
    pub label_declarations: Vec<PositionedToken>,
    pub instruction: PositionedToken,
    pub args: Vec<PositionedToken>,
}

impl InstrCamp {
    pub fn new(label_declarations: Vec<PositionedToken>, instruction: PositionedToken, args: Vec<PositionedToken>) -> Self {
        InstrCamp {
            label_declarations,
            instruction,
            args,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ast {
    pub data_field: Vec<DataCamp>,
    pub instr_field: Vec<InstrCamp>
}
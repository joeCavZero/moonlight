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
pub enum InstrArg {
    Empty,
    AcRR {
        ac: PositionedToken,
        r1: PositionedToken,
        r2: PositionedToken,
    },
    AcR {
        ac: PositionedToken,
        r: PositionedToken,
    },
    R {
        r: PositionedToken,
    },
    AcRNumber {
        ac: PositionedToken,
        r: PositionedToken,
        number: PositionedToken,
    },
    Ac {
        ac: PositionedToken,
    },
    AcNumber {
        ac: PositionedToken,
        number: PositionedToken,
    },
    Number {
        number: PositionedToken,
    },

    // Instruction arguments for pseudo instructions
    Jump {
        target: PositionedToken,
    },
    MulDivSwap {
        rf: PositionedToken,
        rg: PositionedToken,
    },
    LwSw {
        ac: PositionedToken,
        label_reference: PositionedToken,
        number: PositionedToken,
    },
    Call {
        target: PositionedToken,
    }
}

impl InstrArg {
    pub fn new_empty() -> Self {
        InstrArg::Empty
    }

    pub fn new_ac_r_r(ac: PositionedToken, r1: PositionedToken, r2: PositionedToken) -> Self {
        InstrArg::AcRR { ac, r1, r2 }
    }

    pub fn new_ac_r(ac: PositionedToken, r: PositionedToken) -> Self {
        InstrArg::AcR { ac, r }
    }

    pub fn new_r(r: PositionedToken) -> Self {
        InstrArg::R { r }
    }

    pub fn new_ac_r_number(ac: PositionedToken, r: PositionedToken, number: PositionedToken) -> Self {
        InstrArg::AcRNumber { ac, r, number }
    }

    pub fn new_ac(ac: PositionedToken) -> Self {
        InstrArg::Ac { ac }
    }

    pub fn new_ac_number(ac: PositionedToken, number: PositionedToken) -> Self {
        InstrArg::AcNumber { ac, number }
    }

    pub fn new_number(number: PositionedToken) -> Self {
        InstrArg::Number { number }
    }

    pub fn new_jump(target: PositionedToken) -> Self {
        InstrArg::Jump { target }
    }

    pub fn new_mul_div_swap(rf: PositionedToken, rg: PositionedToken) -> Self {
        InstrArg::MulDivSwap { rf, rg }
    }

    pub fn new_lw_sw(ac: PositionedToken, label_reference: PositionedToken, number: PositionedToken) -> Self {
        InstrArg::LwSw {
            ac,
            label_reference,
            number,
        }
    }

    pub fn new_call(target: PositionedToken) -> Self {
        InstrArg::Call { target }
    }
    
}

#[derive(Debug, Clone)]
pub struct InstrCamp {
    pub label_declarations: Vec<PositionedToken>,
    pub instruction: PositionedToken,
    pub arg: InstrArg,
}

impl InstrCamp {
    pub fn new(label_declarations: Vec<PositionedToken>, instruction: PositionedToken, arg: InstrArg) -> Self {
        InstrCamp {
            label_declarations,
            instruction,
            arg,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ast {
    pub data_field: Vec<DataCamp>,
    pub instr_field: Vec<InstrCamp>
}
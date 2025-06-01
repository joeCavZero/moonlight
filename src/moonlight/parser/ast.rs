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
    N,
    R {
        ac: PositionedToken,
        rf: PositionedToken,
        rg: PositionedToken,
    },
    I {
        ac: PositionedToken,
        number: PositionedToken,
    },
    S {
        ac: PositionedToken,
        rf: PositionedToken,
        number: PositionedToken,
    },
    J {
        number: PositionedToken,
    },

    E1 {
        ac: PositionedToken,
        rf: PositionedToken,
    },
    E2 {
        rf: PositionedToken,
    },
    E3 {
        ac: PositionedToken,
    },
    E4 {
        rf: PositionedToken,
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
    },
    Ret,
}

impl InstrArg {
    pub fn new_n() -> Self {
        InstrArg::N
    }

    pub fn new_r(ac: PositionedToken, rf: PositionedToken, rg: PositionedToken) -> Self {
        InstrArg::R { ac, rf, rg }
    }

    pub fn new_i(ac: PositionedToken, number: PositionedToken) -> Self {
        InstrArg::I { ac, number }
    }

    pub fn new_s(ac: PositionedToken, rf: PositionedToken, number: PositionedToken) -> Self {
        InstrArg::S { ac, rf, number }
    }

    pub fn new_j(number: PositionedToken) -> Self {
        InstrArg::J { number }
    }

    pub fn new_e1(ac: PositionedToken, rf: PositionedToken) -> Self {
        InstrArg::E1 { ac, rf }
    }

    pub fn new_e2(rf: PositionedToken) -> Self {
        InstrArg::E2 { rf }
    }

    pub fn new_e3(ac: PositionedToken) -> Self {
        InstrArg::E3 { ac }
    }

    pub fn new_e4(rf: PositionedToken) -> Self {
        InstrArg::E4 { rf }
    }

    // Pseudo instruction arguments

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

    pub fn new_ret() -> Self {
        InstrArg::Ret
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
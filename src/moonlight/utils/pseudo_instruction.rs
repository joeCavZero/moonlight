#[derive(Debug, Clone)]
pub enum PseudoInstruction {
    Jump,
    Lw,
    Sw,
    Mul,
    Div,
    Swap,
    Call,
    Ret,
}
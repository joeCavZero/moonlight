#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone)]
pub enum Directive {
    Include,
    Data,
    Space,
    Word,
    Ascii,
    Byte,
    Inst,
}
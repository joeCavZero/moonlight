#[derive(Debug, Clone, PartialEq)]
pub enum Directive {
    Include,
    Data,
    Space,
    Word,
    Ascii,
    Byte,
    Inst,
}
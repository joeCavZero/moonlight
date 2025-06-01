#[derive(Debug, Clone, PartialEq)]
pub enum Register {
    Rf0,
    Rf1,
    Rf2,
    Rf3,
    Rf4,
    Rf5,
    Rf6,
    Rf7,
    Rf8,
    Rf9,
    Rf10,
    Rf11,
    Rf12,
    Rf13,
    Rf14, // Stack Pointer Register
    Rf15, // Link Register
}
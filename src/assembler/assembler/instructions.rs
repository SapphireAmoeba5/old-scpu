use super::{registers::Register, identifier::Identifier};

#[derive(Debug)]
pub enum Instruction {
    Hlt,
    MovReg64Reg64(Register, Register),
    MovReg64Constant(Register, u64),
}

impl Instruction {
    // Returns a tuple, the u8 is the opcode, and the bool is whether the opcode needs to be
    // represented by one or two bytes
    pub fn opcode(&self) -> (u8, bool) {
        match self {
            Self::Hlt => (0, false),
            Self::MovReg64Reg64(..) => (1, false), 
            Self::MovReg64Constant(..) => (2, false),
        }
    }
}

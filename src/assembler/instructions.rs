use super::registers::Register;

pub enum Instruction {
    MovReg64Reg64(Register, Register),
}

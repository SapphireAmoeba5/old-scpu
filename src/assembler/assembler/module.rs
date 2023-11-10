use crate::assembler::number::Width;

use super::{
    identifier::Identifier, instructions::Instruction, lexer::lex_string, number::Number,
    opcode::Opcode, operation::Operation, registers::Register, symbol::Symbol,
};
use anyhow::{bail, Result};

#[derive(Debug)]
enum SourceCodeToken {
    Instruction(Opcode, Vec<SourceCodeToken>),
    Identifier(Identifier),
    Constant(Number),
    Register(Register),
    LabelDeclaration(Identifier),
}

enum GenericTwoOperandInstruction {
    Reg64Reg64(Register, Register),
    //Reg64Constant(Register, u64),
}

pub struct Module {
    symbols: Vec<Symbol>,
    tokens: Vec<Operation>,
}

impl Module {
    pub fn new() -> Self {
        Self {
            symbols: Vec::new(),
            tokens: Vec::new(),
        }
    }
    pub fn preprocess(&mut self, source: String) -> Result<Self> {
        println!("Preprocessing:\n{}", source);

        let mut preprocess_operations: Vec<SourceCodeToken> = vec![
            SourceCodeToken::Instruction(
                Opcode::Mov,
                vec![
                    SourceCodeToken::Register(Register::X0),
                    SourceCodeToken::Register(Register::X0),
                ],
            ),
            SourceCodeToken::Instruction(Opcode::Hlt, Vec::new()),
        ];

        let mut current_byte_offset: usize = 0;
        for token in preprocess_operations {
            match token {
                SourceCodeToken::Instruction(opcode, operands) => {
                    let (instruction, width) = self.process_instruction(opcode, operands)?;
                    println!("Instr: {:?}, width: {}", instruction, width);
                }

                SourceCodeToken::LabelDeclaration(id) => {
                    self.symbols.push(Symbol::Label(id, current_byte_offset.try_into().unwrap()));
                }

                _ => {
                    println!("Random token!");
                }
            }
        }

        todo!("Not implemented yet")
    }

    // returns a tuple containing the instruction, and the width of the instruction in bytes all
    // wrapped in a result
    fn process_instruction(
        &self,
        opcode: Opcode,
        operands: Vec<SourceCodeToken>,
    ) -> Result<(Instruction, usize)> {
        match opcode {
            Opcode::Hlt => self.process_hlt_instruction(operands),
            Opcode::Mov => self.process_mov_instruction(operands),
        }
    }

    fn process_hlt_instruction(
        &self,
        operands: Vec<SourceCodeToken>,
    ) -> Result<(Instruction, usize)> {
        if !operands.is_empty() {
            bail!(
                "Hlt instructions expects zero operands but found {} operands",
                operands.len()
            );
        }

        Ok((Instruction::Hlt, 1))
    }

    fn process_mov_instruction(
        &self,
        operands: Vec<SourceCodeToken>,
    ) -> Result<(Instruction, usize)> {
        let instr = self.process_generic_two_operand_instruction(operands)?;
        match instr {
            GenericTwoOperandInstruction::Reg64Reg64(dst_reg, src_reg) => Ok((Instruction::MovReg64Reg64(dst_reg, src_reg), 100)),
        
        }
    }

    fn process_generic_two_operand_instruction(&self, mut operands: Vec<SourceCodeToken>) -> Result<GenericTwoOperandInstruction> {
        if operands.len() != 2 {
            bail!("Instruction expects 2 operands but found {} operands", operands.len());
        }

        let right_operand = operands.pop().unwrap();
        let left_operand = operands.pop().unwrap();

        let dest_register = match left_operand {
            SourceCodeToken::Register(reg) => reg,
            _ => bail!("Left operand for instruction must be a register"),
        };

        match right_operand {
            SourceCodeToken::Register(src_register) => Ok(GenericTwoOperandInstruction::Reg64Reg64(src_register, dest_register)),
            _ => bail!("Right operand for instruction is invalid"),
        }


    }

    fn subprocess_mov_instruction_reg_unknown(&self, right_operand: SourceCodeToken) -> Result<(Instruction, usize)> {
        println!("Right operand: {:?}", right_operand);
        todo!()
    }
}

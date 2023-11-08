use crate::assembler::number::Width;

use super::{
    identifier::Identifier, instructions::Instruction, lexer::lex_string, number::Number,
    opcode::Opcode, operation::Operation, registers::Register, symbol::Symbol,
};
use anyhow::{bail, Result};

#[derive(Debug)]
enum PreprocessOperation {
    Instruction(Opcode, Vec<PreprocessOperation>),
    Identifier(Identifier),
    Constant(Number),
    Register(Register),
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

        let mut preprocess_operations: Vec<PreprocessOperation> = vec![
            PreprocessOperation::Instruction(
                Opcode::Mov,
                vec![
                    PreprocessOperation::Register(Register::X0),
                    PreprocessOperation::Constant(Number::from_number(10, Width::Qword)),
                ],
            ),
            PreprocessOperation::Instruction(Opcode::Hlt, Vec::new()),
        ];

        let mut current_byte_offset: usize = 0;
        for token in preprocess_operations {
            match token {
                PreprocessOperation::Instruction(opcode, operands) => {
                    let (instruction, width) = self.process_instruction(opcode, operands)?;
                    println!("Instr: {:?}, width: {}", instruction, width);
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
        operands: Vec<PreprocessOperation>,
    ) -> Result<(Instruction, usize)> {
        match opcode {
            Opcode::Hlt => self.process_hlt_instruction(operands),
            Opcode::Mov => self.process_mov_instruction(operands),
        }
    }

    fn process_hlt_instruction(
        &self,
        operands: Vec<PreprocessOperation>,
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
        mut operands: Vec<PreprocessOperation>,
    ) -> Result<(Instruction, usize)> {
        if operands.len() != 2 {
            bail!("Mov instruction expects 2 operands, but found {} operands", operands.len());
        }
        
        let right_operand = operands.pop().unwrap();
        let left_operand = operands.pop().unwrap();

        match left_operand {
            PreprocessOperation::Register(register) => self.subprocess_mov_instruction_reg_unknown(right_operand),
            _ => bail!("Invalid left operand for mov instruction"),
        }
    }

    fn subprocess_mov_instruction_reg_unknown(&self, right_operand: PreprocessOperation) -> Result<(Instruction, usize)> {
        println!("Right operand: {:?}", right_operand);
        todo!()
    }
}

use crate::assembler::number::Width;

use super::{
    identifier::Identifier, instructions::Instruction, lexer::lex_string, number::Number,
    opcode::Opcode, operation::Operation, registers::Register, symbol::Symbol,
};
use anyhow::{bail, Result};

enum PreprocessOperation {
    Instruction(Opcode, Vec<PreprocessOperation>),
    Identifier(Identifier),
    Constant(Number),
    Register(Register),
}

pub struct Module {
    symbols: Vec<Symbol>,
    preprocessed_tokens: Vec<Operation>,
}

impl Module {
    pub fn new() -> Self {
        Self {
            symbols: Vec::new(),
            preprocessed_tokens: Vec::new(),
        }
    }
    pub fn preprocess(&mut self, source: String) -> Result<Self> {
        println!("Preprocessing:\n{}", source);

        let preprocess_operations: Vec<PreprocessOperation> =
            vec![PreprocessOperation::Instruction(
                Opcode::Mov,
                vec![
                    PreprocessOperation::Register(Register::X0),
                    PreprocessOperation::Constant(Number::from_number(10, Width::Qword)),
                ],
            )];

        let mut current_byte_offset: usize = 0;
        for token in preprocess_operations {
            match token {
                PreprocessOperation::Instruction(opcode, operands) => {
                    let (instruction, width) = self.process_instruction(opcode, operands)?;
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
        Ok((Instruction::Halt, 1))
    }
}

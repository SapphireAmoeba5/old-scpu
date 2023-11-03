use super::{symbol::Symbol, opcode::Opcode, lexer::lex_string, identifier::Identifier};
use anyhow::{Result, bail};

enum PreprocessOperation {
    Instruction(Opcode, Vec<PreprocessOperation>),
    Identifier(Identifier),
}

pub struct Module {
    symbols: Vec<Symbol>,
}

impl Module {
   pub fn preprocess(source: String) -> Result<Self> {
        println!("Preprocessing:\n{}", source);

        bail!("FOIENGOINEG")
   } 
}

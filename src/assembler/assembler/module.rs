use super::{symbol::Symbol, opcode::Opcode, lexer::lex_string};

enum PreprocessOperation {
    Instruction(Opcode, Vec<PreprocessOperation>),
}

pub struct Module {
    public_symbols: Vec<Symbol>,
    symbols: Vec<Symbol>,
}

impl Module {

}

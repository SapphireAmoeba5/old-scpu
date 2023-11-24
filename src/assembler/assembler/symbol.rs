use super::identifier::Identifier;

#[derive(Debug)]
pub enum Symbol {
    // Value
    Constant(u64),
    // Label name, offset
    Label(Identifier, u64),
}

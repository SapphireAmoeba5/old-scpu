use super::identifier::Identifier;

pub enum Symbol {
    // Value
    Constant(u64),
    // Label name, offset
    Label(Identifier, u64),
}

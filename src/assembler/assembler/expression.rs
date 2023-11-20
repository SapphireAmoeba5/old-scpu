use super::identifier::Identifier;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Arity {
    Unary,
    Binary,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Associativity {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,

    Neg,
}

impl Operation {
    pub fn get_precedence(&self) -> usize {
        match self {
            Self::Add | Self::Sub => 0,
            Self::Mul | Self::Div => 1,

            Self::Neg => 2,
        }
    }

    pub fn get_arity(&self) -> Arity {
        match self {
            Self::Add | Self::Sub | Self::Mul | Self::Div => Arity::Binary,

            Self::Neg => Arity::Unary,
        }
    }

    pub fn get_associativity(&self) -> Associativity {
        match self {
            Self::Add | Self::Sub | Self::Mul | Self::Div => Associativity::Left,
            Self::Neg => Associativity::Right,
        }
    }
}

pub enum Value {
    Number(u64),
    Identifier(Identifier),
}

#[derive(Debug)]
pub struct Expression {

}

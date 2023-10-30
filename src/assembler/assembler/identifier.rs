use anyhow::{Result, bail};

fn is_valid_id_char(ch: char) -> bool {
    if ch.is_ascii_alphanumeric() || ch.is_ascii_alphabetic() {
        return true;
    }

    match ch {
        '_' | '$' => true,
        _ => false,
    }
}

fn is_valid_identifier(id: &str) -> bool {
    if id.starts_with(|ch: char| ch.is_ascii_alphanumeric()) {
        return false;
    }

    for ch in id.chars() {
        if !is_valid_id_char(ch) {
            return false;
        }
    }

    true
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Identifier(String);

impl Identifier {
    pub fn new(id: String) -> Result<Self> {
        if !is_valid_identifier(&id) {
            bail!("'{}' is not a valid identifier", id);
        } 

        Ok(Self(id))
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

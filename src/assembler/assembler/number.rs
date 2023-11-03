pub enum Width {
    Byte,
    Word,
    Dword,
    Qword,
}

// For now, the width field is irrelevent
pub struct Number(u64, Width);


impl Number {
    pub fn from_number(value: u64, width: Width) -> Self {
        Self(value, width)
    }
}


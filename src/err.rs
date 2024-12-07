use std::error::Error;

pub enum BitfieldError {
    IndexOutOfRange,
}

impl Error for BitfieldError {}
impl std::fmt::Display for BitfieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Index out of range")
    }
}
impl std::fmt::Debug for BitfieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Index out of range")
    }
}

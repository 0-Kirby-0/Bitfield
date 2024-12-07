//#![allow(dead_code)]
mod err;

mod bitfield;
mod bitset;

pub use bitfield::Bitfield;
pub use bitset::Bitset;
pub use err::BitfieldError;

type Byte = u8;

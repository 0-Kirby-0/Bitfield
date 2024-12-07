mod interactors;
mod ops;

#[derive(Clone, PartialEq, Eq)]
pub struct Bitset<const BYTES: usize> {
    pub bytes: [u8; BYTES],
}
#[macro_export]
macro_rules! bitset {
    ($n:expr) => {
        $crate::bitset::Bitset::<{ $crate::bitset::Bitset::<0>::bytes_needed_for_bits($n) }>::default()
    };
}

impl<const BYTES: usize> Bitset<BYTES> {
    pub const fn capacity(&self) -> usize {
        BYTES * 8
    }
    pub const fn bytes_needed_for_bits(bits: usize) -> usize {
        (bits + 7) / 8
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    pub fn bytes_mut(&mut self) -> &mut [u8] {
        &mut self.bytes
    }
}

impl<const BYTES: usize> std::fmt::Debug for Bitset<BYTES> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bitset<{}> ", BYTES * 8)?;
        for byte in self.bytes.iter().rev() {
            write!(f, "{:08b}", byte)?;
        }
        write!(f, "")
    }
}

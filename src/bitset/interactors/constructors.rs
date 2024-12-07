use crate::Bitset;

impl<const BYTES: usize> Default for Bitset<BYTES> {
    fn default() -> Self {
        Self { bytes: [0; BYTES] }
    }
}

impl<const BYTES: usize> Bitset<BYTES> {
    pub fn cast_from(value: impl Into<usize>) -> Option<Self> {
        let mut out = Self::default();
        out.set_from_castable(value).ok()?;
        Some(out)
    }

    pub fn new_from_bit_iter(iter: impl Iterator<Item = bool>) -> Option<Self> {
        let mut out = Self::default();
        out.set_from_bit_iter(iter).ok()?;
        Some(out)
    }

    pub fn new_from_index(index: impl Into<usize>) -> Option<Self> {
        let mut new = Self::default();
        new.set_bit(index, true).ok()?;
        Some(new)
    }

    pub fn new_from_index_iter(mut iter: impl Iterator<Item = impl Into<usize>>) -> Option<Self> {
        let mut new = Self::default();
        iter.try_for_each(|index| new.set_bit(index, true)).ok()?;
        Some(new)
    }
}

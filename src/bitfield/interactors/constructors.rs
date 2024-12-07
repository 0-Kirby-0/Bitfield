use crate::{Bitfield, BitfieldError, Bitset};

impl<const FIELDS: usize, const BYTES: usize> Default for Bitfield<FIELDS, BYTES> {
    fn default() -> Self {
        Self {
            sets: std::array::from_fn(|_| Bitset::default()),
        }
    }
}

impl<const FIELDS: usize, const BYTES: usize> Bitfield<FIELDS, BYTES> {
    #[inline]
    fn build(setter: impl FnOnce(&mut Self) -> Result<(), BitfieldError>) -> Option<Self> {
        let mut built = Self::default();
        setter(&mut built).ok()?;
        Some(built)
    }

    pub fn new_from_set_iter(iter: impl Iterator<Item = Bitset<BYTES>>) -> Option<Self> {
        Self::build(move |this| this.set_from_set_iter(iter))
    }
    pub fn new_from_castable_iter(iter: impl Iterator<Item = impl Into<usize>>) -> Option<Self> {
        Self::build(move |this| this.set_from_castable_iter(iter))
    }
    pub fn new_from_bit_iters(
        iter: impl Iterator<Item = impl Iterator<Item = bool>>,
    ) -> Option<Self> {
        Self::build(move |this| this.set_from_bit_iters(iter))
    }
    pub fn new_from_index_iters(
        iter: impl Iterator<Item = impl Iterator<Item = impl Into<usize>>>,
    ) -> Option<Self> {
        Self::build(move |this| this.set_from_index_iters(iter))
    }
}

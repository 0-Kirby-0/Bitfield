use crate::{Bitfield, BitfieldError, Bitset};

impl<const SETS: usize, const BYTES: usize> Bitfield<SETS, BYTES> {
    // Direct access to the sets
    pub fn get_set(&self, index: usize) -> Result<&Bitset<BYTES>, BitfieldError> {
        self.sets.get(index).ok_or(BitfieldError::IndexOutOfRange)
    }
    pub fn get_set_iter(&self) -> impl Iterator<Item = &Bitset<BYTES>> {
        self.sets.iter()
    }

    // Actually useful getters
    pub fn get_bit(&self, set_idx: impl Into<usize>, bit_idx: impl Into<usize>) -> Option<bool> {
        self.sets
            .get(set_idx.into())
            .and_then(|set| set.get_bit(bit_idx))
    }
    pub fn cast_to_iter<T: From<usize>>(&self) -> impl Iterator<Item = T> + '_ {
        self.sets.iter().map(|set| set.get_casted())
    }
    pub fn bit_iters(&self) -> impl Iterator<Item = impl Iterator<Item = bool> + '_> + '_ {
        self.sets.iter().map(|set| set.bit_iter())
    }
    pub fn bit_indices_iters(&self) -> impl Iterator<Item = impl Iterator<Item = usize> + '_> + '_ {
        self.sets.iter().map(|set| set.bit_indices_iter())
    }
}

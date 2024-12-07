use crate::{Bitfield, BitfieldError, Bitset};

impl<const SETS: usize, const BYTES: usize> Bitfield<SETS, BYTES> {
    // Direct access to the sets
    pub fn set_set(&mut self, index: usize, other: Bitset<BYTES>) -> Result<(), BitfieldError> {
        self.sets
            .get_mut(index)
            .map(|set| *set = other)
            .map(|_| Ok(()))
            .unwrap_or(Err(BitfieldError::IndexOutOfRange))
    }
    pub fn set_from_set_iter(
        &mut self,
        iter: impl Iterator<Item = Bitset<BYTES>>,
    ) -> Result<(), BitfieldError> {
        iter.enumerate()
            .try_for_each(|(index, set)| self.set_set(index, set))
    }

    // Actually useful setters
    pub fn set_bit(
        &mut self,
        set_idx: impl Into<usize>,
        bit_idx: impl Into<usize>,
        bit: bool,
    ) -> Result<(), BitfieldError> {
        self.sets
            .get_mut(set_idx.into())
            .map(|set| set.set_bit(bit_idx, bit))
            .unwrap_or(Err(BitfieldError::IndexOutOfRange))
    }

    pub fn set_from_castable_iter(
        &mut self,
        iter: impl Iterator<Item = impl Into<usize>>,
    ) -> Result<(), BitfieldError> {
        iter.enumerate()
            .try_for_each(|(index, value)| {
                self.sets
                    .get_mut(index)
                    .and_then(|set| set.set_from_castable(value).ok())
            })
            .ok_or(BitfieldError::IndexOutOfRange)
    }

    pub fn set_from_bit_iters(
        &mut self,
        iter: impl Iterator<Item = impl Iterator<Item = bool>>,
    ) -> Result<(), BitfieldError> {
        iter.enumerate()
            .try_for_each(|(index, iter)| {
                self.sets
                    .get_mut(index)
                    .and_then(|set| set.set_from_bit_iter(iter).ok())
            })
            .ok_or(BitfieldError::IndexOutOfRange)
    }
    pub fn set_from_index_iters(
        &mut self,
        iter: impl Iterator<Item = impl Iterator<Item = impl Into<usize>>>,
    ) -> Result<(), BitfieldError> {
        iter.enumerate()
            .try_for_each(|(index, iter)| {
                self.sets
                    .get_mut(index)
                    .and_then(|set| set.set_from_index_iter(iter).ok())
            })
            .ok_or(BitfieldError::IndexOutOfRange)
    }
}

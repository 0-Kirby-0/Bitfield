use super::*;
use crate::Bitset;

impl<const BYTES: usize> Bitset<BYTES> {
    pub fn get_bit(&self, index: impl Into<usize>) -> Option<bool> {
        let index = index.into();
        if index >= self.capacity() {
            None
        } else {
            let (byte_index, bit_index) = get_bit_indeces(index);
            Some(get_bit_in_byte(self.bytes[byte_index], bit_index))
        }
    }

    /// Casts the bitset to a given T.
    /// Should T be too small to hold the bitset, the higher bits will be truncated.
    // As far as I understand, there is no good way to detect the size of a generic type.
    pub fn get_casted<T: From<usize>>(&self) -> T {
        self.bytes
            .iter()
            .enumerate()
            .fold(0, |acc, (i, &byte)| acc | (byte as usize) << (i * 8))
            .into()
    }

    /// Returns an iterator over the bits of the set.
    pub fn bit_iter(&self) -> impl Iterator<Item = bool> + '_ {
        self.bytes
            .iter()
            .flat_map(|&byte| (0..8).map(move |index| get_bit_in_byte(byte, index)))
    }

    /// Returns an iterator over the indices of the set (true) bits.
    pub fn bit_indices_iter(&self) -> impl Iterator<Item = usize> + '_ {
        self.bytes
            .iter()
            .enumerate()
            .flat_map(|(byte_index, &byte)| {
                (0..8).map(move |bit_index| {
                    (byte_index * 8 + bit_index, get_bit_in_byte(byte, bit_index))
                })
            })
            .filter_map(
                move |(bit_index, bit)| {
                    if bit {
                        Some(bit_index)
                    } else {
                        None
                    }
                },
            )
    }
}

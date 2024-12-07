use super::*;
use crate::Bitset;

impl<const BYTES: usize> Bitset<BYTES> {
    pub fn cast_into<T: From<usize>>(self) -> T {
        self.bytes
            .into_iter()
            .enumerate()
            .fold(0, |acc, (i, byte)| acc | (byte as usize) << (i * 8))
            .into()
    }

    pub fn into_bit_iter(self) -> impl Iterator<Item = bool> {
        self.bytes
            .into_iter()
            .flat_map(|byte| (0..8).map(move |index| get_bit_in_byte(byte, index)))
    }

    pub fn into_bit_indeces_iter(self) -> impl Iterator<Item = usize> {
        self.bytes
            .into_iter()
            .enumerate()
            .flat_map(|(byte_index, byte)| {
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

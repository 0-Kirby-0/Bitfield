use super::*;
use crate::{BitfieldError, Bitset, Byte};

impl<const BYTES: usize> Bitset<BYTES> {
    pub fn set_bit(&mut self, index: impl Into<usize>, bit: bool) -> Result<(), BitfieldError> {
        let index = index.into();
        if index >= self.capacity() {
            Err(BitfieldError::IndexOutOfRange)
        } else {
            let (byte_index, bit_index) = get_bit_indeces(index);
            self.bytes[byte_index] = set_bit_in_byte(self.bytes[byte_index], bit_index, bit);
            Ok(())
        }
    }

    pub fn set_from_castable(&mut self, value: impl Into<usize>) -> Result<(), BitfieldError> {
        let value = value.into();
        if value >= self.capacity() {
            Err(BitfieldError::IndexOutOfRange)
        } else {
            self.bytes.iter_mut().enumerate().for_each(|(i, byte)| {
                *byte = (value >> (i * 8)) as Byte;
            });
            Ok(())
        }
    }

    pub fn set_from_bit_iter(
        &mut self,
        iter: impl Iterator<Item = bool>,
    ) -> Result<(), BitfieldError> {
        iter.enumerate()
            .try_for_each(|(index, bit)| self.set_bit(index, bit))
    }

    pub fn set_from_index_iter(
        &mut self,
        iter: impl Iterator<Item = impl Into<usize>>,
    ) -> Result<(), BitfieldError> {
        iter.into_iter()
            .try_for_each(|index| self.set_bit(index, true))
    }
}

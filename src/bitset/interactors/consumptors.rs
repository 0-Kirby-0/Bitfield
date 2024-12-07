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

impl<const OWN_BYTES: usize> Bitset<OWN_BYTES> {
    /// Truncates the bitset to a given size.
    ///
    /// If the target set is larger, the new fields are zeroed.
    ///
    /// Is the target field is smaller, the excess data is discarded.
    /// - If you want to panic if the target field is smaller, see `fit_into`.
    /// - If you want to fallibly fit the bitfield into a smaller one, see `try_fit_into`.
    pub fn truncate<const OTHER_BYTES: usize>(self) -> Bitset<OTHER_BYTES> {
        let mut out = Bitset::default();
        out.bytes
            .iter_mut()
            .zip(self.bytes)
            .for_each(|(out_byte, self_byte)| {
                *out_byte = self_byte;
            });
        out
    }

    /// Fits the bitset into one of a different size.
    ///
    /// If the target set is larger, the new fields are zeroed.
    ///
    /// If the target field is smaller, the function panics.
    /// - If you want to fallibly fit the bitset into a smaller one, see `try_fit_into`.
    /// - If you want to truncate the bitset to a given size, see `truncate`.
    pub fn fit_into<const OTHER_BYTES: usize>(self) -> Bitset<OTHER_BYTES> {
        if OWN_BYTES <= OTHER_BYTES {
            panic!("Tried to fit a bitset into a smaller one")
        } else {
            self.truncate()
        }
    }

    /// Tries to fit the bitset into one of a different size.
    ///
    /// If the target set is larger, the new fields are zeroed.
    ///
    /// If the target field is smaller, the function returns `None`.
    /// - If you want to panic if the target field is smaller, see `fit_into`.
    /// - If you want to truncate the bitset to a given size, see `truncate`.
    pub fn try_fit_into<const OTHER_BYTES: usize>(self) -> Option<Bitset<OTHER_BYTES>> {
        if OWN_BYTES <= OTHER_BYTES {
            Some(self.truncate())
        } else {
            None
        }
    }
}

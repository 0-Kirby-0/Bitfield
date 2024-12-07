use crate::{Bitfield, Bitset};

impl<const FIELDS: usize, const BYTES: usize> Bitfield<FIELDS, BYTES> {
    pub fn into_set_iter(self) -> impl Iterator<Item = Bitset<BYTES>> {
        self.sets.into_iter()
    }
    pub fn into_castable_iter(self) -> impl Iterator<Item = usize> {
        self.sets.into_iter().map(|set| set.cast_into())
    }
    pub fn into_bit_iters(self) -> impl Iterator<Item = impl Iterator<Item = bool>> {
        self.sets.into_iter().map(|set| set.into_bit_iter())
    }
    pub fn into_index_iters(self) -> impl Iterator<Item = impl Iterator<Item = usize>> {
        self.sets.into_iter().map(|set| set.into_bit_indeces_iter())
    }
}

impl<const OWN_FIELDS: usize, const OWN_BYTES: usize> Bitfield<OWN_FIELDS, OWN_BYTES> {
    /// Fits the bitfield into one of a different size.
    ///
    /// If the target field is larger in any dimension, the new bits are zeroed.
    ///
    ///  If the target field is smaller, the function panics.
    /// - If you want to fallibly fit the bitfield into a smaller one, see `try_fit_into`.
    /// - If you want to truncate the bitfield to a given size, see `truncate`.
    pub fn fit_into<const OTHER_FIELDS: usize, const OTHER_BYTES: usize>(
        self,
    ) -> Bitfield<OTHER_FIELDS, OTHER_BYTES> {
        if OWN_FIELDS <= OTHER_FIELDS && OTHER_BYTES <= OWN_BYTES {
            panic!("Tried to fit a bitfield into a smaller one")
        } else {
            self.truncate()
        }
    }

    /// Tries to fit the bitfield into one of a different size.
    ///
    /// If the target field is larger in any dimension, the new bits are zeroed.
    ///
    /// If the target field is smaller, the function returns `None`.
    /// - If you want to panic if the target field is smaller, see `fit_into`.
    /// - If you want to truncate the bitfield to a given size, see `truncate`.
    pub fn try_fit_into<const OTHER_FIELDS: usize, const OTHER_BYTES: usize>(
        self,
    ) -> Option<Bitfield<OTHER_FIELDS, OTHER_BYTES>> {
        if OWN_FIELDS <= OTHER_FIELDS && OTHER_BYTES <= OWN_BYTES {
            Some(self.truncate())
        } else {
            None
        }
    }

    /// Truncates the bitfield to a given size.
    ///
    /// If the target field is larger in any dimension, the new fields are zeroed.
    ///
    /// Is the target field is smaller, the excess data is discarded.
    /// - If you want to panic if the target field is smaller, see `fit_into`.
    /// - If you want to fallibly fit the bitfield into a smaller one, see `try_fit_into`.
    pub fn truncate<const OTHER_FIELDS: usize, const OTHER_BYTES: usize>(
        self,
    ) -> Bitfield<OTHER_FIELDS, OTHER_BYTES> {
        let mut out = Bitfield::default();
        out.sets
            .iter_mut()
            .zip(self.sets)
            .for_each(|(out_set, self_set)| {
                out_set
                    .bytes
                    .iter_mut()
                    .zip(self_set.bytes)
                    .for_each(|(out_byte, self_byte)| {
                        *out_byte = self_byte;
                    });
            });
        out
    }
}

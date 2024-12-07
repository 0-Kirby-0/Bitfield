use super::*;
use crate::Bitfield;

//Helper

impl<const SETS: usize, const BYTES: usize> std::ops::BitOrAssign for Bitfield<SETS, BYTES> {
    fn bitor_assign(&mut self, other: Self) {
        merge_into(self, other, |a, b| *a |= b);
    }
}
impl<const SETS: usize, const BYTES: usize> std::ops::BitOr for Bitfield<SETS, BYTES> {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        let mut result = self;
        result |= other;
        result
    }
}

impl<const SETS: usize, const BYTES: usize> std::ops::BitAndAssign for Bitfield<SETS, BYTES> {
    fn bitand_assign(&mut self, other: Self) {
        merge_into(self, other, |a, b| *a &= b);
    }
}
impl<const SETS: usize, const BYTES: usize> std::ops::BitAnd for Bitfield<SETS, BYTES> {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        let mut result = self;
        result &= other;
        result
    }
}

//Helper

impl<const SETS: usize, const BYTES: usize> std::ops::Not for Bitfield<SETS, BYTES> {
    type Output = Self;
    fn not(self) -> Self {
        let result = self;
        apply_for_each(result, |set| !set)
    }
}

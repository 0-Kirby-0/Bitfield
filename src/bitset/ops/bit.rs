use super::*;
use crate::Bitset;

impl<const BYTES: usize> std::ops::BitAndAssign for Bitset<BYTES> {
    fn bitand_assign(&mut self, other: Self) {
        merge_into(self, other, |a, b, _| *a &= b);
    }
}

impl<const BYTES: usize> std::ops::BitAnd for Bitset<BYTES> {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        let mut result = self;
        result &= other;
        result
    }
}

impl<const BYTES: usize> std::ops::BitOrAssign for Bitset<BYTES> {
    fn bitor_assign(&mut self, other: Self) {
        merge_into(self, other, |a, b, _| *a |= b);
    }
}

impl<const BYTES: usize> std::ops::BitOr for Bitset<BYTES> {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        let mut result = self;
        result |= other;
        result
    }
}

impl<const BYTES: usize> std::ops::Not for Bitset<BYTES> {
    type Output = Self;
    fn not(self) -> Self {
        let result = self;
        apply_for_each(result, |byte| !byte)
    }
}

impl<const BYTES: usize> std::ops::BitXorAssign for Bitset<BYTES> {
    fn bitxor_assign(&mut self, other: Self) {
        merge_into(self, other, |a, b, _| *a ^= b);
    }
}

impl<const BYTES: usize> std::ops::BitXor for Bitset<BYTES> {
    type Output = Self;
    fn bitxor(self, other: Self) -> Self {
        let mut result = self;
        result ^= other;
        result
    }
}

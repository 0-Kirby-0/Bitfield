use crate::Bitset;

use super::*;

impl<const BYTES: usize> std::ops::AddAssign for Bitset<BYTES> {
    /// If the result of adding does not fit in the bitset, the carry is ignored.
    fn add_assign(&mut self, rhs: Self) {
        merge_into(self, rhs, |a, b, carry| {
            if *carry {
                (*a, *carry) = a.overflowing_add(1);
            }
            (*a, *carry) = a.overflowing_add(b);
        });
    }
}

impl<const BYTES: usize> std::ops::Add for Bitset<BYTES> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut result = self;
        result += rhs;
        result
    }
}

impl<const BYTES: usize> std::ops::SubAssign for Bitset<BYTES> {
    /// If the result of subtracting does not fit in the bitset, the carry is ignored.
    fn sub_assign(&mut self, rhs: Self) {
        merge_into(self, rhs, |a, b, carry| {
            if *carry {
                (*a, *carry) = a.overflowing_sub(1);
            }
            (*a, *carry) = a.overflowing_sub(b);
        });
    }
}

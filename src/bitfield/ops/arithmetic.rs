use super::*;
use crate::Bitfield;

impl<const SETS: usize, const BYTES: usize> std::ops::AddAssign for Bitfield<SETS, BYTES> {
    fn add_assign(&mut self, rhs: Self) {
        merge_into(self, rhs, |a, b| *a += b);
    }
}
impl<const SETS: usize, const BYTES: usize> std::ops::Add for Bitfield<SETS, BYTES> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut result = self;
        result += rhs;
        result
    }
}

impl<const SETS: usize, const BYTES: usize> std::ops::SubAssign for Bitfield<SETS, BYTES> {
    fn sub_assign(&mut self, rhs: Self) {
        merge_into(self, rhs, |a, b| *a -= b);
    }
}
impl<const SETS: usize, const BYTES: usize> std::ops::Sub for Bitfield<SETS, BYTES> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut result = self;
        result -= rhs;
        result
    }
}

mod arithmetic;
mod bit;

use crate::{Bitset, Byte};

//Helpers
#[inline]
fn apply_for_each<const BYTES: usize>(
    base: Bitset<BYTES>,
    apply_fn: impl Fn(Byte) -> Byte,
) -> Bitset<BYTES> {
    let mut result = base;
    (0..BYTES).for_each(|i| {
        let byte = std::mem::take(&mut result.bytes[i]);
        result.bytes[i] = apply_fn(byte);
    });
    result
}

#[inline]
fn merge_into<const BYTES: usize>(
    base: &mut Bitset<BYTES>,
    other: Bitset<BYTES>,
    merge_fn: impl Fn(&mut Byte, Byte, &mut bool),
) {
    let mut carry = false;
    base.bytes
        .iter_mut()
        .zip(other.bytes.iter())
        .for_each(|(a, &b)| {
            merge_fn(a, b, &mut carry);
        });
}

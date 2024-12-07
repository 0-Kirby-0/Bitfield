use crate::{Bitfield, Bitset};
mod arithmetic;

mod bit;

#[inline]
fn merge_into<const SETS: usize, const BYTES: usize>(
    base: &mut Bitfield<SETS, BYTES>,
    other: Bitfield<SETS, BYTES>,
    merge_fn: impl Fn(&mut Bitset<BYTES>, Bitset<BYTES>),
) {
    base.sets.iter_mut().zip(other.sets).for_each(|(a, b)| {
        merge_fn(a, b);
    });
}

#[inline]
fn apply_for_each<const SETS: usize, const BYTES: usize>(
    base: Bitfield<SETS, BYTES>,
    apply_fn: impl Fn(Bitset<BYTES>) -> Bitset<BYTES>,
) -> Bitfield<SETS, BYTES> {
    let mut result = base;
    (0..SETS).for_each(|i| {
        let field = std::mem::take(&mut result.sets[i]);
        result.sets[i] = apply_fn(field);
    });
    result
}

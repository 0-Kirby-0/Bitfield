use crate::Bitset;

mod interactors;
mod ops;
mod structured_access;

pub use interactors::*;
pub use ops::*;
pub use structured_access::*;

pub struct Bitfield<const SETS: usize, const BYTES: usize> {
    sets: [Bitset<BYTES>; SETS],
}

#[macro_export]
/// Create a new Bitfield with the given number of sets and bits per set.
macro_rules! bitfield {
    ($n:expr, $m:expr) => {
        $crate::bitfield::Bitfield::<
            { $n },
            { $crate::bitset::Bitset::<0>::bytes_needed_for_bits($m) },
        >::default()
    };
}

impl<const SETS: usize, const BYTES: usize> Bitfield<SETS, BYTES> {
    pub fn capacity(&self) -> usize {
        SETS
    }
    pub fn bit_capacity(&self) -> usize {
        SETS * BYTES * 8
    }

    pub fn sets(&self) -> &[Bitset<BYTES>] {
        &self.sets
    }
    pub fn sets_mut(&mut self) -> &mut [Bitset<BYTES>] {
        &mut self.sets
    }

    pub fn any(&self) -> bool {
        self.sets.iter().any(|set| set.any())
    }
    pub fn all(&self) -> bool {
        self.sets.iter().all(|set| set.all())
    }
    pub fn none(&self) -> bool {
        self.sets.iter().all(|set| set.none())
    }
}

impl<const SETS: usize, const BYTES: usize> std::fmt::Debug for Bitfield<SETS, BYTES> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bitfield<{}, {}> ", SETS, BYTES)?;
        writeln!(f)?;
        self.sets.iter().try_for_each(|set| {
            set.bytes()
                .iter()
                .rev()
                .try_for_each(|byte| write!(f, "{:08b}", byte))?;
            writeln!(f)
        })
    }
}

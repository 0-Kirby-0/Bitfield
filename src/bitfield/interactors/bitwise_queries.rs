use crate::Bitfield;
impl<const FIELDS: usize, const BYTES: usize> Bitfield<FIELDS, BYTES> {
    pub fn any(&self) -> bool {
        self.sets.iter().any(|set| set.any())
    }
    pub fn all(&self) -> bool {
        self.sets.iter().all(|set| set.all())
    }
    pub fn none(&self) -> bool {
        self.sets.iter().all(|set| set.none())
    }
    pub fn count_ones(&self) -> usize {
        self.sets.iter().map(|set| set.count_ones()).sum()
    }
    pub fn count_zeros(&self) -> usize {
        self.sets.iter().map(|set| set.count_zeros()).sum()
    }
}

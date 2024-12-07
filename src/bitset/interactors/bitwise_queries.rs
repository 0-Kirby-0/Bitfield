use crate::{Bitset, Byte};

impl<const BYTES: usize> Bitset<BYTES> {
    pub fn any(&self) -> bool {
        self.bytes.iter().any(|&byte| byte != 0)
    }
    pub fn all(&self) -> bool {
        self.bytes.iter().all(|&byte| byte == Byte::MAX)
    }
    pub fn none(&self) -> bool {
        self.bytes.iter().all(|&byte| byte == 0)
    }
    pub fn count_ones(&self) -> usize {
        self.bytes
            .iter()
            .map(|byte| byte.count_ones() as usize)
            .sum()
    }
    pub fn count_zeros(&self) -> usize {
        self.bytes
            .iter()
            .map(|byte| byte.count_zeros() as usize)
            .sum()
    }
    pub fn leading_zeros(&self) -> usize {
        self.bytes
            .iter()
            .rev()
            .try_fold(0, |acc, &byte| {
                if byte == 0 {
                    Ok(acc + 8)
                } else {
                    Err(acc + byte.leading_zeros() as usize)
                }
            })
            .unwrap_or_else(|acc| acc)
    }
    pub fn trailing_zeros(&self) -> usize {
        self.bytes
            .iter()
            .try_fold(0, |acc, &byte| {
                if byte == 0 {
                    Ok(acc + 8)
                } else {
                    Err(acc + byte.trailing_zeros() as usize)
                }
            })
            .unwrap_or_else(|acc| acc)
    }
    pub fn leading_ones(&self) -> usize {
        self.bytes
            .iter()
            .rev()
            .try_fold(0, |acc, &byte| {
                if byte == Byte::MAX {
                    Ok(acc + 8)
                } else {
                    Err(acc + (!byte).leading_zeros() as usize)
                }
            })
            .unwrap_or_else(|acc| acc)
    }
    pub fn trailing_ones(&self) -> usize {
        self.bytes
            .iter()
            .try_fold(0, |acc, &byte| {
                if byte == Byte::MAX {
                    Ok(acc + 8)
                } else {
                    Err(acc + (!byte).trailing_zeros() as usize)
                }
            })
            .unwrap_or_else(|acc| acc)
    }
}

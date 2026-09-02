use std::{cmp::Ordering, hash::Hash};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Orderedf64(pub f64);

impl Eq for Orderedf64 {}

impl PartialOrd for Orderedf64 {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Orderedf64 {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl Hash for Orderedf64 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

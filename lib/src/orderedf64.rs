use std::cmp::Ordering;

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Orderedf64(pub f64);

impl Eq for Orderedf64{}

impl PartialOrd for Orderedf64{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>{
        Some(self.cmp(other))
    }
}

impl Ord for Orderedf64{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl Orderedf64{
    pub fn unwrap(self) -> f64{
        self.0
    }
}
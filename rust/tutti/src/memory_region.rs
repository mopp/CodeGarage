use std::ops::Range;
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct MemoryRegion {
    pub addr: usize,
    pub size: usize,
}


impl MemoryRegion {
    pub fn new(addr: usize, size: usize) -> MemoryRegion
    {
        MemoryRegion {
            addr: addr,
            size: size,
        }
    }

    pub fn end_addr(&self) -> usize
    {
        self.addr + self.size
    }

    pub fn range(&self) -> Range<usize>
    {
        self.addr..self.end_addr()
    }
}


impl Ord for MemoryRegion {
    fn cmp(&self, other: &MemoryRegion) -> Ordering
    {
        self.addr.cmp(&other.addr)
    }
}


impl PartialOrd for MemoryRegion {
    fn partial_cmp(&self, other: &MemoryRegion) -> Option<Ordering>
    {
        Some(self.cmp(other))
    }
}

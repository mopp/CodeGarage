use std::ops::Range;

#[derive(Clone)]
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

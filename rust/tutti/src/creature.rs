extern crate rand;

use cpu::Cpu;
use memory_region::MemoryRegion;
use rand::Rng;
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Creature {
    pub core: Cpu,
    pub genome_region: MemoryRegion,
    pub daughter: Option<Box<Creature>>,
    pub mutate_threshold_copy_fail: usize,
    pub count_copy: usize,
    pub geno_type: Option<String>,
}

impl Ord for Creature {
    fn cmp(&self, other: &Creature) -> Ordering {
        self.core.count_fails.cmp(&other.core.count_fails)
    }
}

impl PartialOrd for Creature {
    fn partial_cmp(&self, other: &Creature) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Creature {
    pub fn new(g: MemoryRegion) -> Creature {
        let mut core = Cpu::new();
        core.ip = g.addr as u16;
        Creature {
            core: core,
            genome_region: g,
            daughter: None,
            mutate_threshold_copy_fail: 0,
            count_copy: 0,
            geno_type: None,
        }
    }

    pub fn randomize_mutate_threshold_copy_fail(&mut self) {
        self.mutate_threshold_copy_fail = rand::thread_rng().gen_range(1000, 2500);
    }
}

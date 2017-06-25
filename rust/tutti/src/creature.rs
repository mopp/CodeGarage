extern crate rand;

use cpu::Cpu;
use memory_region::MemoryRegion;
use rand::Rng;

#[derive(Clone)]
pub struct Creature {
    pub core: Cpu,
    pub genome_region: MemoryRegion,
    pub daughter: Option<Box<Creature>>,
    pub mutate_threshold_copy_fail: usize,
    pub count_copy: usize,
}


impl Creature {
    pub fn new(g: MemoryRegion) -> Creature
    {
        let mut core = Cpu::new();
        core.ip = g.addr as u16;
        Creature {
            core: core,
            genome_region: g,
            daughter: None,
            mutate_threshold_copy_fail: 0,
            count_copy: 0,
        }
    }

    pub fn randomize_mutate_threshold_copy_fail(&mut self)
    {
        self.mutate_threshold_copy_fail = rand::thread_rng().gen_range(1000, 2500);
    }
}


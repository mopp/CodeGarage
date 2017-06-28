extern crate rand;

use creature::Creature;
use gene_bank::GeneBank;
use instruction::Instruction;
use memory_region::MemoryRegion;
use rand::Rng;
use std::mem;


pub const UNIVERSE_TOTAL_GENOME_CAPACITY: usize = 8 * 1024;

pub struct Universe {
    genome_soup: [Instruction; UNIVERSE_TOTAL_GENOME_CAPACITY],
    pub free_regions: Vec<MemoryRegion>,
    pub creatures: Vec<Creature>,
    world_clock: usize,
    is_enable_random_mutate: bool,
    mutate_threshold_cosmic_rays: usize,
    pub gene_bank: GeneBank,
}


impl Universe {
    pub fn new() -> Universe
    {
        let soup = [Instruction::Nop0; UNIVERSE_TOTAL_GENOME_CAPACITY];
        let mut free_regions = Vec::new();
        free_regions.push(MemoryRegion::new(0, soup.len()));

        Universe {
            genome_soup: soup,
            free_regions: free_regions,
            creatures: Vec::new(),
            world_clock: 0,
            is_enable_random_mutate: false,
            mutate_threshold_cosmic_rays: 2500,
            gene_bank: GeneBank::new(),
        }
    }

    pub fn randomize_mutate_thresholds(&mut self)
    {
        if self.is_enable_random_mutate == false {
            return;
        }

        for c in self.creatures.iter_mut() {
            c.randomize_mutate_threshold_copy_fail();
        }
        self.randomize_mutate_threshold_cosmic_rays();
    }

    fn randomize_mutate_threshold_cosmic_rays(&mut self)
    {
        self.mutate_threshold_cosmic_rays = rand::thread_rng().gen_range(10000, 20000);
    }

    pub fn enable_random_mutate(&mut self)
    {
        self.is_enable_random_mutate = true;
    }

    pub fn generate_creature(&mut self, instructions: &[Instruction])
    {
        match self.allocate_genome_soup(instructions.len()) {
            None => panic!("no memory"),
            Some(genome_region) => {
                let mut c = Creature::new(genome_region);
                self.write_instructions(c.genome_region.addr, instructions);

                let v = instructions.to_vec();
                {
                    c.geno_type = self.gene_bank.register_genome(&v, None);
                    self.gene_bank.count_up_alive_genome(c.geno_type.as_ref().unwrap());
                }

                self.creatures.push(c);
            }
        }
    }

    fn allocate_genome_soup(&mut self, request_size: usize) -> Option<MemoryRegion> {
        // debug_assert!(request_size != 0);
        if request_size == 0 {
            return None;
        }

        let index = self.free_regions
            .iter()
            .position(|x| request_size <= x.size);

        match index {
            None => None,
            Some(index) => {
                let r = {
                    let v = self.free_regions.get_mut(index).unwrap();

                    let addr = v.addr;
                    v.addr += request_size;
                    v.size -= request_size;

                    Some(MemoryRegion::new(addr, request_size))
                };

                if self.free_regions[index].size == 0 {
                    self.free_regions.remove(index);
                }

                r
            }
        }
    }

    fn free_genome_soup(&mut self, r: MemoryRegion)
    {
        debug_assert!(r.size != 0);

        for v in self.free_regions.iter_mut() {
            if v.end_addr() == r.addr {
                v.size += r.size;
                return;
            } else if v.addr == r.end_addr() {
                v.addr -= r.size;
                v.size += r.size;
                return;
            }
        }

        self.free_regions.push(r);
        self.free_regions.sort();
    }

    pub fn compute_genome_soup_free_size(&self) -> usize
    {
        self.free_regions.iter().fold(0, |acc, ref x| acc + x.size)
    }

    pub fn compute_genome_soup_used_size(&self) -> usize
    {
        self.creatures.iter().fold(0, |acc, ref x| {
            acc + x.genome_region.size +
                match x.daughter {
                    None        => 0,
                    Some(ref d) => d.genome_region.size,
                }
        })
    }

    pub fn compute_genome_soup_free_rate(&self) -> f64
    {
        (self.compute_genome_soup_free_size() as f64 / (UNIVERSE_TOTAL_GENOME_CAPACITY as f64))
    }

    pub fn compute_genome_soup_used_rate(&self) -> f64
    {
        1.0 - self.compute_genome_soup_free_rate()
    }

    fn write_instructions(&mut self, addr: usize, src: &[Instruction])
    {
        let dst = &mut self.genome_soup[addr..(addr + src.len())];
        for (s, i) in dst.iter_mut().zip(src) {
            *s = *i
        }
    }

    fn search_complement_addr(&self, addr: usize, is_forward: bool) -> Option<(usize, usize)>
    {
        self.extract_argument_template(addr)
            .map(|template| {
                debug_assert!(template.len() != 0);
                template
                    .clone()
                    .into_iter()
                    .map(|&x| {
                        use Instruction::*;
                        match x {
                            Nop0 => Nop1,
                            Nop1 => Nop0,
                            _    => panic!("invalid instruction"),
                        }
                    })
                    .collect::<Vec<Instruction>>()
            })
            .and_then(|complement_template| {
                let len = complement_template.len();
                let is_equal_pattern = |window| complement_template == window;

                match is_forward {
                    true => {
                        (&self.genome_soup[(addr + len)..self.genome_soup.len()])
                            .windows(len)
                            .position(is_equal_pattern)
                            .map(|addr_diff| addr + len + addr_diff)
                    }
                    false => {
                        (&self.genome_soup[0..addr])
                            .windows(len)
                            .rposition(is_equal_pattern)
                            .map(|addr_diff| addr_diff)
                    }
                }
                .map(|complement_addr| (complement_addr, len))
            })
    }

    fn search_complement_addr_forward(&self, addr: usize) -> Option<(usize, usize)>
    {
        self.search_complement_addr(addr, true)
    }

    fn search_complement_addr_backward(&self, addr: usize) -> Option<(usize, usize)>
    {
        self.search_complement_addr(addr, false)
    }

    fn extract_argument_template(&self, addr: usize) -> Option<&[Instruction]>
    {
        // the addr have to be the beginning of the template you want to extract.
        // debug_assert_eq!(Instruction::is_nop(self.genome_soup[addr]), true);

        if (self.genome_soup.len() <= addr) || (Instruction::is_nop(self.genome_soup[addr]) == false) {
            return None;
        }

        let target_region = &self.genome_soup[addr..(self.genome_soup.len() - 1)];

        if target_region.len() == 0 {
            return None;
        }

        target_region
            .iter()
            .position(|&x| Instruction::is_nop(x) == false)
            .or(Some(target_region.len() - 1))
            .map(|tail_index| &target_region[0..tail_index])
            .and_then(|r| {
                if r.len() == 0 {
                    None
                } else {
                    Some(r)
                }
            })
    }

    fn execute(&mut self, creature: &mut Creature, ins: Instruction)
    {
        use Instruction::*;
        let mut cpu = creature.core.clone();
        let (ax, bx, cx, dx) = (cpu.ax, cpu.bx, cpu.cx, cpu.dx);
        match ins {
            Nop0 => cpu.count_up_fails(),
            Nop1 => cpu.count_up_fails(),
            Or1 => cpu.cx = cx ^ 1,
            Shl => cpu.cx = cx << 1,
            Zero => cpu.cx = 0,
            IfCz => {
                if cx != 0 {
                    // Skip the next instruction.
                    cpu.ip += 1;
                }
            }
            SubAb => cpu.cx = ax.overflowing_sub(bx).0,
            SubAc => cpu.ax = ax.overflowing_sub(cx).0,
            IncA => cpu.ax = ax.overflowing_add(1).0,
            IncB => cpu.bx = bx.overflowing_add(1).0,
            DecC => cpu.cx = cx.overflowing_sub(1).0,
            IncC => cpu.cx = cx.overflowing_add(1).0,
            PushAx => cpu.push(ax),
            PushBx => cpu.push(bx),
            PushCx => cpu.push(cx),
            PushDx => cpu.push(dx),
            PopAx => {
                match cpu.pop() {
                    Some(v) => cpu.ax = v,
                    None => cpu.count_up_fails(),
                }
            }
            PopBx => {
                match cpu.pop() {
                    Some(v) => cpu.bx = v,
                    None => cpu.count_up_fails(),
                }
            }
            PopCx => {
                match cpu.pop() {
                    Some(v) => cpu.cx = v,
                    None => cpu.count_up_fails(),
                }
            }
            PopDx => {
                match cpu.pop() {
                    Some(v) => cpu.dx = v,
                    None => cpu.count_up_fails(),
                }
            }
            Jmp | Jmpb | Call => {
                if ins == Call {
                    let ip = cpu.ip;
                    cpu.push(ip);
                }

                match self.search_complement_addr(cpu.ip as usize + 1, ins == Jmp || ins == Call) {
                    None               => cpu.count_up_fails(),
                    Some((addr, size)) => cpu.ip = (addr + size - 1) as u16,
                }
            }
            Ret => {
                match cpu.pop() {
                    None    => cpu.count_up_fails(),
                    Some(v) => cpu.ip = v,
                }
            }
            MovCd => cpu.dx = cx,
            MovAb => cpu.bx = ax,
            MovIab => {
                let ax = ax as usize;
                let bx = bx as usize;

                let is_writable = |x, r: &MemoryRegion| (r.addr <= x) && (x < r.end_addr());

                let is_write = {
                    let d = creature.daughter.as_ref();
                    if d.is_some() && is_writable(ax, &d.unwrap().genome_region) && (bx < self.genome_soup.len()) {
                        true
                    } else if is_writable(ax, &creature.genome_region) && (bx < self.genome_soup.len()) {
                        true
                    } else {
                        false
                    }
                };

                if is_write {
                    creature.count_copy += 1;
                    let ins = self.genome_soup[bx as usize];
                    self.genome_soup[ax as usize] = if self.is_enable_random_mutate && ((creature.count_copy % creature.mutate_threshold_copy_fail) == 0) {
                        creature.randomize_mutate_threshold_copy_fail();
                        ins.mutate_bit_randomly()
                    } else {
                        ins
                    }
                }
            }
            Adr => {
                let ip = cpu.ip as usize + 1;
                let f = self.search_complement_addr_forward(ip);
                let b = self.search_complement_addr_backward(ip);
                match (f, b) {
                    (None, None)               => cpu.count_up_fails(),
                    (None, Some((addr, size))) => cpu.ax = (addr + size) as u16,
                    (Some((addr, size)), None) => cpu.ax = (addr + size) as u16,
                    (Some((addr_f, size_f)), Some((addr_b, size_b))) => {
                        // Find the nearest one.
                        cpu.ax =
                            if (addr_f - ip) < (ip - addr_b) {
                                (addr_f + size_f) as u16
                            } else {
                                (addr_b + size_b) as u16
                            };
                    }
                }
            }
            Adrf | Adrb => {
                match self.search_complement_addr(cpu.ip as usize + 1, ins == Adrf) {
                    None               => cpu.count_up_fails(),
                    Some((addr, size)) => cpu.ax = (addr + size) as u16,
                }
            }
            Mal => {
                match self.allocate_genome_soup(cx as usize) {
                    None                => cpu.count_up_fails(),
                    Some(genome_region) => {
                        cpu.ax = genome_region.addr as u16;
                        if let Some(ref mut daughter) = creature.daughter {
                            self.free_genome_soup(daughter.genome_region);
                        }
                        creature.daughter = Some(Box::new(Creature::new(genome_region)));
                    }
                }
            }
            Divide => {
                if creature.daughter.is_some() {
                    let daughter = creature.daughter.clone();
                    creature.daughter = None;

                    let mut daughter = *daughter.unwrap();
                    let daughter_genome = self.genome_soup[daughter.genome_region.range()].to_vec();

                    {
                        daughter.geno_type = self.gene_bank.register_genome(&daughter_genome, creature.geno_type.as_ref());
                        debug_assert_eq!(daughter.geno_type.is_some(), true);
                        self.gene_bank.count_up_alive_genome(daughter.geno_type.as_ref().unwrap());
                    }

                    if self.is_enable_random_mutate {
                        daughter.randomize_mutate_threshold_copy_fail();
                    }
                    self.creatures.push(daughter);
                }
            }
        }

        creature.core = cpu;
    }

    fn fetch(&self, creature: &Creature) -> Instruction
    {
        self.genome_soup[creature.core.ip as usize]
    }

    fn increment_ip(&self, creature: &mut Creature)
    {
        let cpu = &mut creature.core;
        cpu.ip =
            if (self.genome_soup.len() - 1) <= (cpu.ip as usize) {
                creature.genome_region.addr as u16
            } else {
                cpu.ip + 1
            };
    }

    fn one_instruction_cycle(&mut self, creature: &mut Creature)
    {
        let ins = self.fetch(creature);
        self.execute(creature, ins);
        self.increment_ip(creature);

        // println!("Fetch: {:?}", ins);
        // println!("Execute: {}", creature.core);
    }

    fn execute_creature(&mut self, creature: &mut Creature, insts_count: usize)
    {
        for _ in 0..insts_count {
            self.one_instruction_cycle(creature);

            self.world_clock += 1;

            if self.is_enable_random_mutate && ((self.world_clock % self.mutate_threshold_cosmic_rays) == 0) {
                self.randomize_mutate_threshold_cosmic_rays();

                let target_index = rand::thread_rng().gen_range(0, self.genome_soup.len());
                let p = &mut self.genome_soup[target_index];
                *p = p.mutate_bit_randomly();
            }
        }
    }

    #[test]
    fn execute_creature_by_index(&mut self, index: usize, insts_count: usize)
    {
        let mut c = self.creatures[index].clone();
        self.execute_creature(&mut c, insts_count);
        self.creatures[index] = c;
    }

    pub fn execute_all_creatures(&mut self, power: f64)
    {
        self.creatures.sort();

        let mut cs = mem::replace(&mut self.creatures, Vec::new());
        for c in cs.iter_mut() {
            let size = c.genome_region.size as f64;
            let time_slice = size.powf(power).floor() as usize;
            self.execute_creature(c, time_slice);
        }
        cs.append(&mut self.creatures);
        self.creatures = cs;
    }

    pub fn wakeup_reaper_if_genome_usage_over(&mut self, threshold: f64)
    {
        while threshold < self.compute_genome_soup_used_rate() {
            match self.creatures.pop() {
                None => panic!("!?"),
                Some(target) => {
                    self.gene_bank.count_up_dead_genome(target.geno_type.as_ref().unwrap());

                    if let Some(daughter) = target.daughter {
                        self.free_genome_soup(daughter.genome_region);
                    }

                    self.free_genome_soup(target.genome_region);
                }
            }
        }
    }

    pub fn count_creatures(&self) -> usize
    {
        self.creatures.len()
    }

    pub fn gene_bank(&self) -> &GeneBank
    {
        &self.gene_bank
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use instruction::*;
    use instruction::Instruction::*;

    #[test]
    fn test_alloc_free()
    {
        let mut univ = Universe::new();
        assert_eq!(univ.allocate_genome_soup(0), None);

        let r1 = univ.allocate_genome_soup(10).unwrap();
        assert_eq!(r1.addr, 0);
        assert_eq!(r1.size, 10);
        assert_eq!(univ.compute_genome_soup_free_size(), UNIVERSE_TOTAL_GENOME_CAPACITY - 10);

        let r2 = univ.allocate_genome_soup(1000).unwrap();
        assert_eq!(r2.addr, 10);
        assert_eq!(r2.size, 1000);
        assert_eq!(univ.compute_genome_soup_free_size(), UNIVERSE_TOTAL_GENOME_CAPACITY - 10 - 1000);

        univ.free_genome_soup(r1);
        assert_eq!(univ.compute_genome_soup_free_size(), UNIVERSE_TOTAL_GENOME_CAPACITY - 1000);

        let r1 = univ.allocate_genome_soup(10).unwrap();
        assert_eq!(r1.addr, 0);
        assert_eq!(r1.size, 10);
        assert_eq!(univ.compute_genome_soup_free_size(), UNIVERSE_TOTAL_GENOME_CAPACITY - 10 - 1000);
        univ.free_genome_soup(r1);

        let r2 = univ.allocate_genome_soup(2000).unwrap();
        assert_eq!(r2.addr, 1010);
        assert_eq!(r2.size, 2000);
        assert_eq!(univ.compute_genome_soup_free_size(), UNIVERSE_TOTAL_GENOME_CAPACITY - 3000);

        let r3 = univ.allocate_genome_soup(500).unwrap();
        assert_eq!(r3.addr, 3010);
        assert_eq!(r3.size, 500);
        assert_eq!(univ.compute_genome_soup_free_size(), UNIVERSE_TOTAL_GENOME_CAPACITY - 3500);

        univ.free_genome_soup(r2);
        assert_eq!(univ.compute_genome_soup_free_size(), UNIVERSE_TOTAL_GENOME_CAPACITY - 1500);
    }

    #[test]
    fn test_extract_argument_template()
    {
        let mut univ = Universe::new();
        let insts = [
            Jmp,
            Nop0,
            Nop1,
            Jmp,
            Nop0,
            Jmp,
            Nop1,
            Nop1,
            Nop1,
            Nop1,
            Zero, // Dummy to terminal the template.
        ];

        univ.write_instructions(0, &insts);
        assert_eq!(univ.extract_argument_template(1), Some(vec![Nop0, Nop1].as_slice()));
        assert_eq!(univ.extract_argument_template(4), Some(vec![Nop0].as_slice()));
        assert_eq!(univ.extract_argument_template(6), Some(vec![Nop1, Nop1, Nop1, Nop1].as_slice()));
    }

    #[test]
    fn test_search_complement_addr()
    {
        let mut univ = Universe::new();
        let insts = [
            Jmp,
            Nop0,
            Nop1,
            Jmp,
            Nop1,
            Nop0,
            Zero, // Dummy to terminal the template.
        ];

        univ.write_instructions(0, &insts);
        assert_eq!(univ.search_complement_addr_forward(1), Some((4, 2)));
        assert_eq!(univ.search_complement_addr_forward(4), None);
        assert_eq!(univ.search_complement_addr_backward(4), Some((1, 2)));
        assert_eq!(univ.search_complement_addr_backward(1), None);

        let insts = [
            Zero, // Dummy to terminal the template.
            Nop0,
            Nop1,
            Jmp,
            Nop1,
            Nop0,
            Jmp,
            Nop0,
            Nop0,
            Nop0,
            Nop0,
            Zero,
            IncA,
            IncA,
            Jmp,
            Nop1,
            Nop1,
            Nop1,
            Nop1,
            Zero, // Dummy to terminal the template.
            ];
        univ.write_instructions(10, &insts);
        assert_eq!(univ.search_complement_addr_forward(10 + 1), Some((10 + 4, 2)));
        assert_eq!(univ.search_complement_addr_forward(10 + 7), Some((10 + 15, 4)));
        assert_eq!(univ.search_complement_addr_backward(10 + 4), Some((11, 2)));
        assert_eq!(univ.search_complement_addr_backward(10 + 7), None);
        assert_eq!(univ.search_complement_addr_backward(10 + 15), Some((17, 4)));
    }

    fn prepare_test_creature(insts: &[Instruction]) -> (Universe, Creature)
    {
        let mut univ = Universe::new();

        univ.generate_creature(&insts);
        let c = univ.creatures[0].clone();

        (univ, c)
    }

    #[test]
    fn test_instruction_nop()
    {
        let insts = [Nop0, Nop1, Nop0, Nop1];
        let (mut univ, mut c) = prepare_test_creature(&insts);

        univ.execute_creature_by_index(0, 4);
        c.core.ip = c.genome_region.size as u16;
        assert_eq!(univ.creatures[0].core, c.core);
    }

    #[test]
    fn test_instruction_or1()
    {
        let insts = [Nop1, Or1, Jmpb, Nop0, Zero];
        let (mut univ, mut c) = prepare_test_creature(&insts);

        univ.execute_creature_by_index(0, 2);
        c.core.cx = 1;
        c.core.ip = 2;
        assert_eq!(univ.creatures[0].core, c.core);

        univ.execute_creature_by_index(0, 2);
        c.core.cx = 0;
        c.core.ip = 2;
        assert_eq!(univ.creatures[0].core, c.core);
    }

    #[test]
    fn test_instruction_shl()
    {
        let insts = [Nop0, Or1, Shl, Jmpb, Nop1, Zero];
        let (mut univ, mut c) = prepare_test_creature(&insts);

        univ.execute_creature_by_index(0, 4);
        c.core.cx = 2;
        c.core.ip = 1;
        assert_eq!(univ.creatures[0].core, c.core);

        univ.execute_creature_by_index(0, 2);
        c.core.cx = 3 << 1;
        c.core.ip += 2;
        assert_eq!(univ.creatures[0].core, c.core);
    }

    #[test]
    fn test_instruction_zero()
    {
        let insts = [Nop1, Or1, Shl, Zero, Jmpb, Nop0, Zero];
        let (mut univ, mut c) = prepare_test_creature(&insts);

        univ.execute_creature_by_index(0, 3);
        c.core.ip += 3;
        c.core.cx = 2;
        assert_eq!(univ.creatures[0].core, c.core);

        univ.execute_creature_by_index(0, 2);
        c.core.ip = c.genome_region.addr as u16 + 1;
        c.core.cx = 0;
        assert_eq!(univ.creatures[0].core, c.core);
    }

    #[test]
    fn test_instruction_if_cz()
    {
        let insts = [Nop1, IfCz, Or1, Jmpb, Nop0, Zero];
        let (mut univ, mut c) = prepare_test_creature(&insts);

        univ.execute_creature_by_index(0, 2);
        c.core.ip += 2;
        assert_eq!(univ.creatures[0].core, c.core);

        univ.execute_creature_by_index(0, 1);
        c.core.ip += 1;
        c.core.cx = 1;
        assert_eq!(univ.creatures[0].core, c.core);

        univ.execute_creature_by_index(0, 1);
        c.core.ip = c.genome_region.addr as u16 + 1;
        c.core.cx = 1;
        assert_eq!(univ.creatures[0].core, c.core);

        univ.execute_creature_by_index(0, 1);
        c.core.ip += 2;
        c.core.cx = 1;
        assert_eq!(univ.creatures[0].core, c.core);
    }

    #[test]
    fn test_instruction_sub()
    {
        let insts = [Nop1, IncA, IncA, IncB, SubAb, SubAc, Jmpb, Nop0, Zero];
        let (mut univ, mut c) = prepare_test_creature(&insts);

        univ.execute_creature_by_index(0, 5);
        c.core.ip += 5;
        c.core.ax = 2;
        c.core.bx = 1;
        c.core.cx = 1;
        assert_eq!(univ.creatures[0].core, c.core);

        univ.execute_creature_by_index(0, 2);
        c.core.ip = c.genome_region.addr as u16 + 1;
        c.core.ax = 1;
        assert_eq!(univ.creatures[0].core, c.core);
    }

    #[test]
    fn test_instruction_inc_dec()
    {
        let insts = [IncA, IncB, IncA, IncB, IncC, IncC, IncC, DecC];
        let (mut univ, mut c) = prepare_test_creature(&insts);

        univ.execute_creature_by_index(0, 8);
        c.core.ax = 2;
        c.core.bx = 2;
        c.core.cx = 2;
        c.core.ip = c.genome_region.size as u16;
        assert_eq!(univ.creatures[0].core, c.core);
    }

    #[test]
    fn test_instruction_push()
    {
        let insts = [IncA, IncB, IncC, PushAx, PushBx, PushCx, PushDx];
        let (mut univ, mut c) = prepare_test_creature(&insts);

        univ.execute_creature_by_index(0, insts.len());
        c.core.ax = 1;
        c.core.bx = 1;
        c.core.cx = 1;
        c.core.stack[0] = 1;
        c.core.stack[1] = 1;
        c.core.stack[2] = 1;
        c.core.stack[3] = 0;
        c.core.sp = 4;
        c.core.ip = c.genome_region.size as u16;
        assert_eq!(univ.creatures[0].core, c.core);
    }

    #[test]
    fn test_instruction_pop()
    {
        let insts = [
            IncA,
            IncB,
            IncC,
            PushAx,
            PushBx,
            PushCx,
            PushDx,
            PopAx,
            PopBx,
            PopCx,
            PopDx,
        ];
        let (mut univ, mut c) = prepare_test_creature(&insts);

        univ.execute_creature_by_index(0, insts.len());
        c.core.stack[0] = 1;
        c.core.stack[1] = 1;
        c.core.stack[2] = 1;
        c.core.stack[3] = 0;
        c.core.sp = 0;
        c.core.ax = 0;
        c.core.bx = 1;
        c.core.cx = 1;
        c.core.dx = 1;
        c.core.ip = c.genome_region.size as u16;
        assert_eq!(univ.creatures[0].core, c.core);
    }

    #[test]
    fn test_instruction_jmp()
    {
        let insts = [
            Nop1,
            Nop0,
            Nop1,
            Jmp,
            Nop0,
            Zero,
            Zero,
            Zero,
            Nop1,
            Jmp,
            Nop0,
            Nop1,
            Zero,
            Nop1,
            Nop0,
            Jmpb,
            Nop0,
            Nop1,
            Nop0,
            Zero,
            ];
        let (mut univ, mut c) = prepare_test_creature(&insts);

        univ.execute_creature_by_index(0, 4);
        c.core.ip += 9;
        assert_eq!(univ.creatures[0].core, c.core);

        univ.execute_creature_by_index(0, 1);
        c.core.ip += 6;
        assert_eq!(univ.creatures[0].core, c.core);

        univ.execute_creature_by_index(0, 1);
        c.core.ip = c.genome_region.addr as u16 + 3;
        assert_eq!(univ.creatures[0].core, c.core);
    }

    #[test]
    fn test_instruction_call_ret()
    {
        let insts = [
            Zero,
            Call,
            Nop1,
            Nop0,
            Nop1,
            Jmp,
            Nop0,
            Nop1,
            Nop0,
            Ret,
            Zero,
        ];
        let (mut univ, mut c) = prepare_test_creature(&insts);

        univ.execute_creature_by_index(0, 2);
        c.core.stack[0] = 1;
        c.core.sp += 1;
        c.core.ip += 9;
        assert_eq!(univ.creatures[0].core, c.core);

        univ.execute_creature_by_index(0, 1);
        c.core.sp -= 1;
        c.core.ip = c.genome_region.addr as u16 + 2;
        assert_eq!(univ.creatures[0].core, c.core);
    }

    #[test]
    fn test_instruction_mov()
    {
        let insts = [IncC, MovCd, IncA, MovAb, IncA, MovIab];
        let (mut univ, mut c) = prepare_test_creature(&insts);

        univ.execute_creature_by_index(0, 2);
        c.core.ip += 2;
        c.core.cx = 1;
        c.core.dx = 1;
        assert_eq!(univ.creatures[0].core, c.core);

        univ.execute_creature_by_index(0, 2);
        c.core.ip += 2;
        c.core.ax = 1;
        c.core.bx = 1;
        assert_eq!(univ.creatures[0].core, c.core);

        univ.execute_creature_by_index(0, 1);
        c.core.ip += 1;
        c.core.ax = 2;
        assert_eq!(univ.creatures[0].core, c.core);

        univ.execute_creature_by_index(0, 1);
        assert_eq!(univ.genome_soup[c.core.ax as usize], univ.genome_soup[c.core.bx as usize]);
    }

    #[test]
    fn test_instruction_adr()
    {
        let insts = [
            Nop0,
            Nop1,
            Adrb,
            Nop1,
            Nop0,
            Adrf,
            Nop1,
            Zero,
            Zero,
            Zero,
            Zero,
            Zero,
            Nop0,
            Zero,
        ];
        let (mut univ, mut c) = prepare_test_creature(&insts);

        univ.execute_creature_by_index(0, 3);
        c.core.ip += 3;
        c.core.ax = c.genome_region.addr as u16 + 2;
        assert_eq!(univ.creatures[0].core, c.core);

        univ.execute_creature_by_index(0, 3);
        c.core.ip += 3;
        c.core.ax = c.genome_region.addr as u16 + 13;
        assert_eq!(univ.creatures[0].core, c.core);
    }

    #[test]
    fn test_instruction_mal_divide()
    {
        let insts = [IncC, IncC, IncC, Mal, Divide];
        let (mut univ, mut c) = prepare_test_creature(&insts);

        univ.execute_creature_by_index(0, 4);
        c.core.ip += 4;
        c.core.cx = 3;
        c.core.ax = univ.creatures[0].daughter.as_ref().unwrap().genome_region.addr as u16;
        assert_eq!(univ.creatures[0].core, c.core);

        univ.execute_creature_by_index(0, 1);
        c.core.ip += 1;
        assert_eq!(univ.creatures[0].core, c.core);
        assert_eq!(univ.creatures[0].daughter.is_none(), true);
        assert_eq!(univ.creatures[1].genome_region.addr, c.core.ax as usize);
    }
}

use std::collections::VecDeque;
use std::mem;
use std::ops::Range;

type Address             = u16;
type Register            = Address;
type InstructionWordSize = Address;

#[derive(Debug)]
struct Cpu {
    ax: Register,
    bx: Register,
    cx: Register,
    dx: Register,
    sp: Register,
    ip: Register,
    flags: u8,
    stack: [Register; 10],
}

impl Cpu {
    fn new() -> Cpu
    {
        Cpu {
            ax: 0,
            bx: 0,
            cx: 0,
            dx: 0,
            sp: 0,
            ip: 0,
            flags: 0,
            stack: [0; 10],
        }
    }

    fn push(&mut self, v: Register)
    {
        self.stack[self.sp as usize] = v;
        self.sp += 1;
    }

    fn pop(&mut self) -> Register
    {
        let v = self.stack[self.sp as usize];
        self.sp -= 1;
        v
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Instruction {
    Nop0   = 0x00, // no operation
    Nop1   = 0x01, // no operation
    Or1    = 0x02, // flip low order bit of cx, cx ^= 1
    Shl    = 0x03, // shift left cx register, cx <<= 1
    Zero   = 0x04, // set cx register to zero, cx = 0
    IfCz   = 0x05, // if cx == 0 execute next instruction
    SubAb  = 0x06, // subtract bx from ax, cx = ax - bx
    SubAc  = 0x07, // subtract cx from ax, ax = ax - cx
    IncA   = 0x08, // increment ax, ax = ax + 1
    IncB   = 0x09, // increment bx, bx = bx + 1
    DecC   = 0x0A, // decrement cx, cx = cx - 1
    IncC   = 0x0B, // increment cx, cx = cx + 1
    PushAx = 0x0C, // push ax on stack
    PushBx = 0x0D, // push bx on stack
    PushCx = 0x0E, // push cx on stack
    PushDx = 0x0F, // push dx on stack
    PopAx  = 0x10, // pop top of stack into ax
    PopBx  = 0x11, // pop top of stack into bx
    PopCx  = 0x12, // pop top of stack into cx
    PopDx  = 0x13, // pop top of stack into dx
    Jmp    = 0x14, // move ip to template
    Jmpb   = 0x15, // move ip backward to template
    Call   = 0x16, // call a procedure
    Ret    = 0x17, // return from a procedure
    MovCd  = 0x18, // move cx to dx, dx = cx
    MovAb  = 0x19, // move ax to bx, bx = ax
    MovIab = 0x1A, // move instruction at address in bx to address in ax
    Adr    = 0x1B, // address of nearest template to ax
    Adrb   = 0x1C, // search backward for template
    Adrf   = 0x1D, // search forward for template
    Mal    = 0x1E, // allocate memory for daughter cell
    Divide = 0x1F, // cell division
}

impl Instruction {
    fn is_nop(x: Instruction) -> bool
    {
        (x == Instruction::Nop0) || (x == Instruction::Nop1)
    }
}

struct MemoryRegion {
    addr: usize,
    size: usize,
}

impl MemoryRegion {
    fn new(addr: usize, size: usize) -> MemoryRegion
    {
        MemoryRegion {
            addr: addr,
            size: size,
        }
    }

    fn end_addr(&self) -> usize
    {
        self.addr + self.size
    }

    fn range(&self) -> Range<usize>
    {
        self.addr..self.end_addr()
    }
}

struct Creature {
    core: Cpu,
    genome: MemoryRegion,
}

impl Creature {
    fn new(g: MemoryRegion) -> Creature {
        Creature {
            core: Cpu::new(),
            genome: g,
        }
    }
}

const UNIVERSE_TOTAL_GENOME_CAPACITY: usize = 8 * 1024;

struct Universe {
    genome_soup: [Instruction; UNIVERSE_TOTAL_GENOME_CAPACITY],
    free_regions: VecDeque<MemoryRegion>,
    creatures: VecDeque<Creature>,
}

impl Universe {
    fn new() -> Universe
    {
        let soup = [Instruction::Nop0; UNIVERSE_TOTAL_GENOME_CAPACITY];
        let mut free_regions = VecDeque::new();
        free_regions.push_front(MemoryRegion::new(0, soup.len()));

        Universe {
            genome_soup: soup,
            free_regions: free_regions,
            creatures: VecDeque::new(),
        }
    }

    fn generate_creature(&mut self, instrunctions: &[Instruction])
    {
        match self.allocate_genome_pool(instrunctions.len()) {
            None => panic!("no memory"),
            Some(genome_region) => {
                let c = Creature::new(genome_region);
                self.write_to_genome_pool(&c.genome, instrunctions);
                self.creatures.push_front(c);
            }
        }
    }

    fn allocate_genome_pool(&mut self, request_size: usize) -> Option<MemoryRegion>
    {
        let mut allocated_genome = None;

        for v in self.free_regions.iter_mut() {
            if request_size <= v.size {
                let addr = v.addr;
                v.addr += request_size;
                v.size -= request_size;

                allocated_genome = Some(MemoryRegion::new(addr, request_size));
                break;
            }
        }

        allocated_genome
    }

    fn free_genome_pool(&mut self, _: MemoryRegion)
    {
        // TODO
    }

    fn read_from_genome_pool(&self, r: &MemoryRegion) -> &[Instruction]
    {
        &self.genome_soup[r.range()]
    }

    fn write_to_genome_pool(&mut self, r: &MemoryRegion, data: &[Instruction])
    {
        // TODO: check write privilege.
        let slice = &mut self.genome_soup[r.range()];
        for i in 0..data.len() {
            slice[i] = data[i];
        }
    }

    fn search_complement_template(&self, template: &[Instruction], begin_addr: usize) -> Option<usize>
    {
        let complement_template: Vec<Instruction> = template.clone().into_iter().map(|&x| {
            use Instruction::*;
            match x {
                Nop0 => Nop1,
                Nop1 => Nop0,
                _ => panic!("invalid instrunction"),
            }
        }).collect();

        const SEARCH_LIMIT: usize = 1000;
        let r = MemoryRegion::new(begin_addr, SEARCH_LIMIT);
        let search_region = &self.genome_soup[r.range()];

        search_region.windows(complement_template.len()).position(|window| {
            let mut cnt = 0;
            for i in window.iter() {
                if complement_template[cnt] != *i {
                    return false;
                }
                cnt += 1;
            }
            true
        })
    }

    fn search_template_begin(&self, addr: usize, size: usize, is_forward: bool) -> Option<usize>
    {
        let range =
            match is_forward {
                true  => addr..(addr + size),
                false => (addr - size + 1)..(addr + 1),
            };

        for (i, &x) in self.genome_soup[range].iter().enumerate() {
            if Instruction::is_nop(x) {
                return Some(i);
            }
        }

        None
    }

    fn search_template(&self, r: &MemoryRegion, is_forward: bool) -> Option<&[Instruction]>
    {
        let begin_index = self.genome_soup[r.range()]
            .iter()
            .position(|&x| Instruction::is_nop(x) == true);
        let begin_addr = match begin_index {
            Some(i) => r.addr + i,
            None => return None,
        };

        if begin_addr == r.end_addr() {
            return Some(&self.genome_soup[begin_addr..begin_addr]);
        }

        let end_index = self.genome_soup[(begin_addr + 1)..r.end_addr()]
            .iter()
            .position(|&x| Instruction::is_nop(x) == false);

        let end_addr =
            match end_index {
                Some(i) => begin_addr + i + 1,
                None => r.end_addr(),
            };

        Some(&self.genome_soup[begin_addr..end_addr])
    }

    fn extract_template(&self, begin_addr: usize) -> Option<Vec<Instruction>>
    {
        if Instruction::is_nop(self.genome_soup[begin_addr]) == false {
            return None;
        }

        let search_region = MemoryRegion::new(begin_addr, self.genome_soup.len());
        let mut extracted_template = Vec::new();
        let slice = &self.genome_soup[search_region.range()];
        for i in slice {
            if Instruction::is_nop(*i) == false {
                break;
            }

            extracted_template.push(*i);
        }

        Some(extracted_template)
    }

    fn execute(&self, cpu: &mut Cpu, ins: Instruction)
    {
        use Instruction::*;
        let (ax, bx, cx, dx) = (cpu.ax, cpu.bx, cpu.cx, cpu.dx);
        match ins {
            Nop0   => {},
            Nop1   => {},
            Or1    => cpu.cx = cx ^ 1,
            Shl    => cpu.cx = cx << 1,
            Zero   => cpu.cx = 0,
            IfCz   => {
                if cx != 0 {
                    // Skip the next instruction.
                    cpu.ip += 1;
                }
            },
            SubAb  => cpu.cx = ax - bx,
            SubAc  => cpu.ax = ax - cx,
            IncA   => cpu.ax = ax + 1,
            IncB   => cpu.bx = bx + 1,
            DecC   => cpu.cx = cx - 1,
            IncC   => cpu.cx = cx + 1,
            PushAx => cpu.push(ax),
            PushBx => cpu.push(bx),
            PushCx => cpu.push(cx),
            PushDx => cpu.push(dx),
            PopAx  => cpu.ax = cpu.pop(),
            PopBx  => cpu.bx = cpu.pop(),
            PopCx  => cpu.cx = cpu.pop(),
            PopDx  => cpu.dx = cpu.pop(),
            Jmp => {
                //TODO
            },
            Jmpb => {
                //TODO
            },
            Call => {
            },
            MovCd => {
                cpu.dx = cpu.cx
            },
            MovAb => {
                cpu.bx = cpu.ax
            },
            MovIab => {
                // TODO
            },
            Adr => {
                // TODO
            },
            Adrb => {
                // TODO
            },
            Adrf => {
                // TODO
            },
            Mal => {
                // TODO
            },
            Divide => {
                // TODO
            },
            _  => panic!("No implementation")
        }
    }

    fn fetch(&self, creature: &Creature) -> Instruction
    {
        let ip = creature.core.ip as usize;
        self.read_from_genome_pool(&MemoryRegion::new(ip, ip + 1))[0]
    }

    fn give_cpu_time(&self, creature: &mut Creature)
    {
        const SLICED_TIME: usize = 10;

        for _ in 0..SLICED_TIME {
            // fetch
            let ins = self.fetch(creature);

            let cpu = &mut creature.core;
            self.execute(cpu, ins);
            cpu.ip += 1;

            if (cpu.ip as usize) == creature.genome.size {
                cpu.ip = 0;
            }

            println!("CPU: {:?}", cpu);
        }
    }

    fn works(&mut self)
    {
        let mut cs = mem::replace(&mut self.creatures, VecDeque::new());
        for i in cs.iter_mut() {
            self.give_cpu_time(i);
        }
        self.creatures = cs;
    }
}

fn main() {
    use Instruction::*;

    let mut univ = Universe::new();
    let insts = [
        Nop1,
        Nop1,
        Nop1,
        Nop1,
        Zero,
        Or1,
        Shl,
        Shl,
    ];
    univ.generate_creature(&insts);
    univ.works();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_template()
    {
        use Instruction::*;

        let mut univ = Universe::new();
        let insts = [
            Nop1, Nop1, Nop1, Nop1,
            Zero,  Or1,  Shl,  Shl,
            Nop1, Nop1, Nop0, Nop0,
            Nop0, Nop1, Nop1, Nop1,
            Zero, Zero, Zero, Zero,
            Nop0, Nop1, Nop1, Nop1,
            Zero, Zero, Zero, Zero,
            Nop0, Nop1, Nop1, Nop1,
        ];
        univ.write_to_genome_pool(&MemoryRegion::new(0, insts.len()), &insts);

        assert_eq!(univ.search_template(&MemoryRegion::new(0, insts.len()), true), Some(&insts[0..4]));
        assert_eq!(univ.search_template(&MemoryRegion::new(4, insts.len() - 4), true), Some(&insts[8..16]));
        assert_eq!(univ.search_template(&MemoryRegion::new(16, 8), true), Some(&insts[20..24]));
        assert_eq!(univ.search_template(&MemoryRegion::new(24, 4), true), None);
        assert_eq!(univ.search_template(&MemoryRegion::new(28, 4), true), Some(&insts[28..32]));
        assert_eq!(univ.search_template(&MemoryRegion::new(0, 1), true), Some(&insts[0..1]));
    }

    #[test]
    fn test_extract_template()
    {
        use Instruction::*;

        let mut univ = Universe::new();
        let insts = [
            Nop1, Nop1, Nop1, Nop1,
            Zero,  Or1,  Shl,  Shl,
            Nop1, Nop1, Nop0, Nop0,
            Nop0, Nop1, Nop1, Nop1,
            Zero, Zero, Zero, Zero,
            Nop0, Nop1, Nop1, Nop1,
        ];
        univ.write_to_genome_pool(&MemoryRegion::new(0, insts.len()), &insts);

        assert_eq!(univ.extract_template(0), Some(vec![Nop1, Nop1, Nop1, Nop1]));
        // assert_eq!(univ.extract_template(4), Some(vec![Nop1, Nop1, Nop0, Nop0, Nop0, Nop1, Nop1, Nop1]));
    }
}

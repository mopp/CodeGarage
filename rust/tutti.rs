use std::collections::VecDeque;
use std::mem;
use std::ops::Range;
use std::fmt;

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

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s1 = format!("ax = 0x{:02X}, bx = 0x{:02X}, cx = 0x{:02X}, dx = 0x{:02X}, ", self.ax, self.bx, self.cx, self.dx);
        let s2 = format!("ip = 0x{:02X}, sp = 0x{:02X}, ", self.ip, self.sp);
        let s3 = format!("flags = 0x{:02X}", self.flags);

        write!(f, "{}{}{}", s1, s2, s3)
    }
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

    fn search_template(&self, addr: usize, size: usize, is_search_begin: bool, is_forward: bool) -> Option<usize>
    {
        let v: Vec<(usize, &Instruction)> =
            match is_forward {
                true  => self.genome_soup[addr..(addr + size)].iter().enumerate().collect(),
                false => self.genome_soup[(addr - size + 1)..(addr + 1)].iter().rev().enumerate().collect(),
            };

        for (i, &x) in v {
            if Instruction::is_nop(x) != is_search_begin {
                continue;
            }

            return
                match is_search_begin {
                    true  => Some(i),
                    false => {
                        match i {
                            0 => Some(0),
                            _ => Some(i - 1)
                        }
                    }
                };
        }

        None
    }

    fn search_template_begin_forward(&self, addr: usize, size: usize) -> Option<usize>
    {
        self.search_template(addr, size, true, true)
    }

    fn search_template_end_forward(&self, addr: usize, size: usize) -> Option<usize>
    {
        self.search_template(addr, size, false, true)
    }

    fn search_template_begin_backward(&self, addr: usize, size: usize) -> Option<usize>
    {
        self.search_template(addr, size, true, false)
    }

    fn search_template_end_backward(&self, addr: usize, size: usize) -> Option<usize>
    {
        self.search_template(addr, size, false, false)
    }

    fn search_complement_template(&self, template: &[Instruction], begin_addr: usize, is_forward: bool) -> Option<usize>
    {
        let complement_template: Vec<Instruction> = template
            .clone()
            .into_iter()
            .map(|&x| {
                use Instruction::*;
                match x {
                    Nop0 => Nop1,
                    Nop1 => Nop0,
                    _    => panic!("invalid instrunction"),
                }
            })
        .collect();

        const SEARCH_LIMIT: usize = 32;
        let (begin_addr, end_addr) =
            match is_forward {
                true => {
                    let t = begin_addr + SEARCH_LIMIT;
                    let l = self.genome_soup.len();
                    let e =
                        if t < l {
                            t
                        } else {
                            l
                        };
                    (begin_addr, e)
                },
                false => {
                    let b =
                        if SEARCH_LIMIT <= begin_addr {
                            begin_addr - SEARCH_LIMIT
                        } else {
                            0
                        };
                    (b, begin_addr + 1)
                }
            };

        let f = |window| complement_template.as_slice() == window;
        let slice = &self.genome_soup[begin_addr..end_addr];
        let len = complement_template.len();

        match is_forward {
            true  => slice.windows(len).position(f),
            false => slice.windows(len).rposition(f).map(|x| x - len),
        }
    }

    fn extract_template(&self, addr: usize, size: usize) -> Option<&[Instruction]>
    {
        let target_region = self.read_from_genome_pool(&MemoryRegion::new(addr, size));
        if Instruction::is_nop(target_region[0]) == false {
            return None;
        }

        // Find the tail of the template.
        let head_index = 0;
        let tail_index = target_region
            .iter()
            .position(|&x| Instruction::is_nop(x) == false)
            .unwrap_or(target_region.len() - 1);

        let range =
            if head_index == tail_index {
                head_index..(head_index + 1)
            } else {
                head_index..tail_index
            };

        Some(&target_region[range])
    }


    fn execute(&mut self, cpu: &mut Cpu, ins: Instruction)
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
            Jmp | Jmpb | Call => {
                let addr = cpu.ip as usize + 1;
                let template =
                    match self.extract_template(addr, 8) {
                        Some(v) => v,
                        None => panic!("Nop0/Nop1 have to be placed after Jmp/Jmpb instruction."),
                    };

                if ins == Call {
                    let ip = cpu.ip;
                    cpu.push(ip + 1);
                }

                let is_forward = ins == Jmp;
                match self.search_complement_template(template, addr, is_forward) {
                    None => {
                        // jmp instruction is ignored.
                    },
                    Some(addr_diff) => {
                        let len = template.len();
                        cpu.ip =
                            if is_forward {
                                (addr + len - 1 + addr_diff) as u16
                            } else {
                                (addr + len - 1 - addr_diff) as u16
                            };
                    }
                }
            },
            Ret => {
                cpu.ip = cpu.pop();
            },
            MovCd => {
                cpu.dx = cpu.cx
            },
            MovAb => {
                cpu.bx = cpu.ax
            },
            MovIab => {
                let ins = self.read_from_genome_pool(&MemoryRegion::new(bx as usize, bx as usize+ 1))[0];
                self.write_to_genome_pool(&MemoryRegion::new(ax as usize, ax as usize+ 1), &[ins]);
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
            }
        }
    }

    fn fetch(&self, creature: &Creature) -> Instruction
    {
        let ip = creature.core.ip as usize;
        self.read_from_genome_pool(&MemoryRegion::new(ip, ip + 1))[0]
    }

    fn one_instruction_cycle(&mut self, creature: &mut Creature)
    {
        // Fetch
        let ins = self.fetch(creature);
        println!("Fetch: {:?}", ins);

        // Execute
        let cpu = &mut creature.core;
        self.execute(cpu, ins);
        println!("Execute: {}", cpu);

        cpu.ip += 1;
        if (cpu.ip as usize) == creature.genome.size {
            cpu.ip = 0;
        }
    }

    fn works(&mut self)
    {
        let mut cs = mem::replace(&mut self.creatures, VecDeque::new());
        for c in cs.iter_mut() {
            const SLICED_TIME: usize = 10;

            for _ in 0..SLICED_TIME {
                self.one_instruction_cycle(c);
            }
        }
        self.creatures = cs;
    }
}

fn main() {
    use Instruction::*;

    let mut univ = Universe::new();
    let insts = [
        Jmp,
        Nop1,
        Nop1,
        Nop1,
        Nop1,
        Zero,
        Or1,
        Zero,
        Shl,
        IncA,
        Nop0,
        Nop0,
        Nop0,
        Nop0,
        IncB,
        Jmpb,
        Nop1,
        Nop1,
        Nop1,
        Nop1,
        Zero,
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

        assert_eq!(univ.search_template(0,  10, true, true),  Some(0));
        assert_eq!(univ.search_template(6,  10, true, true),  Some(2));
        assert_eq!(univ.search_template(6,   2, true, true),  None);
        assert_eq!(univ.search_template(7,   7, true, false), Some(4));
        assert_eq!(univ.search_template(31, 10, true, false), Some(0));
        assert_eq!(univ.search_template(27, 10, true, false), Some(4));

        assert_eq!(univ.search_template(7,   6, false, true),  Some(0));
        assert_eq!(univ.search_template(6,  10, false, true),  Some(0));
        assert_eq!(univ.search_template(8,  10, false, true),  Some(7));
        assert_eq!(univ.search_template(7,   7, false, false), Some(0));
        assert_eq!(univ.search_template(31, 10, false, false), Some(3));
        assert_eq!(univ.search_template(27, 10, false, false), Some(0));
        assert_eq!(univ.search_template(31,  3, false, false), None);
    }

    #[test]
    fn test_extract_template()
    {
        use Instruction::*;

        let mut univ = Universe::new();
        let insts = [
            Jmp,
            Nop1,
            Nop1,
            Nop1,
            Nop1,
        ];

        univ.write_to_genome_pool(&MemoryRegion::new(0, insts.len()), &insts);
        assert_eq!(univ.extract_template(1, insts.len()), Some(vec![Nop1, Nop1, Nop1, Nop1].as_slice()));

        let insts = [
            Nop1,
        ];
        univ.write_to_genome_pool(&MemoryRegion::new(0, insts.len()), &insts);
        assert_eq!(univ.extract_template(0, insts.len()), Some(vec![Nop1].as_slice()));

        let insts = [
            Nop0,
            Nop0,
            Nop0,
            Nop0,
            Zero,
        ];
        univ.write_to_genome_pool(&MemoryRegion::new(0, insts.len()), &insts);
        assert_eq!(univ.extract_template(0, insts.len()), Some(vec![Nop0, Nop0, Nop0, Nop0].as_slice()));
    }
}

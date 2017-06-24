use std::collections::VecDeque;
use std::fmt;
use std::mem;
use std::ops::Range;

type Register = u16;

#[derive(Debug, Clone, PartialEq, Eq)]
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
        if self.sp == 0 {
            panic!("Stack is empty.");
        }

        self.sp -= 1;
        self.stack[self.sp as usize]
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

#[derive(Clone)]
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

#[derive(Clone)]
struct Creature {
    core: Cpu,
    genome_region: MemoryRegion,
}

impl Creature {
    fn new(g: MemoryRegion) -> Creature
    {
        let mut core = Cpu::new();
        core.ip = g.addr as u16;
        Creature {
            core: core,
            genome_region: g,
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

    fn generate_creature(&mut self, instructions: &[Instruction])
    {
        match self.allocate_genome_pool(instructions.len()) {
            None => panic!("no memory"),
            Some(genome_region) => {
                let c = Creature::new(genome_region);
                self.write_instructions(c.genome_region.addr, instructions);
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
                template
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
                },
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
        debug_assert_eq!(Instruction::is_nop(self.genome_soup[addr]), true);

        let target_region = &self.genome_soup[addr..(self.genome_soup.len() - 1)];
        target_region
            .iter()
            .position(|&x| Instruction::is_nop(x) == false)
            .or(Some(target_region.len() - 1))
            .map(|tail_index| &target_region[0..tail_index])
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
                if ins == Call {
                    let ip = cpu.ip;
                    cpu.push(ip);
                }

                match self.search_complement_addr(cpu.ip as usize + 1, ins == Jmp || ins == Call) {
                    None               => {},
                    Some((addr, size)) => cpu.ip = (addr + size - 1) as u16,
                }
            },
            Ret   => cpu.ip = cpu.pop(),
            MovCd => cpu.dx = cx,
            MovAb => cpu.bx = ax,
            MovIab => {
                self.genome_soup[ax as usize] = self.genome_soup[bx as usize];
            },
            Adr => {
                // TODO
            },
            Adrf => {
                // TODO
            },
            Adrb => {
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
        self.genome_soup[creature.core.ip as usize]
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
        if (cpu.ip as usize) == creature.genome_region.size {
            cpu.ip = 0;
        }
    }

    fn execute_all_creatures(&mut self, sliced_time: usize)
    {
        let mut cs = mem::replace(&mut self.creatures, VecDeque::new());
        for c in cs.iter_mut() {
            for _ in 0..sliced_time {
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
    univ.execute_all_creatures(10);
}


#[cfg(test)]
mod tests {
    use super::*;
    use Instruction::*;

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
        let insts = [ Nop0, Nop1, Nop0, Nop1 ];
        let (mut univ, mut c) = prepare_test_creature(&insts);

        univ.execute_all_creatures(4);
        assert_eq!(univ.creatures[0].core, c.core);

        univ.execute_all_creatures(4);
        assert_eq!(univ.creatures[0].core, c.core);
    }

    #[test]
    fn test_instruction_or1()
    {
        let insts = [ Or1 ];
        let (mut univ, mut c) = prepare_test_creature(&insts);

        univ.execute_all_creatures(1);
        c.core.cx = 1;
        assert_eq!(univ.creatures[0].core, c.core);

        univ.execute_all_creatures(1);
        c.core.cx = 0;
        assert_eq!(univ.creatures[0].core, c.core);
    }

    #[test]
    fn test_instruction_shl()
    {
        let insts = [ Or1, Shl ];
        let (mut univ, mut c) = prepare_test_creature(&insts);

        univ.execute_all_creatures(2);
        c.core.cx = 2;
        assert_eq!(univ.creatures[0].core, c.core);

        univ.execute_all_creatures(2);
        c.core.cx = 3 << 1;
        assert_eq!(univ.creatures[0].core, c.core);
    }

    #[test]
    fn test_instruction_zero()
    {
        let insts = [ Or1, Shl, Zero ];
        let (mut univ, mut c) = prepare_test_creature(&insts);

        univ.execute_all_creatures(2);
        c.core.ip += 2;
        c.core.cx = 2;
        assert_eq!(univ.creatures[0].core, c.core);

        univ.execute_all_creatures(1);
        c.core.ip = c.genome_region.addr as u16;
        c.core.cx = 0;
        assert_eq!(univ.creatures[0].core, c.core);
    }

    #[test]
    fn test_instruction_if_cz()
    {
        let insts = [ IfCz, Or1, Nop0 ];
        let (mut univ, mut c) = prepare_test_creature(&insts);

        univ.execute_all_creatures(1);
        c.core.ip += 1;
        assert_eq!(univ.creatures[0].core, c.core);

        univ.execute_all_creatures(1);
        c.core.ip += 1;
        c.core.cx = 1;
        assert_eq!(univ.creatures[0].core, c.core);

        univ.execute_all_creatures(1);
        c.core.ip = c.genome_region.addr as u16;
        c.core.cx = 1;
        assert_eq!(univ.creatures[0].core, c.core);

        univ.execute_all_creatures(1);
        c.core.ip += 2;
        c.core.cx = 1;
        assert_eq!(univ.creatures[0].core, c.core);
    }

    #[test]
    fn test_instruction_sub()
    {
        let insts = [ IncA, IncA, IncB, SubAb, SubAc];
        let (mut univ, mut c) = prepare_test_creature(&insts);

        univ.execute_all_creatures(4);
        c.core.ip += 4;
        c.core.ax = 2;
        c.core.bx = 1;
        c.core.cx = 1;
        assert_eq!(univ.creatures[0].core, c.core);

        univ.execute_all_creatures(1);
        c.core.ip = c.genome_region.addr as u16;
        c.core.ax = 1;
        assert_eq!(univ.creatures[0].core, c.core);
    }

    #[test]
    fn test_instruction_inc_dec()
    {
        let insts = [ IncA, IncB, IncA, IncB, IncC, IncC, IncC, DecC];
        let (mut univ, mut c) = prepare_test_creature(&insts);

        univ.execute_all_creatures(8);
        c.core.ax = 2;
        c.core.bx = 2;
        c.core.cx = 2;
        assert_eq!(univ.creatures[0].core, c.core);
    }

    #[test]
    fn test_instruction_push()
    {
        let insts = [IncA, IncB, IncC, PushAx, PushBx, PushCx, PushDx];
        let (mut univ, mut c) = prepare_test_creature(&insts);

        univ.execute_all_creatures(insts.len());
        c.core.ax       = 1;
        c.core.bx       = 1;
        c.core.cx       = 1;
        c.core.stack[0] = 1;
        c.core.stack[1] = 1;
        c.core.stack[2] = 1;
        c.core.stack[3] = 0;
        c.core.sp       = 4;
        assert_eq!(univ.creatures[0].core, c.core);
    }

    #[test]
    fn test_instruction_pop()
    {
        let insts = [IncA, IncB, IncC, PushAx, PushBx, PushCx, PushDx, PopAx, PopBx, PopCx, PopDx];
        let (mut univ, mut c) = prepare_test_creature(&insts);

        univ.execute_all_creatures(insts.len());
        c.core.stack[0] = 1;
        c.core.stack[1] = 1;
        c.core.stack[2] = 1;
        c.core.stack[3] = 0;
        c.core.sp = 0;
        c.core.ax = 0;
        c.core.bx = 1;
        c.core.cx = 1;
        c.core.dx = 1;
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

        univ.execute_all_creatures(4);
        c.core.ip += 9;
        assert_eq!(univ.creatures[0].core, c.core);

        univ.execute_all_creatures(1);
        c.core.ip += 6;
        assert_eq!(univ.creatures[0].core, c.core);

        univ.execute_all_creatures(1);
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

        univ.execute_all_creatures(2);
        c.core.stack[0] = 1;
        c.core.sp += 1;
        c.core.ip += 9;
        assert_eq!(univ.creatures[0].core, c.core);

        univ.execute_all_creatures(1);
        c.core.sp -= 1;
        c.core.ip = c.genome_region.addr as u16 + 2;
        assert_eq!(univ.creatures[0].core, c.core);
    }

    #[test]
    fn test_instruction_mov()
    {
        let insts = [
            IncC,
            MovCd,
            IncA,
            MovAb,
            IncA,
            MovIab,
        ];
        let (mut univ, mut c) = prepare_test_creature(&insts);

        univ.execute_all_creatures(2);
        c.core.ip += 2;
        c.core.cx = 1;
        c.core.dx = 1;
        assert_eq!(univ.creatures[0].core, c.core);

        univ.execute_all_creatures(2);
        c.core.ip += 2;
        c.core.ax = 1;
        c.core.bx = 1;
        assert_eq!(univ.creatures[0].core, c.core);

        univ.execute_all_creatures(1);
        c.core.ip += 1;
        c.core.ax = 2;
        assert_eq!(univ.creatures[0].core, c.core);

        univ.execute_all_creatures(1);
        assert_eq!(univ.genome_soup[c.core.ax as usize], univ.genome_soup[c.core.bx as usize]);
    }
}

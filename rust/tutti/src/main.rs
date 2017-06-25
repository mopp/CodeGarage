extern crate rand;

mod cpu;
mod creature;
mod instruction;
mod memory_region;
mod universe;

use instruction::Instruction;
use universe::Universe;


fn main() {
    use Instruction::*;

    let mut univ = Universe::new();
    univ.enable_random_mutate();

    let insts = [
        Nop1,
        Nop1,
        Nop1,
        Nop1,
        Zero,
        Or1,
        Shl,
        Shl,
        MovCd,
        Adrb,
        Nop0,
        Nop0,
        Nop0,
        Nop0,
        SubAc,
        MovAb,
        Adrf,
        Nop0,
        Nop0,
        Nop0,
        Nop1,
        IncA,
        SubAb,
        Nop1,
        Nop1,
        Nop0,
        Nop1,
        Mal,
        Call,
        Nop0,
        Nop0,
        Nop1,
        Nop1,
        Divide,
        Jmpb,
        Nop0,
        Nop0,
        Nop1,
        Nop0,
        IfCz,
        Nop1,
        Nop1,
        Nop0,
        Nop0,
        PushAx,
        PushBx,
        PushCx,
        Nop1,
        Nop0,
        Nop1,
        Nop0,
        MovIab,
        DecC,
        IfCz,
        Jmp,
        Nop0,
        Nop1,
        Nop0,
        Nop0,
        IncA,
        IncB,
        Jmp,
        Nop0,
        Nop1,
        Nop0,
        Nop1,
        IfCz,
        Nop1,
        Nop0,
        Nop1,
        Nop1,
        PopCx,
        PopBx,
        PopAx,
        Ret,
        Nop1,
        Nop1,
        Nop1,
        Nop0,
        IfCz,
        ];
    univ.generate_creature(&insts);

    loop {
        univ.execute_all_creatures(1.2);
        univ.wakeup_reaper_if_genome_usage_over(0.8);
        println!("# of creatures: {}", univ.count_creatures());
        println!("Genome usage rate: {}", univ.compute_genome_soup_used_rate());
    }
}

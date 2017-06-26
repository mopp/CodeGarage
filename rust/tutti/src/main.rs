extern crate rand;

mod cpu;
mod creature;
mod instruction;
mod memory_region;
mod universe;
mod gene_bank;

use instruction::Instruction;
use universe::Universe;


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
        Jmpb,
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

    univ.enable_random_mutate();
    univ.randomize_mutate_thresholds();
    loop {
        univ.execute_all_creatures(1.2);
        univ.wakeup_reaper_if_genome_usage_over(0.8);
        println!("==========");
        println!("# of creatures: {}", univ.count_creatures());
        println!("Genome usage rate: {}", univ.compute_genome_soup_used_rate());
        println!("Bank Info\n{}", univ.gene_bank());
        println!("==========");
    }
}

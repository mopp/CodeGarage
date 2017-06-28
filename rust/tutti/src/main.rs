#[macro_use]
extern crate chan;
extern crate chan_signal;
extern crate rand;
extern crate chrono;

mod cpu;
mod creature;
mod gene_bank;
mod instruction;
mod memory_region;
mod universe;

use chan_signal::Signal;
use instruction::Instruction;
use std::fs::File;
use std::io::prelude::*;
use std::thread;
use universe::Universe;
use chrono::Local;


fn main() {
    let mut univ = Universe::new();

    let signal = chan_signal::notify(&[Signal::INT, Signal::TERM]);
    let (sdone, rdone) = chan::sync(0);
    thread::spawn(move || run(sdone));

    use Instruction::*;

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
        println!("Genome used size: {}", universe::UNIVERSE_TOTAL_GENOME_CAPACITY - univ.compute_genome_soup_free_size());
        println!("Genome free size: {}", univ.compute_genome_soup_free_size());

        assert_eq!(universe::UNIVERSE_TOTAL_GENOME_CAPACITY, univ.compute_genome_soup_free_size() + univ.compute_genome_soup_used_size());

        println!("Genome usage rate: {}", univ.compute_genome_soup_used_rate());
        println!("Genome free rate:  {}", univ.compute_genome_soup_free_rate());
        println!("# of creatures: {}", univ.count_creatures());
        println!("Bank Info\n{}", univ.gene_bank());
        println!("# of free regions {:?}", univ.free_regions.len());
        // println!("{:?}", univ.free_regions);
        println!("==========");

        if univ.count_creatures() == 0 {
            panic!("NO CREATURES !");
        }

        chan_select! {
            signal.recv() -> _ => {
                let filename = Local::now().format("%Y%m%d%H%M%S.txt").to_string();

                println!("\n\nDUMP ALL GENOMEs to {}", filename);

                let mut file = File::create(filename).unwrap();
                file.write_fmt(format_args!("{}", univ.gene_bank.dump_all_recorded_genoms())).unwrap();
                break;
            },
            rdone.recv() => { }
        }
    }
}

fn run(_sdone: chan::Sender<()>)
{
}

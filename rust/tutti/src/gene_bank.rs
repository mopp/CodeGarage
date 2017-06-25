use instruction::Instruction;
use std::collections::HashMap;
use std::fmt;


#[derive(Debug, PartialEq, Eq, Clone)]
struct GenomeRecord {
    genome: Vec<Instruction>,
    genome_type: usize,
    mother_info: Option<Box<GenomeRecord>>,
}


impl fmt::Display for GenomeRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{:x}", self.genome.len(), self.genome_type)
    }
}


impl GenomeRecord {
    fn new(genome: Vec<Instruction>, mother: Option<Box<GenomeRecord>>) -> GenomeRecord
    {
        let t =
            if let Some(m) = mother.as_ref() {
                m.genome_type + 1
            } else {
                0
            };

        GenomeRecord {
            genome: genome,
            genome_type: t,
            mother_info: mother,
        }
    }
}


pub struct GeneBank {
    records: Vec<GenomeRecord>,
    alive_count_map: HashMap<String, usize>,
    dead_count_map: HashMap<String, usize>,
}


impl fmt::Display for GeneBank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.records
            .iter()
            .map(|r| {
                let key           = r.to_string();
                let default_value = 0;
                let alive_count   = self.alive_count_map.get(&key).unwrap_or(&default_value);
                let dead_count    = self.dead_count_map.get(&key).unwrap_or(&default_value);
                [
                    format!("GenoType: {}", key),
                    format!("  # of borns : {:>}", alive_count + dead_count),
                    format!("  # of alives: {:>}", alive_count),
                    format!("  # of deads : {:>}", dead_count),
                ].join("\n")
            })
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", s)
    }
}


impl GeneBank {
    pub fn new() -> GeneBank
    {
        GeneBank {
            records: Vec::new(),
            alive_count_map: HashMap::new(),
            dead_count_map: HashMap::new(),
        }
    }

    pub fn register_genome(&mut self, genome: &Vec<Instruction>, mother: Option<Vec<Instruction>>)
    {
        let genome = (*genome).clone();
        if mother.is_none() {
            self.records.push(GenomeRecord::new(genome, None));
            return;
        }

        match (self.find_genome_record(&genome), self.find_genome_record(&mother.unwrap())) {
            (None, Some(i)) => {
                let m = self.records[i].clone();
                self.records.push(GenomeRecord::new(genome, Some(Box::new(m))));
            },
            _ => {},
        }
    }

    fn find_genome_record(&self, target: &Vec<Instruction>) -> Option<usize>
    {
        self.records
            .iter()
            .position(|r| r.genome == *target)
    }

    pub fn count_up_alive_genome(&mut self, genome: &Vec<Instruction>)
    {
        match self.find_genome_record(genome) {
            None    => panic!("You have to register the genome !"),
            Some(i) => {
                let key = self.records[i].to_string();
                let count = self.alive_count_map.remove(&key).unwrap_or(0);
                self.alive_count_map.insert(key.clone(), count + 1);
            }
        }
    }

    pub fn count_up_dead_genome(&mut self, genome: &Vec<Instruction>)
    {
        match self.find_genome_record(genome) {
            None    => panic!("You have to register the genome !"),
            Some(i) => {
                let key = self.records[i].to_string();
                let count = self.dead_count_map.remove(&key).unwrap_or(0);
                self.dead_count_map.insert(key.clone(), count + 1);

                let m = &mut self.alive_count_map;
                if m.contains_key(&key) {
                    let count = m.remove(&key).unwrap();
                    m.insert(key, count - 1);
                }
            }
        }
    }
}

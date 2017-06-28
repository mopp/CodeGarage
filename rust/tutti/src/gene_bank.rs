use instruction::Instruction;
use std::collections::HashMap;
use std::fmt;
use std::collections::HashSet;


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
        let mut keys = HashSet::new();
        for i in self.records.iter() {
            let key = i.to_string();
            match self.alive_count_map.get(&key) {
                Some(c) if *c != 0 => { keys.insert(key); },
                _                  => { },
            }
        }

        let mut v = Vec::new();
        v.extend(keys.into_iter());
        v.sort();

        let s = v
            .iter()
            .map(|key| {
                let key = key.to_string();
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

    pub fn register_genome(&mut self, genome: &Vec<Instruction>, mother: Option<&String>) -> Option<String>
    {
        let genome = (*genome).clone();
        if mother.is_none() && self.find_genome_record(&genome).is_none() {
            let r = GenomeRecord::new(genome, None);
            let tag = r.to_string();
            self.records.push(r);

            return Some(tag);
        }

        match (self.find_genome_record(&genome), self.find_genome_record_by_type(mother)) {
            (None, Some(mother)) => {
                let r = GenomeRecord::new(genome, Some(Box::new(mother)));
                let tag = r.to_string();
                self.records.push(r);
                Some(tag)
            },
            (Some(index), _) => {
                Some(self.records[index].to_string())
            }
            (None, None) => panic!("None, None - {:?}", mother),
        }
    }

    fn find_genome_record(&self, target: &Vec<Instruction>) -> Option<usize>
    {
        self.records
            .iter()
            .position(|r| r.genome == *target)
    }

    fn find_genome_record_by_type(&self, target: Option<&String>) -> Option<GenomeRecord>
    {
        target
            .and_then(|t| {
                self.records
                    .iter()
                    .find(|x| x.to_string() == *t)
            })
            .and_then(|x| Some(x.clone()))
    }

    pub fn count_up_alive_genome(&mut self, geno_type: &String)
    {
        let key = (*geno_type).clone();
        let count = self.alive_count_map.remove(&key).unwrap_or(0);
        self.alive_count_map.insert(key, count + 1);
    }

    pub fn count_up_dead_genome(&mut self, geno_type: &String)
    {
        let key = (*geno_type).clone();
        let count = self.dead_count_map.remove(&key).unwrap_or(0);
        self.dead_count_map.insert(key.clone(), count + 1);

        let m = &mut self.alive_count_map;
        if m.contains_key(&key) {
            let count = m.remove(&key).unwrap();
            m.insert(key, count - 1);
        }
    }

    pub fn dump_all_recorded_genoms(&self) -> String
    {
        let mut keys =
            self.records
            .iter()
            .fold(HashSet::new(), |mut acc, ref x| {
                acc.insert(x.to_string());
                acc
            })
            .into_iter()
            .collect::<Vec<String>>();
        keys.sort();

        keys
            .into_iter()
            .map(|key| {
                let r = self.find_genome_record_by_type(Some(&key)).unwrap();
                let default_value = 0;
                let alive_count   = self.alive_count_map.get(&key).unwrap_or(&default_value);
                let dead_count    = self.dead_count_map.get(&key).unwrap_or(&default_value);
                [
                    format!("GenoType: {}", key),
                    format!("  # of borns : {:>}", alive_count + dead_count),
                    format!("  # of alives: {:>}", alive_count),
                    format!("  # of deads : {:>}", dead_count),
                    format!("  Genome     : [{}]", r.genome.iter().map(|&x| format!("{:?}", x)).collect::<Vec<String>>().join(", ")),
                ].join("\n")
            })
            .collect::<Vec<String>>()
            .join("\n\n")
    }
}

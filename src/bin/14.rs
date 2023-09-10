use itertools::Itertools;
use parse_display::{Display, FromStr};
use std::{collections::HashMap, fmt::Display, str::FromStr};

#[derive(Debug, PartialEq)]
struct Bitmask {
    ones_mask: u64,
    zeros_mask: u64,
    x_mask: u64,
}

impl Bitmask {
    pub fn new() -> Self {
        Self {
            ones_mask: 0,
            zeros_mask: 0,
            x_mask: 0,
        }
    }

    pub fn apply_to_memory_value(&self, number: &u64) -> u64 {
        (number & !(self.zeros_mask)) | self.ones_mask
    }

    pub fn apply_to_memory_address(&self, address: &u64) -> Vec<u64> {
        let modified_address = address | self.ones_mask;
        let combinations = (1 << self.x_mask.count_ones() as u64) as u64;

        let mut addresses = vec![];
        for c in 0..combinations {
            let mut current_combination_index = 0;
            let mut working_zeros_mask: u64 = 0;
            let mut working_ones_mask: u64 = 0;
            for i in 0..64 {
                if self.x_mask & (1 << i) > 0 {
                    if c & (1 << current_combination_index) > 0 {
                        working_ones_mask |= 1 << i;
                    } else {
                        working_zeros_mask |= 1 << i;
                    }
                    current_combination_index += 1;
                }
            }

            addresses.push((modified_address & !working_zeros_mask) | working_ones_mask);
        }

        addresses
    }
}

impl Display for Bitmask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Z{:064b} O{:064b} X{:064b}",
            self.zeros_mask, self.ones_mask, self.x_mask
        ))
    }
}

impl FromStr for Bitmask {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bm = Self::new();

        for (idx, bit) in s.chars().rev().enumerate() {
            match bit {
                '0' => bm.zeros_mask |= 1 << idx,
                '1' => bm.ones_mask |= 1 << idx,
                'X' => bm.x_mask |= 1 << idx,
                _ => return Err(()),
            }
        }

        Ok(bm)
    }
}

#[derive(Display, FromStr, PartialEq, Debug)]
enum Instruction {
    #[display("mask = {0}")]
    MaskUpdate(Bitmask),
    #[display("mem[{0}] = {1}")]
    MemoryWrite(u64, u64),
}

pub fn part_one(input: &str) -> Option<u64> {
    let instructions = input
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect_vec();

    let mut current_bm = Bitmask::new();
    let mut memory: HashMap<u64, u64> = HashMap::new();
    for i in instructions {
        match i {
            Instruction::MaskUpdate(bm) => current_bm = bm,
            Instruction::MemoryWrite(address, value) => {
                memory.insert(address, current_bm.apply_to_memory_value(&value));
            }
        }
    }

    Some(memory.values().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let instructions = input
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect_vec();

    let mut current_bm = Bitmask::new();
    let mut memory: HashMap<u64, u64> = HashMap::new();
    for i in instructions {
        match i {
            Instruction::MaskUpdate(bm) => current_bm = bm,
            Instruction::MemoryWrite(address, value) => {
                for new_address in current_bm.apply_to_memory_address(&address) {
                    memory.insert(new_address, value);
                }
            }
        }
    }

    Some(memory.values().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(51));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(208));
    }
}

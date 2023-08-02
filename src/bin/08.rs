use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub enum AbortState {
    Running,
    AbortByLoop,
    AbortByEnd,
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Noop(i32),
    Accumulator(i32),
    Jump(i32),
}

impl TryFrom<&str> for Operation {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (op, arg) = value.split_once(' ').unwrap();
        match op {
            "nop" => {
                if let Ok(n) = arg.parse::<i32>() {
                    Ok(Self::Noop(n))
                } else {
                    Err(())
                }
            }
            "acc" => {
                if let Ok(n) = arg.parse::<i32>() {
                    Ok(Self::Accumulator(n))
                } else {
                    Err(())
                }
            }
            "jmp" => {
                if let Ok(n) = arg.parse::<i32>() {
                    Ok(Self::Jump(n))
                } else {
                    Err(())
                }
            }
            _ => Err(()),
        }
    }
}

pub struct CPU {
    accumulator: i32,
    program_counter: usize,
    instructions: Vec<Operation>,
    inst_history: HashSet<usize>,
}

impl CPU {
    pub fn new_from_code(code: &str) -> CPU {
        CPU {
            accumulator: 0,
            program_counter: 0,
            instructions: code.lines().map(|s| s.try_into().unwrap()).collect(),
            inst_history: HashSet::new(),
        }
    }

    pub fn reset(&mut self) {
        self.accumulator = 0;
        self.program_counter = 0;
        self.inst_history = HashSet::new();
    }

    pub fn flip_inst(&mut self, index: usize) -> bool {
        self.instructions[index] = match self.instructions[index] {
            Operation::Noop(n) => Operation::Jump(n),
            Operation::Jump(n) => Operation::Noop(n),
            Operation::Accumulator(_) => return false,
        };
        true
    }

    // run an instruction and return if next instruction would be one already visited
    pub fn run_step(&mut self) -> AbortState {
        let op = self.instructions.get(self.program_counter).unwrap();
        self.inst_history.insert(self.program_counter);

        match op {
            Operation::Noop(_) => {
                self.program_counter += 1;
            }
            Operation::Accumulator(n) => {
                self.accumulator += n;
                self.program_counter += 1;
            }
            Operation::Jump(n) => {
                self.program_counter = (self.program_counter as i32 + n) as usize;
            }
        };

        if self.inst_history.contains(&self.program_counter) {
            AbortState::AbortByLoop
        } else if self.program_counter >= self.instructions.len() {
            AbortState::AbortByEnd
        } else {
            AbortState::Running
        }
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut cpu = CPU::new_from_code(input);
    while cpu.run_step() == AbortState::Running {}
    Some(cpu.accumulator)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut cpu = CPU::new_from_code(input);
    for index in 0..cpu.instructions.len() {
        cpu.flip_inst(index);
        cpu.reset();

        let mut state;
        loop {
            state = cpu.run_step();
            if state != AbortState::Running {
                break;
            }
        }

        if state == AbortState::AbortByEnd {
            return Some(cpu.accumulator);
        }
        cpu.flip_inst(index);
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_flip() {
        let mut cpu_nop = CPU::new_from_code("nop +5");
        assert!(cpu_nop.flip_inst(0));
        assert_eq!(cpu_nop.instructions[0], Operation::Jump(5));
        let mut cpu_acc = CPU::new_from_code("acc +5");
        assert!(!cpu_acc.flip_inst(0));
        assert_eq!(cpu_acc.instructions[0], Operation::Accumulator(5));
        let mut cpu_jmp = CPU::new_from_code("jmp +5");
        assert!(cpu_jmp.flip_inst(0));
        assert_eq!(cpu_jmp.instructions[0], Operation::Noop(5));
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(5));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}

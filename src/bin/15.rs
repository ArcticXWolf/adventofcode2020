use std::collections::HashMap;

use itertools::Itertools;

pub fn find_next_said_number(
    last_said_number: &u64,
    current_offset: &u64,
    mut hm: HashMap<u64, u64>,
) -> (u64, HashMap<u64, u64>) {
    if let Some(x) = hm.get(last_said_number) {
        let offset = *x;
        hm.insert(*last_said_number, *current_offset);
        return (*current_offset - offset, hm);
    }

    hm.insert(*last_said_number, *current_offset);
    (0, hm)
}

pub fn part_one(input: &str) -> Option<u64> {
    let numbers = input
        .lines()
        .flat_map(|s| s.split(',').filter_map(|n| n.parse::<u64>().ok()))
        .collect_vec();

    let mut hm: HashMap<u64, u64> = HashMap::new();
    let mut said_number = 0;
    for (idx, n) in numbers.iter().enumerate() {
        (said_number, hm) = find_next_said_number(n, &(idx as u64), hm);
    }

    for i in numbers.len()..2019 {
        (said_number, hm) = find_next_said_number(&said_number, &(i as u64), hm);
    }

    Some(said_number)
}

pub fn part_two(input: &str) -> Option<u64> {
    let numbers = input
        .lines()
        .flat_map(|s| s.split(',').filter_map(|n| n.parse::<u64>().ok()))
        .collect_vec();

    let mut hm: HashMap<u64, u64> = HashMap::new();
    let mut said_number = 0;
    for (idx, n) in numbers.iter().enumerate() {
        (said_number, hm) = find_next_said_number(n, &(idx as u64), hm);
    }

    let mut log_number = 10;
    for i in numbers.len()..(30000000 - 1) {
        (said_number, hm) = find_next_said_number(&said_number, &(i as u64), hm);
        if i >= log_number {
            // println!("At {} with number {}", i, said_number);
            log_number *= 10;
        }
    }

    Some(said_number)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(436));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), Some(175594));
    }
}

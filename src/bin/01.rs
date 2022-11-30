use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let numbers = input.lines().map(|l| l.parse::<u32>().unwrap());
    let mut combinations = numbers.combinations(2);
    Some(
        combinations
            .find(|a| a.iter().sum::<u32>() == 2020)
            .unwrap()
            .iter()
            .product(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let numbers = input.lines().map(|l| l.parse::<u32>().unwrap());
    let mut combinations = numbers.combinations(3);
    Some(
        combinations
            .find(|a| a.iter().sum::<u32>() == 2020)
            .unwrap()
            .iter()
            .product(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(514579));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(241861950));
    }
}

use std::collections::HashSet;

use itertools::Itertools;

mod parser {
    use nom::{
        character::complete::{alpha1, line_ending},
        error::Error as NomError,
        multi::separated_list1,
        sequence::pair,
        Finish, IResult,
    };

    pub fn parse_groups(s: &str) -> Result<Vec<Vec<&str>>, NomError<&str>> {
        let (_, x) = separated_list1(pair(line_ending, line_ending), parse_group)(s).finish()?;
        Ok(x)
    }

    fn parse_group(s: &str) -> IResult<&str, Vec<&str>> {
        separated_list1(line_ending, alpha1)(s)
    }
}

pub fn find_intersection(group: Vec<&str>) -> Vec<char> {
    match group.len() {
        0 => vec![],
        1 => group[0].chars().collect_vec(),
        _ => group[0]
            .chars()
            .filter(|c| group[1..].iter().all(|g| g.chars().contains(c)))
            .collect_vec(),
    }
}

pub fn find_union(group: Vec<&str>) -> Vec<char> {
    let mut set = HashSet::new();

    for cs in group {
        set.extend(cs.chars());
    }

    set.into_iter().collect_vec()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parser::parse_groups(input)
            .unwrap()
            .into_iter()
            .map(|g| find_union(g).len() as u32)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        parser::parse_groups(input)
            .unwrap()
            .into_iter()
            .map(|g| find_intersection(g).len() as u32)
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(11));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(6));
    }
}

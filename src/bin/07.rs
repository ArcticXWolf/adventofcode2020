use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug, PartialEq)]
pub struct Rule {
    outer: String,
    contained: Vec<(u32, String)>,
}

impl From<(&str, Vec<(&str, &str)>)> for Rule {
    fn from(value: (&str, Vec<(&str, &str)>)) -> Self {
        Self {
            outer: value.0.to_string(),
            contained: value
                .1
                .into_iter()
                .map(|(n, s)| (n.parse().unwrap(), s.to_string()))
                .collect_vec(),
        }
    }
}

fn find_bag_rules_containing(rules: &Vec<Rule>, mut bag_types: HashSet<String>) -> HashSet<String> {
    for rule in rules {
        if rule.contained.iter().any(|(_, bt)| bag_types.contains(bt)) {
            bag_types.insert(rule.outer.clone());
        }
    }
    bag_types
}

fn calculate_next_level_of_bags(
    rules: &Vec<Rule>,
    mut bag_map: HashMap<String, u32>,
) -> HashMap<String, u32> {
    for rule in rules {
        if bag_map.contains_key(&rule.outer) {
            continue;
        }

        if rule.contained.is_empty() {
            bag_map.insert(rule.outer.clone(), 1);
            continue;
        }

        if rule
            .contained
            .iter()
            .all(|(_, bt)| bag_map.contains_key(bt))
        {
            bag_map.insert(
                rule.outer.clone(),
                rule.contained
                    .iter()
                    .map(|(n, bt)| n * bag_map.get(bt).unwrap())
                    .sum::<u32>()
                    + 1,
            );
        }
    }

    bag_map
}

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::{tag, take_until},
        character::complete::{digit1, line_ending, space1},
        combinator::into,
        error::Error as NomError,
        multi::separated_list1,
        sequence::{separated_pair, terminated},
        Finish, IResult,
    };

    use crate::Rule;

    pub fn parse_rules(s: &str) -> Result<Vec<Rule>, NomError<&str>> {
        let (_, x) = separated_list1(line_ending, parse_rule)(s).finish()?;
        Ok(x)
    }

    pub fn parse_rule(s: &str) -> IResult<&str, Rule> {
        into(parse_rule_components)(s)
    }

    pub fn parse_rule_components(s: &str) -> IResult<&str, (&str, Vec<(&str, &str)>)> {
        separated_pair(parse_bag, tag(" contain "), parse_bag_list)(s)
    }

    pub fn parse_bag(s: &str) -> IResult<&str, &str> {
        terminated(take_until(" bag"), alt((tag(" bags"), tag(" bag"))))(s)
    }

    pub fn parse_bag_list(s: &str) -> IResult<&str, Vec<(&str, &str)>> {
        terminated(
            alt((
                parse_empty_bag_list,
                separated_list1(tag(", "), parse_bag_amount),
            )),
            tag("."),
        )(s)
    }

    pub fn parse_empty_bag_list(s: &str) -> IResult<&str, Vec<(&str, &str)>> {
        let x = tag("no other bags")(s);
        if let Ok((s, _)) = x {
            return Ok((s, vec![]));
        }
        Err(x.unwrap_err())
    }

    pub fn parse_bag_amount(s: &str) -> IResult<&str, (&str, &str)> {
        separated_pair(digit1, space1, parse_bag)(s)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let rules = parser::parse_rules(input).unwrap();
    let my_bag = String::from("shiny gold");
    let mut bag_types_containing =
        find_bag_rules_containing(&rules, HashSet::from([my_bag.clone()]));
    let mut bag_old = HashSet::new();
    bag_types_containing.remove(&my_bag);

    while bag_types_containing != bag_old {
        bag_old = bag_types_containing.clone();
        bag_types_containing = find_bag_rules_containing(&rules, bag_types_containing);
    }
    Some(bag_types_containing.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let rules = parser::parse_rules(input).unwrap();

    let my_bag = String::from("shiny gold");
    let mut bag_map = calculate_next_level_of_bags(&rules, HashMap::new());

    while !bag_map.contains_key(&my_bag) {
        bag_map = calculate_next_level_of_bags(&rules, bag_map);
    }
    Some(*bag_map.get(&my_bag).unwrap() - 1)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(parser::parse_bag("light red bags"), Ok(("", "light red")));
        assert_eq!(parser::parse_bag("light blue bag"), Ok(("", "light blue")));
        assert_eq!(
            parser::parse_bag_list("1 light blue bag, 2 light red bags."),
            Ok(("", vec![("1", "light blue"), ("2", "light red")]))
        );
        assert_eq!(
            parser::parse_rule("light red bags contain 1 bright white bag, 2 muted yellow bags."),
            Ok((
                "",
                Rule {
                    outer: String::from("light red"),
                    contained: vec![
                        (1, String::from("bright white")),
                        (2, String::from("muted yellow"))
                    ]
                }
            ))
        );
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(4));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(32));
    }
}

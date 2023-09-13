use std::{collections::HashMap, ops::Range};

use itertools::Itertools;

#[derive(Debug)]
pub struct RuleRange(Range<u32>);

impl From<(u32, u32)> for RuleRange {
    fn from(value: (u32, u32)) -> Self {
        Self(value.0..(value.1 + 1))
    }
}

#[derive(Debug)]
pub struct Rule {
    name: String,
    ranges: Vec<RuleRange>,
}

impl From<(&str, Vec<RuleRange>)> for Rule {
    fn from(value: (&str, Vec<RuleRange>)) -> Self {
        Self {
            name: value.0.to_string(),
            ranges: value.1,
        }
    }
}

impl Rule {
    pub fn is_rule_valid_for_number(&self, number: u32) -> bool {
        self.ranges.iter().any(|range| range.0.contains(&number))
    }

    pub fn is_rule_possible_for_numbers(&self, numbers: &[u32]) -> bool {
        numbers.iter().all(|n| self.is_rule_valid_for_number(*n))
    }
}

#[derive(Debug)]
pub struct Ticket {
    raw_numbers: Vec<u32>,
}

impl From<Vec<u32>> for Ticket {
    fn from(value: Vec<u32>) -> Self {
        Self { raw_numbers: value }
    }
}

impl Ticket {
    pub fn are_all_fields_valid_for_every_rule(&self, rules: &[Rule]) -> Result<(), u32> {
        for n in &self.raw_numbers {
            if rules.iter().any(|rule| rule.is_rule_valid_for_number(*n)) {
                continue;
            }
            return Err(*n);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct ReferenceFile {
    rules: Vec<Rule>,
    my_ticket: Ticket,
    other_tickets: Vec<Ticket>,
}

impl From<(Vec<Rule>, Ticket, Vec<Ticket>)> for ReferenceFile {
    fn from(value: (Vec<Rule>, Ticket, Vec<Ticket>)) -> Self {
        Self {
            rules: value.0,
            my_ticket: value.1,
            other_tickets: value.2,
        }
    }
}

mod parser {
    use nom::{
        bytes::complete::{tag, take_till},
        character::complete::{line_ending, u32},
        combinator::into,
        error::Error as NomError,
        multi::separated_list1,
        sequence::{pair, preceded, separated_pair, terminated, tuple},
        Finish, IResult,
    };

    use crate::{ReferenceFile, Rule, RuleRange, Ticket};

    pub fn parse_file(s: &str) -> Result<ReferenceFile, NomError<&str>> {
        let (_, x) = into(parse_file_raw)(s).finish()?;
        Ok(x)
    }

    fn parse_file_raw(s: &str) -> IResult<&str, (Vec<Rule>, Ticket, Vec<Ticket>)> {
        tuple((parse_rules, parse_my_ticket, parse_other_tickets))(s)
    }

    fn parse_rules(s: &str) -> IResult<&str, Vec<Rule>> {
        terminated(
            separated_list1(line_ending, parse_rule),
            pair(line_ending, line_ending),
        )(s)
    }

    fn parse_rule(s: &str) -> IResult<&str, Rule> {
        into(parse_rule_raw)(s)
    }

    fn parse_rule_raw(s: &str) -> IResult<&str, (&str, Vec<RuleRange>)> {
        separated_pair(parse_rule_name, tag(": "), parse_rangelist)(s)
    }

    fn parse_rule_name(s: &str) -> IResult<&str, &str> {
        take_till(|c| c == ':')(s)
    }

    fn parse_rangelist(s: &str) -> IResult<&str, Vec<RuleRange>> {
        separated_list1(tag(" or "), parse_range)(s)
    }

    fn parse_range(s: &str) -> IResult<&str, RuleRange> {
        into(parse_range_raw)(s)
    }

    fn parse_range_raw(s: &str) -> IResult<&str, (u32, u32)> {
        separated_pair(u32, tag("-"), u32)(s)
    }

    fn parse_my_ticket(s: &str) -> IResult<&str, Ticket> {
        preceded(
            pair(tag("your ticket:"), line_ending),
            terminated(parse_ticket, pair(line_ending, line_ending)),
        )(s)
    }

    fn parse_other_tickets(s: &str) -> IResult<&str, Vec<Ticket>> {
        preceded(
            pair(tag("nearby tickets:"), line_ending),
            separated_list1(line_ending, parse_ticket),
        )(s)
    }

    fn parse_ticket(s: &str) -> IResult<&str, Ticket> {
        into(parse_ticket_raw)(s)
    }

    fn parse_ticket_raw(s: &str) -> IResult<&str, Vec<u32>> {
        separated_list1(tag(","), u32)(s)
    }
}

pub fn construct_field_lists(tickets: &[Ticket], amount_fields: usize) -> Vec<Vec<u32>> {
    let mut result = vec![];
    for i in 0..amount_fields {
        let mut fieldlist = vec![];

        for ticket in tickets {
            fieldlist.push(*ticket.raw_numbers.get(i).unwrap());
        }

        result.push(fieldlist);
    }
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let ref_file = parser::parse_file(input).unwrap();

    let mut sum = 0;
    for ticket in ref_file.other_tickets {
        if let Err(error_code) = ticket.are_all_fields_valid_for_every_rule(&ref_file.rules) {
            sum += error_code;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ref_file = parser::parse_file(input).unwrap();

    let mut valid_tickets = vec![];
    for ticket in ref_file.other_tickets {
        if ticket.are_all_fields_valid_for_every_rule(&ref_file.rules) == Ok(()) {
            valid_tickets.push(ticket);
        }
    }
    let fieldlists = construct_field_lists(&valid_tickets, ref_file.rules.len());

    let mut recognized_fields: HashMap<String, usize> = HashMap::new();
    let mut last_amount_of_recognized_fields = 100;
    while recognized_fields.len() < fieldlists.len()
        && last_amount_of_recognized_fields != recognized_fields.len()
    {
        last_amount_of_recognized_fields = recognized_fields.len();
        for rule in &ref_file.rules {
            let mut possible_ids = vec![];
            for (flid, fieldlist) in fieldlists.iter().enumerate() {
                if recognized_fields.values().contains(&flid) {
                    continue;
                }
                if rule.is_rule_possible_for_numbers(fieldlist) {
                    possible_ids.push(flid);
                }
            }

            if possible_ids.len() == 1 {
                recognized_fields.insert(rule.name.clone(), *possible_ids.first().unwrap());
            }
        }
    }

    if recognized_fields.len() == ref_file.rules.len() {
        let mut result = 1;
        for (rule_name, id) in recognized_fields {
            if rule_name.starts_with("departure") {
                if let Some(x) = ref_file.my_ticket.raw_numbers.get(id) {
                    result *= *x as u64;
                }
            }
        }
        return Some(result);
    }

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(71));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), Some(7));
    }
}

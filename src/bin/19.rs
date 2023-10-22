use std::collections::HashMap;

// So for part 2 we need to explore all possible choices and thus we need to
// split at each choice. In this particular case we keep a list of all splits
// with all unmatched strings per split. If we are done parsing and this list
// contains one empty string, then there was one path where everything matched.
// (technically there could also be multiple paths, so we only check if there
// is at least one empty string)

// Adapted from https://github.com/jocelyn-stericker/aoc/blob/main/p2020_19/src/lib.rs

#[derive(Debug)]
struct RefList(Vec<u16>);
impl RefList {
    fn match_str<'a>(&self, rule_set: &RuleSet, s: &'a str) -> Vec<&'a str> {
        self.match_str_rule(rule_set, s, 0)
    }

    // match one rule of a rule list and recursively go to the next
    // keeping all possible paths from choices intact
    fn match_str_rule<'a>(
        &self,
        rule_set: &RuleSet,
        remaining: &'a str,
        current_idx: usize,
    ) -> Vec<&'a str> {
        let rule_idx = self.0.get(current_idx).unwrap();
        let rule = rule_set.0.get(rule_idx).unwrap();
        let remaining_options = rule.match_str(rule_set, remaining);

        if current_idx + 1 >= self.0.len() {
            return remaining_options;
        }

        let mut return_remaining_options = vec![];
        for r in remaining_options {
            for ro in self.match_str_rule(rule_set, r, current_idx + 1) {
                return_remaining_options.push(ro);
            }
        }
        return return_remaining_options;
    }
}

#[derive(Debug)]
enum Rule {
    Literal(char),
    Concatenation(RefList),
    Choice(RefList, RefList),
}

impl Rule {
    fn match_str<'a>(&self, rule_set: &RuleSet, remaining: &'a str) -> Vec<&'a str> {
        if remaining.is_empty() {
            return vec![];
        }

        match self {
            Self::Literal(c) => {
                // Strip of a matched character and return the remaining
                if remaining.starts_with(*c) {
                    let stripped = remaining.strip_prefix(*c).unwrap();
                    return vec![stripped];
                }
                // If there is no match, then we return an empty list (to not
                // continue with this path)
                vec![]
            }
            Self::Concatenation(reflist) => reflist.match_str(rule_set, remaining),
            Self::Choice(reflist1, reflist2) => {
                // try to match both paths and then include them in the list
                // of possible choice splits
                let mut remaining_options = vec![];
                for s in reflist1.match_str(rule_set, remaining) {
                    remaining_options.push(s);
                }
                for s in reflist2.match_str(rule_set, remaining) {
                    remaining_options.push(s);
                }

                remaining_options
            }
        }
    }
}

#[derive(Debug)]
struct RuleSet(HashMap<u16, Rule>);

impl Default for RuleSet {
    fn default() -> Self {
        Self(HashMap::default())
    }
}

impl RuleSet {
    fn match_str(&self, s: &str) -> bool {
        let initial_rule = self.0.get(&0).unwrap();
        let remaining = initial_rule.match_str(&self, s);
        remaining.iter().any(|s| s.is_empty())
    }
}

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{char, u16},
        combinator::map,
        error::Error as NomError,
        multi::separated_list1,
        sequence::{delimited, separated_pair},
        Finish, IResult,
    };

    use crate::{RefList, Rule};

    pub fn parse_key_rule(s: &str) -> Result<(u16, Rule), NomError<&str>> {
        let (_, x) = separated_pair(u16, tag(": "), parse_rule)(s).finish()?;
        Ok(x)
    }

    fn parse_rule(s: &str) -> IResult<&str, Rule> {
        alt((
            parse_rule_choice,
            parse_rule_concatenation,
            parse_rule_literal,
        ))(s)
    }

    fn parse_rule_literal(s: &str) -> IResult<&str, Rule> {
        map(parse_rule_literal_raw, Rule::Literal)(s)
    }

    fn parse_rule_literal_raw(s: &str) -> IResult<&str, char> {
        delimited(tag("\""), alt((char('a'), char('b'))), tag("\""))(s)
    }

    fn parse_rule_concatenation(s: &str) -> IResult<&str, Rule> {
        map(parse_rule_key_list, Rule::Concatenation)(s)
    }

    fn parse_rule_choice(s: &str) -> IResult<&str, Rule> {
        map(
            separated_pair(parse_rule_key_list, tag(" | "), parse_rule_key_list),
            |(p1, p2)| Rule::Choice(p1, p2),
        )(s)
    }

    fn parse_rule_key_list(s: &str) -> IResult<&str, RefList> {
        map(separated_list1(tag(" "), u16), RefList)(s)
    }
}

fn parse_input(s: &str) -> (RuleSet, Vec<&str>) {
    let (rules_str, messages_str) = s.split_once("\n\n").unwrap();
    let mut ruleset = RuleSet::default();
    for rs in rules_str.lines() {
        let (k, r) = parser::parse_key_rule(rs).unwrap();
        ruleset.0.insert(k, r);
    }
    (ruleset, messages_str.lines().collect())
}

pub fn part_one(_input: &str) -> Option<u32> {
    let (ruleset, messages) = parse_input(_input);

    Some(messages.iter().filter(|s| ruleset.match_str(*s)).count() as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    let (mut ruleset, messages) = parse_input(_input);

    ruleset
        .0
        .insert(8, Rule::Choice(RefList(vec![42]), RefList(vec![42, 8])));
    ruleset.0.insert(
        11,
        Rule::Choice(RefList(vec![42, 31]), RefList(vec![42, 11, 31])),
    );

    Some(messages.iter().filter(|s| ruleset.match_str(*s)).count() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concatenation_rule() {
        let mut ruleset = RuleSet::default();
        ruleset
            .0
            .insert(0, Rule::Concatenation(RefList(vec![1, 2, 3])));
        ruleset.0.insert(1, Rule::Literal('a'));
        ruleset.0.insert(2, Rule::Literal('b'));
        ruleset.0.insert(3, Rule::Literal('c'));

        assert_eq!(ruleset.match_str("abc"), true);
        assert_eq!(ruleset.match_str("abb"), false);
        assert_eq!(ruleset.match_str("cbc"), false);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_two(&input), Some(12));
    }
}

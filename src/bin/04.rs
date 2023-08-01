#[derive(Debug)]
pub enum PassportField {
    BirthYear(String),
    IssueYear(String),
    ExpirationYear(String),
    Height(String),
    HairColor(String),
    EyeColor(String),
    PassportId(String),
    CountryId(String),
}

impl From<(&str, &str)> for PassportField {
    fn from(value: (&str, &str)) -> Self {
        match value.0 {
            "byr" => PassportField::BirthYear(value.1.to_string()),
            "iyr" => PassportField::IssueYear(value.1.to_string()),
            "eyr" => PassportField::ExpirationYear(value.1.to_string()),
            "hgt" => PassportField::Height(value.1.to_string()),
            "hcl" => PassportField::HairColor(value.1.to_string()),
            "ecl" => PassportField::EyeColor(value.1.to_string()),
            "pid" => PassportField::PassportId(value.1.to_string()),
            "cid" => PassportField::CountryId(value.1.to_string()),
            _ => unreachable!(),
        }
    }
}

impl PassportField {
    fn is_valid(&self) -> bool {
        match self {
            PassportField::BirthYear(s) => {
                if let Ok(number) = s.parse::<u32>() {
                    return (1920..=2002).contains(&number);
                }
                false
            }
            PassportField::IssueYear(s) => {
                if let Ok(number) = s.parse::<u32>() {
                    return (2010..=2020).contains(&number);
                }
                false
            }
            PassportField::ExpirationYear(s) => {
                if let Ok(number) = s.parse::<u32>() {
                    return (2020..=2030).contains(&number);
                }
                false
            }
            PassportField::Height(s) => {
                if s.ends_with("cm") {
                    if let Ok(number) = s.strip_suffix("cm").unwrap().parse::<u32>() {
                        return (150..=193).contains(&number);
                    }
                } else if s.ends_with("in") {
                    if let Ok(number) = s.strip_suffix("in").unwrap().parse::<u32>() {
                        return (59..=76).contains(&number);
                    }
                }
                false
            }
            PassportField::HairColor(s) => {
                let hex = vec![
                    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
                ];
                if let Some((empty, code)) = s.split_once('#') {
                    return empty.is_empty() && code.chars().all(|x| hex.contains(&x));
                }
                false
            }
            PassportField::EyeColor(s) => {
                matches!(
                    s.as_str(),
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
                )
            }
            PassportField::PassportId(s) => {
                if let Ok(number) = s.parse::<u32>() {
                    return number < 1000000000 && s.chars().count() == 9;
                }
                false
            }
            PassportField::CountryId(_) => true,
        }
    }
}

#[derive(Debug)]
pub struct Passport {
    fields: Vec<PassportField>,
}

impl From<Vec<PassportField>> for Passport {
    fn from(value: Vec<PassportField>) -> Self {
        Passport { fields: value }
    }
}

impl Passport {
    fn is_valid(&self) -> bool {
        self.fields
            .iter()
            .any(|f| matches!(f, PassportField::BirthYear(_)))
            && self
                .fields
                .iter()
                .any(|f| matches!(f, PassportField::IssueYear(_)))
            && self
                .fields
                .iter()
                .any(|f| matches!(f, PassportField::ExpirationYear(_)))
            && self
                .fields
                .iter()
                .any(|f| matches!(f, PassportField::Height(_)))
            && self
                .fields
                .iter()
                .any(|f| matches!(f, PassportField::HairColor(_)))
            && self
                .fields
                .iter()
                .any(|f| matches!(f, PassportField::EyeColor(_)))
            && self
                .fields
                .iter()
                .any(|f| matches!(f, PassportField::PassportId(_)))
    }

    fn is_valid_with_fields(&self) -> bool {
        self.is_valid() && self.fields.iter().all(|x| x.is_valid())
    }
}

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alpha1, line_ending},
        combinator::into,
        error::Error as NomError,
        multi::separated_list1,
        sequence::{pair, separated_pair},
        Finish, IResult, InputTakeAtPosition,
    };

    use crate::{Passport, PassportField};

    pub fn parse_passports(s: &str) -> Result<Vec<Passport>, NomError<&str>> {
        let (_, x) = separated_list1(pair(line_ending, line_ending), parse_passport)(s).finish()?;
        Ok(x)
    }

    fn parse_passport(s: &str) -> IResult<&str, Passport> {
        into(parse_passport_fields)(s)
    }

    fn parse_passport_fields(s: &str) -> IResult<&str, Vec<PassportField>> {
        separated_list1(alt((line_ending, tag(" "))), parse_passport_field)(s)
    }

    fn parse_passport_field(s: &str) -> IResult<&str, PassportField> {
        into(parse_str_field)(s)
    }

    fn parse_str_field(s: &str) -> IResult<&str, (&str, &str)> {
        separated_pair(alpha1, tag(":"), parse_nonwhitespace)(s)
    }

    fn parse_nonwhitespace(s: &str) -> IResult<&str, &str> {
        s.split_at_position_complete(|item| item.is_whitespace())
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let passports = parser::parse_passports(input).unwrap();
    Some(
        passports
            .iter()
            .map(|p| p.is_valid())
            .filter(|b| *b)
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let passports = parser::parse_passports(input).unwrap();
    Some(
        passports
            .iter()
            .map(|p| p.is_valid_with_fields())
            .filter(|b| *b)
            .count() as u32,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(10));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(6));
    }
}

struct PasswordPolicyRange {
    min: u32,
    max: u32,
}

impl From<(u32, u32)> for PasswordPolicyRange {
    fn from(value: (u32, u32)) -> Self {
        let (min, max) = value;
        PasswordPolicyRange { min, max }
    }
}

pub struct Policy {
    char_range: PasswordPolicyRange,
    char: char,
}

impl From<(PasswordPolicyRange, char)> for Policy {
    fn from(value: (PasswordPolicyRange, char)) -> Self {
        Policy {
            char_range: value.0,
            char: value.1,
        }
    }
}

impl Policy {
    fn is_password_valid_sled(&self, password: &str) -> bool {
        let policy_count = password.chars().filter(|c| *c == self.char).count();
        self.char_range.min <= policy_count.try_into().unwrap()
            && self.char_range.max >= policy_count.try_into().unwrap()
    }

    fn is_password_valid_toboggan(&self, password: &str) -> bool {
        let (first, second) = (
            password
                .chars()
                .nth(self.char_range.min as usize - 1)
                .unwrap(),
            password
                .chars()
                .nth(self.char_range.max as usize - 1)
                .unwrap(),
        );
        (first == self.char) ^ (second == self.char)
    }
}

mod parser {
    use super::{PasswordPolicyRange, Policy};
    use nom::{
        bytes::complete::tag,
        character::complete::{alpha1, anychar, space1, u32},
        combinator::into,
        error::Error as NomError,
        sequence::separated_pair,
        Finish, IResult,
    };

    pub fn parse_line(line: &str) -> Result<(Policy, &str), NomError<&str>> {
        let (_, x) = separated_pair(policy, tag(": "), alpha1)(line).finish()?;
        Ok(x)
    }

    fn policy(s: &str) -> IResult<&str, Policy> {
        into(separated_pair(range, space1, anychar))(s)
    }

    fn range(s: &str) -> IResult<&str, PasswordPolicyRange> {
        into(number_pair)(s)
    }

    fn number_pair(s: &str) -> IResult<&str, (u32, u32)> {
        separated_pair(u32, tag("-"), u32)(s)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .flat_map(parser::parse_line)
            .into_iter()
            .map(|(policy, password)| policy.is_password_valid_sled(password))
            .filter(|b| *b)
            .count()
            .try_into()
            .unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .flat_map(parser::parse_line)
            .into_iter()
            .map(|(policy, password)| policy.is_password_valid_toboggan(password))
            .filter(|b| *b)
            .count()
            .try_into()
            .unwrap(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(1));
    }
}

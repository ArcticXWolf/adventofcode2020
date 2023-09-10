use advent_of_code::helpers::crt;
use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
pub enum ShuttleID {
    #[display("x")]
    Unknown,
    #[display("{0}")]
    Known(u32),
}

#[derive(Debug)]
struct Notes {
    arrival_time: u32,
    shuttle_times: Vec<ShuttleID>,
}

impl Notes {
    pub fn from_str(input: &str) -> Self {
        let arrival_time = input.lines().next().unwrap().parse::<u32>().unwrap();
        let shuttle_times = input
            .lines()
            .nth(1)
            .unwrap()
            .split(',')
            .filter_map(|x| x.parse::<ShuttleID>().ok())
            .collect_vec();

        Notes {
            arrival_time,
            shuttle_times,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let notes = Notes::from_str(input);

    let mut best_shuttle = (u32::MAX, u32::MAX);
    for sid in notes.shuttle_times {
        if let ShuttleID::Known(st) = sid {
            if st - notes.arrival_time % st < best_shuttle.1 {
                best_shuttle = (st, st - notes.arrival_time % st);
            }
        }
    }

    Some(best_shuttle.0 * best_shuttle.1)
}

pub fn calculate_t_from_sids(sids: Vec<ShuttleID>) -> i64 {
    let mut numbers: Vec<(i64, i64)> = vec![];
    for (sid_idx, sid) in sids.iter().enumerate() {
        if let ShuttleID::Known(s) = sid {
            numbers.push((*s as i64, (*s as i64) - (sid_idx as i64)));
        }
    }

    crt(numbers)
}

pub fn part_two(input: &str) -> Option<i64> {
    let notes = Notes::from_str(input);

    Some(calculate_t_from_sids(notes.shuttle_times))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(295));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(1068781));
    }

    #[test]
    fn test_part_two_t_calculation_multiple() {
        assert_eq!(
            calculate_t_from_sids(vec![
                ShuttleID::Known(7),
                ShuttleID::Known(13),
                ShuttleID::Unknown,
                ShuttleID::Known(17)
            ]),
            1442
        );
    }
}

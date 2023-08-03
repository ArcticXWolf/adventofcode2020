use itertools::Itertools;

pub fn find_wrong_number(numbers: &Vec<i64>, preamble: usize) -> Option<i64> {
    for (i, n) in numbers.iter().skip(preamble).enumerate() {
        let numbers_allowed = numbers.iter().skip(i).take(preamble);
        if numbers_allowed.clone().all(|x| {
            if (n - x) == *x {
                true
            } else {
                !numbers_allowed.clone().contains(&(*n - *x))
            }
        }) {
            return Some(*n);
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<i64> {
    let numbers = input
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect_vec();
    find_wrong_number(&numbers, 25)
}

pub fn find_weakness(numbers: &Vec<i64>, preamble: usize) -> Option<i64> {
    let wrong_number = find_wrong_number(numbers, preamble).unwrap();

    'outer: for (start_index, _) in numbers.iter().enumerate() {
        let mut running_total = 0;
        let mut smallest_n = i64::MAX;
        let mut largest_n = i64::MIN;

        for current_n in numbers.iter().skip(start_index) {
            running_total += current_n;
            smallest_n = smallest_n.min(*current_n);
            largest_n = largest_n.max(*current_n);
            match running_total.cmp(&wrong_number) {
                std::cmp::Ordering::Equal => return Some(smallest_n + largest_n),
                std::cmp::Ordering::Greater => continue 'outer,
                std::cmp::Ordering::Less => (),
            }
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<i64> {
    let numbers = input
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect_vec();
    find_weakness(&numbers, 25)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        let numbers = input
            .lines()
            .map(|l| l.parse::<i64>().unwrap())
            .collect_vec();
        assert_eq!(find_wrong_number(&numbers, 5), Some(127));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        let numbers = input
            .lines()
            .map(|l| l.parse::<i64>().unwrap())
            .collect_vec();
        assert_eq!(find_weakness(&numbers, 5), Some(62));
    }
}

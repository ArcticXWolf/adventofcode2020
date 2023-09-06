use itertools::Itertools;

pub fn get_sorted_adapter_list(input: &str) -> Vec<u32> {
    let mut adapters_list = input
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect_vec();

    adapters_list.push(0);
    adapters_list.push(adapters_list.iter().max().unwrap() + 3);
    adapters_list.sort();
    adapters_list
}

pub fn split_into_isolated_lists(input_list: Vec<u32>) -> Vec<Vec<u32>> {
    let mut results = vec![];
    let mut current_list = vec![];

    for (a, b) in input_list.iter().tuple_windows() {
        current_list.push(*a);
        if b - a == 3 {
            results.push(current_list);
            current_list = vec![];
        }
    }

    results
}

pub fn calculate_combinations_of_list(input_list: Vec<u32>) -> u32 {
    match input_list.len() {
        0 => return 0, // should not happen
        1 | 2 => return 1,
        _ => {}
    }

    // first and last element are fixed in a list. Create all combinations of elements between them.
    let mut combinations = 0;
    for x in input_list
        .iter()
        .skip(1)
        .take(input_list.len() - 2)
        .powerset()
    {
        // recreate the combination list with first and last element from original list.
        let mut intermediate_list = vec![input_list[0]];
        intermediate_list.append(&mut x.iter().map(|n| **n).collect_vec());
        intermediate_list.push(*input_list.last().unwrap());

        if is_valid_adapter_chain(intermediate_list) {
            combinations += 1;
        }
    }

    combinations
}

pub fn is_valid_adapter_chain(input_list: Vec<u32>) -> bool {
    match input_list.len() {
        0 | 1 => false, // should not happen
        _ => {
            for (a, b) in input_list.iter().tuple_windows() {
                if b - a > 3 {
                    return false;
                }
            }
            true
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let adapters_list = get_sorted_adapter_list(input);

    let mut result_1 = 0;
    let mut result_3 = 0;
    for (a, b) in adapters_list.iter().tuple_windows() {
        if b - a == 1 {
            result_1 += 1;
        } else if b - a == 3 {
            result_3 += 1;
        }
    }
    Some(result_1 * result_3)
}

pub fn part_two(input: &str) -> Option<u64> {
    let adapters_list = get_sorted_adapter_list(input);

    let mut combinations_per_sublist = vec![];
    for sublist in split_into_isolated_lists(adapters_list) {
        combinations_per_sublist.push(calculate_combinations_of_list(sublist) as u64);
    }

    Some(combinations_per_sublist.iter().product())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(35));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), Some(8));
    }
}

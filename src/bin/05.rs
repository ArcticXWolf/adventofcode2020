use itertools::Itertools;

#[derive(Debug)]
pub struct SeatAssignment {
    row: u32,
    column: u32,
}

impl TryFrom<&str> for SeatAssignment {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.chars().count() != 10 {
            return Err(());
        }
        let (row_text, col_text) = value.split_at(7);
        let row_text = row_text.replace('F', "0").replace('B', "1");
        let col_text = col_text.replace('L', "0").replace('R', "1");
        let row = match u32::from_str_radix(&row_text, 2) {
            Ok(n) => n,
            Err(_) => return Err(()),
        };
        let col = match u32::from_str_radix(&col_text, 2) {
            Ok(n) => n,
            Err(_) => return Err(()),
        };

        Ok(SeatAssignment { row, column: col })
    }
}

impl SeatAssignment {
    fn seat_id(&self) -> u32 {
        self.row * 8 + self.column
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let seat_assignments: Vec<SeatAssignment> =
        input.lines().map(|l| l.try_into().unwrap()).collect_vec();
    Some(
        seat_assignments
            .iter()
            .map(|sa| sa.seat_id())
            .max()
            .unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let seat_assignments: Vec<SeatAssignment> =
        input.lines().map(|l| l.try_into().unwrap()).collect_vec();
    let seat_ids = seat_assignments.iter().map(|sa| sa.seat_id()).collect_vec();
    let seat_min = *seat_ids.iter().min().unwrap();
    let seat_max = *seat_ids.iter().max().unwrap();
    let my_seat = (seat_min..seat_max)
        .find(|x| {
            seat_ids.contains(&(*x + 1)) && seat_ids.contains(&(*x - 1)) && !seat_ids.contains(x)
        })
        .unwrap();

    Some(my_seat)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(820));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(817));
    }
}

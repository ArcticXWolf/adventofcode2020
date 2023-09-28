use advent_of_code::algebra_helpers::{Point2, PointGrid};
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
pub enum Space {
    #[display(".")]
    Floor,
    #[display("L")]
    Empty,
    #[display("#")]
    Occupied,
}

pub fn create_grid(input: &str) -> PointGrid<isize, 2, Space> {
    let mut grid = PointGrid::default();

    for (y, row) in input.lines().enumerate() {
        for (x, character) in row.chars().enumerate() {
            grid.insert(
                Point2::new(x as isize, y as isize),
                character.to_string().parse().unwrap(),
            )
        }
    }

    grid
}

pub fn step_gol(
    input: &PointGrid<isize, 2, Space>,
    next_state_function: &dyn Fn(&Point2<isize>, &PointGrid<isize, 2, Space>) -> Space,
) -> (PointGrid<isize, 2, Space>, bool) {
    let mut new_grid = PointGrid::default();
    let mut changed = false;
    for pos in input.iter_full_bounds() {
        let new_value = next_state_function(&pos, input);

        if *input.get(&pos).unwrap() != new_value {
            changed = true;
        }

        new_grid.insert(pos, new_value);
    }

    (new_grid, changed)
}

pub fn get_next_state_part1(pos: &Point2<isize>, grid: &PointGrid<isize, 2, Space>) -> Space {
    match grid.get(pos) {
        Some(Space::Floor) => Space::Floor,
        Some(Space::Empty) => {
            if Point2::<isize>::directions_with_diagonals()
                .iter()
                .filter_map(|d| grid.get(&(*pos + *d)))
                .all(|s| *s == Space::Empty || *s == Space::Floor)
            {
                Space::Occupied
            } else {
                Space::Empty
            }
        }
        Some(Space::Occupied) => {
            if Point2::<isize>::directions_with_diagonals()
                .iter()
                .filter_map(|d| grid.get(&(*pos + *d)))
                .filter(|s| **s == Space::Occupied)
                .count()
                >= 4
            {
                Space::Empty
            } else {
                Space::Occupied
            }
        }
        None => unreachable!(),
    }
}

pub fn get_next_state_part2(pos: &Point2<isize>, grid: &PointGrid<isize, 2, Space>) -> Space {
    match grid.get(pos) {
        Some(Space::Floor) => Space::Floor,
        Some(Space::Empty) => {
            for d in Point2::<isize>::directions_with_diagonals().iter() {
                let mut distance = 1;
                loop {
                    let s = grid.get(&(*pos + *d * distance));
                    match s {
                        Some(Space::Floor) => {
                            distance += 1;
                            continue;
                        }
                        Some(Space::Occupied) => {
                            return Space::Empty;
                        }
                        Some(Space::Empty) => {
                            break;
                        }
                        None => {
                            break;
                        }
                    }
                }
            }
            Space::Occupied
        }
        Some(Space::Occupied) => {
            let mut occupied_seats = 0;
            for d in Point2::<isize>::directions_with_diagonals().iter() {
                let mut distance = 1;
                loop {
                    let s = grid.get(&(*pos + *d * distance));
                    match s {
                        Some(Space::Floor) => {
                            distance += 1;
                            continue;
                        }
                        Some(Space::Occupied) => {
                            occupied_seats += 1;
                            break;
                        }
                        Some(Space::Empty) => {
                            break;
                        }
                        None => {
                            break;
                        }
                    }
                }
            }

            if occupied_seats >= 5 {
                Space::Empty
            } else {
                Space::Occupied
            }
        }
        None => unreachable!(),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = create_grid(input);

    let mut changed = true;
    while changed {
        (grid, changed) = step_gol(&grid, &get_next_state_part1);
    }

    Some(
        grid.0
            .iter()
            .filter(|(_, v)| **v == Space::Occupied)
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = create_grid(input);

    let mut changed = true;
    while changed {
        (grid, changed) = step_gol(&grid, &get_next_state_part2);
    }

    Some(
        grid.0
            .iter()
            .filter(|(_, v)| **v == Space::Occupied)
            .count() as u32,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(37));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(26));
    }
}

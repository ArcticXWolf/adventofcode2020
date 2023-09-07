use advent_of_code::helpers::{Point, PointDirection, PointGrid};
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

pub fn create_grid(input: &str) -> PointGrid<Space> {
    let mut grid = PointGrid::default();

    for (y, row) in input.lines().enumerate() {
        for (x, character) in row.chars().enumerate() {
            grid.insert(
                Point {
                    x: x as isize,
                    y: y as isize,
                },
                character.to_string().parse().unwrap(),
            )
        }
    }

    grid
}

pub fn step_gol(
    input: &PointGrid<Space>,
    next_state_function: &dyn Fn(&Point<isize>, &PointGrid<Space>) -> Space,
) -> (PointGrid<Space>, bool) {
    let mut new_grid = PointGrid::default();
    let (min_dimensions, max_dimensions) = input.dimensions();
    let mut changed = false;
    for y in min_dimensions.y..=max_dimensions.y {
        for x in min_dimensions.x..=max_dimensions.x {
            let pos = Point { x, y };
            let new_value = next_state_function(&pos, input);

            if *input.get(&pos).unwrap() != new_value {
                changed = true;
            }

            new_grid.insert(pos, new_value);
        }
    }

    (new_grid, changed)
}

pub fn get_next_state_part1(pos: &Point<isize>, grid: &PointGrid<Space>) -> Space {
    match grid.get(pos) {
        Some(Space::Floor) => Space::Floor,
        Some(Space::Empty) => {
            if PointDirection::all_with_diagonals()
                .filter_map(|d| grid.get(&pos.get_point_in_direction(d, 1)))
                .all(|s| *s == Space::Empty || *s == Space::Floor)
            {
                Space::Occupied
            } else {
                Space::Empty
            }
        }
        Some(Space::Occupied) => {
            if PointDirection::all_with_diagonals()
                .filter_map(|d| grid.get(&pos.get_point_in_direction(d, 1)))
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

pub fn get_next_state_part2(pos: &Point<isize>, grid: &PointGrid<Space>) -> Space {
    match grid.get(pos) {
        Some(Space::Floor) => Space::Floor,
        Some(Space::Empty) => {
            for d in PointDirection::all_with_diagonals() {
                let mut distance = 1;
                loop {
                    let s = grid.get(&pos.get_point_in_direction(d, distance));
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
            for d in PointDirection::all_with_diagonals() {
                let mut distance = 1;
                loop {
                    let s = grid.get(&pos.get_point_in_direction(d, distance));
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
        grid.points
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
        grid.points
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

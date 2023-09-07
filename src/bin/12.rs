use advent_of_code::helpers::{Point, PointDirection};
use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
enum NavInstruction {
    #[display("N{0}")]
    North(u32),
    #[display("E{0}")]
    East(u32),
    #[display("S{0}")]
    South(u32),
    #[display("W{0}")]
    West(u32),
    #[display("L{0}")]
    Left(u32),
    #[display("R{0}")]
    Right(u32),
    #[display("F{0}")]
    Forward(u32),
}

#[derive(Debug)]
struct Ferry {
    waypoint: Point<isize>,
    position: Point<isize>,
    facing_direction: PointDirection,
}

impl Ferry {
    pub fn new() -> Self {
        Self {
            waypoint: Point { x: 10, y: -1 },
            position: Point { x: 0, y: 0 },
            facing_direction: PointDirection::East,
        }
    }

    pub fn adjust_by_instruction_part1(&mut self, instruction: &NavInstruction) {
        match instruction {
            NavInstruction::North(d) => {
                self.position = self
                    .position
                    .get_point_in_direction(&PointDirection::North, *d as isize);
            }
            NavInstruction::East(d) => {
                self.position = self
                    .position
                    .get_point_in_direction(&PointDirection::East, *d as isize);
            }
            NavInstruction::South(d) => {
                self.position = self
                    .position
                    .get_point_in_direction(&PointDirection::South, *d as isize);
            }
            NavInstruction::West(d) => {
                self.position = self
                    .position
                    .get_point_in_direction(&PointDirection::West, *d as isize);
            }
            NavInstruction::Left(d) => {
                self.facing_direction = match d % 360 {
                    0 => self.facing_direction.clone(),
                    90 => self.facing_direction.direction_left(),
                    180 => self.facing_direction.direction_left().direction_left(),
                    270 => self
                        .facing_direction
                        .direction_left()
                        .direction_left()
                        .direction_left(),
                    _ => {
                        println!("{:?}", instruction);
                        unimplemented!();
                    }
                };
            }
            NavInstruction::Right(d) => {
                self.facing_direction = match d % 360 {
                    0 => self.facing_direction.clone(),
                    90 => self.facing_direction.direction_right(),
                    180 => self.facing_direction.direction_right().direction_right(),
                    270 => self
                        .facing_direction
                        .direction_right()
                        .direction_right()
                        .direction_right(),
                    _ => {
                        println!("{:?}", instruction);
                        unimplemented!();
                    }
                };
            }
            NavInstruction::Forward(d) => {
                self.position = self
                    .position
                    .get_point_in_direction(&self.facing_direction, *d as isize);
            }
        }
    }

    pub fn adjust_by_instruction_part2(&mut self, instruction: &NavInstruction) {
        match instruction {
            NavInstruction::North(d) => {
                self.waypoint = self
                    .waypoint
                    .get_point_in_direction(&PointDirection::North, *d as isize);
            }
            NavInstruction::East(d) => {
                self.waypoint = self
                    .waypoint
                    .get_point_in_direction(&PointDirection::East, *d as isize);
            }
            NavInstruction::South(d) => {
                self.waypoint = self
                    .waypoint
                    .get_point_in_direction(&PointDirection::South, *d as isize);
            }
            NavInstruction::West(d) => {
                self.waypoint = self
                    .waypoint
                    .get_point_in_direction(&PointDirection::West, *d as isize);
            }
            NavInstruction::Left(d) => {
                self.waypoint = match d % 360 {
                    0 => self.waypoint,
                    90 => spin_pos_left(&self.waypoint),
                    180 => spin_pos_left(&spin_pos_left(&self.waypoint)),
                    270 => spin_pos_right(&self.waypoint),
                    _ => {
                        println!("{:?}", instruction);
                        unimplemented!();
                    }
                };
            }
            NavInstruction::Right(d) => {
                self.waypoint = match d % 360 {
                    0 => self.waypoint,
                    90 => spin_pos_right(&self.waypoint),
                    180 => spin_pos_left(&spin_pos_left(&self.waypoint)),
                    270 => spin_pos_left(&self.waypoint),
                    _ => {
                        println!("{:?}", instruction);
                        unimplemented!();
                    }
                };
            }
            NavInstruction::Forward(d) => {
                for _ in 0..*d {
                    self.position = self.position + self.waypoint;
                }
            }
        }
    }
}

pub fn spin_pos_right(pos: &Point<isize>) -> Point<isize> {
    Point {
        x: -pos.y,
        y: pos.x,
    }
}

pub fn spin_pos_left(pos: &Point<isize>) -> Point<isize> {
    Point {
        x: pos.y,
        y: -pos.x,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let instructions = input
        .lines()
        .filter_map(|l| l.parse::<NavInstruction>().ok())
        .collect_vec();
    let mut ferry = Ferry::new();

    for i in instructions {
        ferry.adjust_by_instruction_part1(&i);
    }

    Some(ferry.position.manhattan_distance(&Point { x: 0, y: 0 }) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let instructions = input
        .lines()
        .filter_map(|l| l.parse::<NavInstruction>().ok())
        .collect_vec();
    let mut ferry = Ferry::new();

    for i in instructions {
        ferry.adjust_by_instruction_part2(&i);
    }

    Some(ferry.position.manhattan_distance(&Point { x: 0, y: 0 }) as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(25));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(286));
    }
}

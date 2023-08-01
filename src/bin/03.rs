use std::fmt::Display;

use itertools::Itertools;

enum Location {
    Empty,
    Tree,
}

impl TryFrom<char> for Location {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Location::Empty),
            '#' => Ok(Location::Tree),
            _ => Err(()),
        }
    }
}

struct Slope {
    x_offset: u32,
    y_offset: u32,
}

struct Map {
    grid: Vec<Vec<Location>>,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        Map {
            grid: value
                .lines()
                .map(|x| x.chars().map(|x| x.try_into().unwrap()).collect_vec())
                .collect_vec(),
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.grid.iter() {
            for loc in line {
                let char_s = match loc {
                    Location::Empty => '.',
                    Location::Tree => '#',
                };
                f.write_fmt(format_args!("{}", char_s))?;
            }
            f.write_str("\n")?;
        }
        f.write_str("\n")
    }
}

impl Map {
    fn count_trees_on_slope(&self, slope: Slope) -> u32 {
        let mut pos = (0, 0);
        let size = (self.grid.get(0).unwrap().len(), self.grid.len());
        let mut trees = 0;

        while pos.1 < size.1 {
            if let Location::Tree = self.grid.get(pos.1).unwrap().get(pos.0).unwrap() {
                trees += 1;
            }

            pos.0 += slope.x_offset as usize;
            pos.0 %= size.0;
            pos.1 += slope.y_offset as usize;
        }

        trees
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map: Map = input.into();
    let slope = Slope {
        x_offset: 3,
        y_offset: 1,
    };

    Some(map.count_trees_on_slope(slope))
}

pub fn part_two(input: &str) -> Option<u32> {
    let map: Map = input.into();
    let slopes = vec![
        Slope {
            x_offset: 1,
            y_offset: 1,
        },
        Slope {
            x_offset: 3,
            y_offset: 1,
        },
        Slope {
            x_offset: 5,
            y_offset: 1,
        },
        Slope {
            x_offset: 7,
            y_offset: 1,
        },
        Slope {
            x_offset: 1,
            y_offset: 2,
        },
    ];

    Some(
        slopes
            .into_iter()
            .map(|s| map.count_trees_on_slope(s))
            .product(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(336));
    }
}

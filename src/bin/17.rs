use advent_of_code::algebra_helpers::{Point, PointGrid, PointGridIterator};
use parse_display::{Display, FromStr, ParseError};

#[derive(Display, FromStr, PartialEq, Debug)]
enum Cell {
    #[display("#")]
    Active,
    #[display(".")]
    Inactive,
}

impl Cell {
    pub fn calculate_status_from_neighbor_count(&self, n: isize) -> Cell {
        match self {
            Cell::Active => match n {
                2 | 3 => Cell::Active,
                _ => Cell::Inactive,
            },
            Cell::Inactive => match n {
                3 => Cell::Active,
                _ => Cell::Inactive,
            },
        }
    }
}

#[derive(Debug)]
struct PocketDimension<const N: usize>(PointGrid<isize, N, Cell>);

impl<const N: usize> PocketDimension<N> {
    pub fn calculate_next_gol_step(&self) -> PocketDimension<N> {
        let mut new_dimension: PocketDimension<N> = Self(PointGrid::default());
        let (min, max) = self.0.dimensions();
        for p in PointGridIterator::new(min + Point::filled(-1), max + Point::filled(2)) {
            let old_state = self.0.get(&p).unwrap_or(&Cell::Inactive);
            let amount_active_neighbors = self.amount_active_neighbors(&p) as isize;
            let new_state = old_state.calculate_status_from_neighbor_count(amount_active_neighbors);
            // println!(
            //     "Point {} has {} neighbors: {} -> {}",
            //     p, amount_active_neighbors, old_state, new_state
            // );

            if new_state == Cell::Active {
                new_dimension.0.insert(p, new_state);
            }
        }

        new_dimension
    }

    pub fn amount_active_neighbors(&self, pos: &Point<isize, N>) -> usize {
        Point::directions_with_diagonals()
            .iter()
            .filter_map(|d| self.0.get(&(*pos + *d)))
            .filter(|c| **c == Cell::Active)
            .count()
    }

    pub fn amount_active_cells(&self) -> usize {
        self.0 .0.values().filter(|v| **v == Cell::Active).count()
    }
}

impl<const N: usize> TryFrom<&str> for PocketDimension<N> {
    type Error = ParseError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut dimension: PocketDimension<N> = Self(PointGrid::default());

        for (y, row) in input.lines().enumerate() {
            for (x, character) in row.chars().enumerate() {
                let mut point: [isize; N] = Point::zero().into();
                point[0] = x as isize;
                point[1] = y as isize;
                dimension
                    .0
                    .insert(point.into(), character.to_string().parse::<Cell>()?);
            }
        }

        Ok(dimension)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut pd: PocketDimension<3> = input.try_into().unwrap();

    for _ in 0..6 {
        pd = pd.calculate_next_gol_step();
    }

    Some(pd.amount_active_cells())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut pd: PocketDimension<4> = input.try_into().unwrap();

    for _ in 0..6 {
        pd = pd.calculate_next_gol_step();
    }

    Some(pd.amount_active_cells())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(112));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), Some(848));
    }
}

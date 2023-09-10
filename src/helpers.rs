use std::{
    collections::HashMap,
    fmt::{self, Display},
    ops::{Add, Sub},
    slice::Iter,
};

use parse_display::{Display, FromStr};
/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

#[derive(Debug, Clone, Display, FromStr, PartialEq, Eq)]
pub enum PointDirection {
    #[display("^")]
    North,
    #[display("/")]
    NorthEast,
    #[display(">")]
    East,
    #[display("\\")]
    SouthEast,
    #[display("v")]
    South,
    #[display("/")]
    SouthWest,
    #[display("<")]
    West,
    #[display("\\")]
    NorthWest,
}

impl PointDirection {
    pub fn all_with_diagonals() -> Iter<'static, PointDirection> {
        static D: [PointDirection; 8] = [
            PointDirection::North,
            PointDirection::NorthEast,
            PointDirection::East,
            PointDirection::SouthEast,
            PointDirection::South,
            PointDirection::SouthWest,
            PointDirection::West,
            PointDirection::NorthWest,
        ];

        D.iter()
    }

    pub fn all() -> Iter<'static, PointDirection> {
        static D: [PointDirection; 4] = [
            PointDirection::North,
            PointDirection::East,
            PointDirection::South,
            PointDirection::West,
        ];

        D.iter()
    }

    pub fn direction_left(&self) -> Self {
        match self {
            PointDirection::North => PointDirection::West,
            PointDirection::East => PointDirection::North,
            PointDirection::South => PointDirection::East,
            PointDirection::West => PointDirection::South,
            _ => unimplemented!(),
        }
    }

    pub fn direction_right(&self) -> Self {
        match self {
            PointDirection::North => PointDirection::East,
            PointDirection::East => PointDirection::South,
            PointDirection::South => PointDirection::West,
            PointDirection::West => PointDirection::North,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Display, FromStr)]
#[display("({x},{y})")]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Point<isize> {
    pub fn manhattan_distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    pub fn get_point_in_direction(&self, direction: &PointDirection, distance: isize) -> Self {
        match direction {
            PointDirection::North => Self {
                x: self.x,
                y: self.y - distance,
            },
            PointDirection::NorthEast => Self {
                x: self.x + distance,
                y: self.y - distance,
            },
            PointDirection::East => Self {
                x: self.x + distance,
                y: self.y,
            },
            PointDirection::SouthEast => Self {
                x: self.x + distance,
                y: self.y + distance,
            },
            PointDirection::South => Self {
                x: self.x,
                y: self.y + distance,
            },
            PointDirection::SouthWest => Self {
                x: self.x - distance,
                y: self.y + distance,
            },
            PointDirection::West => Self {
                x: self.x - distance,
                y: self.y,
            },
            PointDirection::NorthWest => Self {
                x: self.x - distance,
                y: self.y - distance,
            },
        }
    }

    pub fn is_in_rectangle(&self, min: Self, max: Self) -> bool {
        self.x >= min.x && self.x < max.x && self.y >= min.y && self.y > max.y
    }

    pub fn wrap_around_in_rectangle(&self, min: Self, max: Self) -> Self {
        let mut new_x = self.x;
        let mut new_y = self.y;

        if new_x < min.x {
            new_x = max.x - 1;
        } else if new_x >= max.x {
            new_x = min.x;
        }

        if new_y < min.y {
            new_y = max.y - 1;
        } else if new_y >= max.y {
            new_y = min.y;
        }

        Point { x: new_x, y: new_y }
    }
}

#[derive(Debug, Clone)]
pub struct PointGrid<U> {
    pub points: HashMap<Point<isize>, U>,
}

impl<U> Default for PointGrid<U> {
    fn default() -> Self {
        Self {
            points: HashMap::new(),
        }
    }
}

impl<U> PointGrid<U> {
    pub fn insert(&mut self, coord: Point<isize>, value: U) {
        self.points.insert(coord, value);
    }
    pub fn get(&self, coord: &Point<isize>) -> Option<&U> {
        self.points.get(coord)
    }

    pub fn dimensions(&self) -> (Point<isize>, Point<isize>) {
        (
            Point {
                x: self.points.keys().map(|p| p.x).min().unwrap(),
                y: self.points.keys().map(|p| p.y).min().unwrap(),
            },
            Point {
                x: self.points.keys().map(|p| p.x).max().unwrap(),
                y: self.points.keys().map(|p| p.y).max().unwrap(),
            },
        )
    }

    pub fn wrap_around(&self, point: &Point<isize>, direction: &PointDirection) -> Point<isize> {
        match direction {
            PointDirection::North => *self
                .points
                .keys()
                .filter(|p| p.x == point.x)
                .max_by_key(|p| p.y)
                .unwrap(),
            PointDirection::East => *self
                .points
                .keys()
                .filter(|p| p.y == point.y)
                .min_by_key(|p| p.x)
                .unwrap(),
            PointDirection::South => *self
                .points
                .keys()
                .filter(|p| p.x == point.x)
                .min_by_key(|p| p.y)
                .unwrap(),
            PointDirection::West => *self
                .points
                .keys()
                .filter(|p| p.y == point.y)
                .max_by_key(|p| p.x)
                .unwrap(),
            _ => unimplemented!(),
        }
    }
}

impl<U> Display for PointGrid<U>
where
    U: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (min, max) = self.dimensions();
        writeln!(f, "Grid ({}, {}):", min, max)?;
        for y in min.y..(max.y + 1) {
            for x in min.x..(max.x + 1) {
                if let Some(u) = self.get(&Point { x, y }) {
                    write!(f, "{}", u)?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        write!(f, "")
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Add<Output = T>> Add for Point3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Point3<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Point3<i32> {
    pub fn directions() -> Vec<Point3<i32>> {
        vec![
            Point3 { x: -1, y: 0, z: 0 }, // West
            Point3 { x: 1, y: 0, z: 0 },  // East
            Point3 { x: 0, y: -1, z: 0 }, // South
            Point3 { x: 0, y: 1, z: 0 },  // North
            Point3 { x: 0, y: 0, z: -1 }, // Above
            Point3 { x: 0, y: 0, z: 1 },  // Below
        ]
    }
}

pub fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

pub fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        std::mem::swap(&mut max, &mut min);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

// returns (g, x, y) for a*x + b*y = g
pub fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    let mut max = a;
    let mut min = b;

    let mut prev_x: i64 = 1;
    let mut x: i64 = 0;
    let mut prev_y: i64 = 0;
    let mut y: i64 = 1;

    loop {
        let q = max / min;
        (x, prev_x) = (prev_x - q * x, x);
        (y, prev_y) = (prev_y - q * y, y);
        (max, min) = (min, max % min);

        if min == 0 {
            return (max, prev_x, prev_y);
        }
    }
}

// Extended gcd for multiple numbers
pub fn egcd_mn(numbers: &Vec<i64>) -> Option<(i64, Vec<i64>)> {
    if numbers.len() < 2 {
        return None;
    }
    let mut results: Vec<i64> = vec![1];
    let mut current = *numbers.first().unwrap();
    for n in numbers.iter().skip(1) {
        let (g, x, y) = egcd(current, *n);
        results = results.into_iter().map(|r| r * x).collect();
        results.push(y);
        current = g;
    }

    Some((current, results))
}

// Chinese Remainder Theorem
// https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Existence_.28direct_construction.29
pub fn crt(numbers_with_remainders: Vec<(i64, i64)>) -> i64 {
    let full_product: i64 = numbers_with_remainders.iter().map(|n| n.0).product();
    let mut result = 0;

    for (n, offset) in numbers_with_remainders.iter() {
        let product_without_n = full_product / n;
        let (_, inv, _) = egcd(product_without_n, *n);
        result += offset * product_without_n * inv;
    }

    result.rem_euclid(full_product)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(20, 15), 5);
        assert_eq!(gcd(13, 17), 1);
    }

    #[test]
    fn test_egcd() {
        assert_eq!(egcd(20, 15), (5, 1, -1));
        assert_eq!(egcd(13, 17), (1, 4, -3));
    }

    #[test]
    fn test_egcd_mn() {
        assert_eq!(egcd_mn(&vec![20, 15, 10]), Some((5, vec![1, -1, 0])));
        assert_eq!(egcd_mn(&vec![19, 31, 59]), Some((1, vec![-13, 8, 0])));
    }
}

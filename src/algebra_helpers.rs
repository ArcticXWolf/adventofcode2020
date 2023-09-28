use itertools::Itertools;
use num_traits::Num;
use std::collections::HashMap;
use std::ops::{self, Index, IndexMut};
use std::{array, fmt};
use std::{
    fmt::Debug,
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};

// Taken and adapted from MIT-licensed code library lina: https://github.com/LukasKalbertodt/lina

pub trait Scalar:
    Num + Clone + Copy + Debug + AddAssign + SubAssign + MulAssign + DivAssign
{
}

impl<T> Scalar for T where
    T: Num + Clone + Copy + Debug + AddAssign + SubAssign + MulAssign + DivAssign
{
}

pub trait Float: Scalar + num_traits::Float + num_traits::FloatConst {}

impl<T> Float for T where T: Scalar + num_traits::Float + num_traits::FloatConst {}

#[repr(transparent)]
pub struct Point<T: Scalar, const N: usize>(pub(crate) [T; N]);

pub type Point2<T> = Point<T, 2>;

impl<T: Scalar> Point<T, 2> {
    pub fn new(x: T, y: T) -> Self {
        Self([x, y])
    }
}

pub type Point3<T> = Point<T, 3>;

impl<T: Scalar> Point<T, 3> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self([x, y, z])
    }
}

pub type Point4<T> = Point<T, 4>;

impl<T: Scalar> Point<T, 4> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self([x, y, z, w])
    }
}

impl<T: Scalar, const N: usize> Point<T, N> {
    pub fn zero() -> Self {
        std::array::from_fn(|_| T::zero()).into()
    }

    pub fn is_zero(&self) -> bool {
        self.0.iter().all(T::is_zero)
    }

    pub fn origin() -> Self {
        Self::zero()
    }

    pub fn filled(value: T) -> Self {
        std::array::from_fn(|_| value).into()
    }

    pub fn unit_in_dimension(dimension: usize) -> Self {
        std::array::from_fn(|i| if i == dimension { T::one() } else { T::zero() }).into()
    }

    pub fn unit_vectors() -> Vec<Self> {
        (0..N).map(|i| Self::unit_in_dimension(i)).collect()
    }

    pub fn directions() -> Vec<Self> {
        Self::unit_vectors()
            .into_iter()
            .map(|p| Point::zero() - p)
            .chain(Self::unit_vectors().into_iter())
            .collect()
    }

    pub fn directions_with_diagonals() -> Vec<Self> {
        let mut current_vectors = vec![Self::zero()];

        for i in 0..N {
            current_vectors = current_vectors
                .iter()
                .flat_map(|v| {
                    vec![
                        *v - Self::unit_in_dimension(i),
                        *v,
                        *v + Self::unit_in_dimension(i),
                    ]
                })
                .collect();
        }

        current_vectors.remove(usize::pow(3, N as u32) / 2); // remove identity

        current_vectors
    }

    pub fn distance2_from(self, other: Self) -> T {
        (self - other).length2()
    }

    pub fn distance_from(self, other: Self) -> T
    where
        T: Float,
    {
        (self - other).length()
    }

    pub fn length2(&self) -> T {
        self.0
            .iter()
            .map(|&c| c * c)
            .fold(T::zero(), |acc, e| acc + e)
    }

    pub fn length(&self) -> T
    where
        T: Float,
    {
        self.length2().sqrt()
    }

    pub fn vec_to(self, other: Self) -> Point<T, N> {
        other - self
    }
}

impl<T: Scalar, const N: usize> From<Point<T, N>> for [T; N] {
    fn from(value: Point<T, N>) -> Self {
        value.0
    }
}

impl<T: Scalar, const N: usize> From<[T; N]> for Point<T, N> {
    fn from(value: [T; N]) -> Self {
        Self(value)
    }
}

impl<T: Scalar, const N: usize> Index<usize> for Point<T, N> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: Scalar, const N: usize> IndexMut<usize> for Point<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: Scalar, const N: usize> fmt::Debug for Point<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point")?;
        write!(f, "[")?;
        for (i, e) in self.0.into_iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            e.fmt(f)?;
        }
        write!(f, "]")
    }
}

impl<T: Scalar, const N: usize> ops::Add<Point<T, N>> for Point<T, N> {
    type Output = Self;
    fn add(self, rhs: Point<T, N>) -> Self::Output {
        array::from_fn(|i| self[i] + rhs[i]).into()
    }
}

impl<T: Scalar, const N: usize> ops::AddAssign<Point<T, N>> for Point<T, N> {
    fn add_assign(&mut self, rhs: Point<T, N>) {
        for (lhs, rhs) in IntoIterator::into_iter(&mut self.0).zip(rhs.0) {
            *lhs += rhs;
        }
    }
}

impl<T: Scalar, const N: usize> ops::SubAssign<Point<T, N>> for Point<T, N> {
    fn sub_assign(&mut self, rhs: Point<T, N>) {
        for (lhs, rhs) in IntoIterator::into_iter(&mut self.0).zip(rhs.0) {
            *lhs -= rhs;
        }
    }
}

impl<T: Scalar, const N: usize> ops::Sub<Self> for Point<T, N> {
    type Output = Point<T, N>;
    fn sub(self, rhs: Self) -> Self::Output {
        array::from_fn(|i| self[i] - rhs[i]).into()
    }
}

impl<T: Scalar + std::hash::Hash, const N: usize> std::hash::Hash for Point<T, N> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl<T: Scalar, const N: usize> PartialEq for Point<T, N> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}
impl<T: Scalar + Eq, const N: usize> Eq for Point<T, N> {}

impl<T: Scalar, const N: usize> Clone for Point<T, N> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T: Scalar, const N: usize> Copy for Point<T, N> {}

#[derive(Debug, Clone)]
pub struct PointGrid<T: Scalar, const N: usize, U>(pub HashMap<Point<T, N>, U>);

impl<T: Scalar, const N: usize, U> Default for PointGrid<T, N, U> {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl<T: Scalar + std::hash::Hash + Eq, const N: usize, U> PointGrid<T, N, U> {
    pub fn insert(&mut self, p: Point<T, N>, value: U) {
        self.0.insert(p, value);
    }

    pub fn get(&self, p: &Point<T, N>) -> Option<&U> {
        self.0.get(p)
    }
}

impl<T: Scalar + Ord, const N: usize, U> PointGrid<T, N, U> {
    pub fn dimensions(&self) -> (Point<T, N>, Point<T, N>) {
        (
            Point {
                0: (0..N)
                    .map(|n| self.0.keys().map(|p| *p.0.get(n).unwrap()).min().unwrap())
                    .into_iter()
                    .collect_vec()
                    .try_into()
                    .unwrap(),
            },
            Point {
                0: (0..N)
                    .map(|n| self.0.keys().map(|p| *p.0.get(n).unwrap()).max().unwrap())
                    .into_iter()
                    .collect_vec()
                    .try_into()
                    .unwrap(),
            },
        )
    }
}

pub struct PointGridIterator<T: Scalar, const N: usize> {
    lower_bound: Point<T, N>,
    upper_bound: Point<T, N>,
    last: Point<T, N>,
}

impl<T: Scalar + PartialOrd, const N: usize> PointGridIterator<T, N> {
    pub fn new(lower_bound: Point<T, N>, upper_bound: Point<T, N>) -> Self {
        Self {
            lower_bound,
            upper_bound,
            last: lower_bound,
        }
    }
}

impl<T: Scalar + PartialOrd, const N: usize> Iterator for PointGridIterator<T, N> {
    type Item = Point<T, N>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.last == self.upper_bound {
            return None;
        }

        let result = self.last;
        for n in (0..N).rev() {
            if self.last.0[n] + T::one() >= self.upper_bound.0[n] {
                self.last.0[n] = self.lower_bound.0[n];
            } else {
                self.last.0[n] += T::one();
                return Some(result);
            }
        }
        self.last = self.upper_bound;
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_directions() {
        assert_eq!(
            Point2::<isize>::directions(),
            vec![
                Point2::new(-1, 0),
                Point2::new(0, -1),
                Point2::new(1, 0),
                Point2::new(0, 1)
            ]
        );
        assert_eq!(
            Point3::<isize>::directions(),
            vec![
                Point3::new(-1, 0, 0),
                Point3::new(0, -1, 0),
                Point3::new(0, 0, -1),
                Point3::new(1, 0, 0),
                Point3::new(0, 1, 0),
                Point3::new(0, 0, 1)
            ]
        );
    }

    #[test]
    fn test_directions_with_diagonals() {
        assert_eq!(
            Point2::<isize>::directions_with_diagonals(),
            vec![
                Point2::new(-1, -1),
                Point2::new(-1, 0),
                Point2::new(-1, 1),
                Point2::new(0, -1),
                Point2::new(0, 1),
                Point2::new(1, -1),
                Point2::new(1, 0),
                Point2::new(1, 1),
            ]
        );
        assert_eq!(
            Point3::<isize>::directions_with_diagonals(),
            vec![
                Point3::new(-1, -1, -1),
                Point3::new(-1, -1, 0),
                Point3::new(-1, -1, 1),
                Point3::new(-1, 0, -1),
                Point3::new(-1, 0, 0),
                Point3::new(-1, 0, 1),
                Point3::new(-1, 1, -1),
                Point3::new(-1, 1, 0),
                Point3::new(-1, 1, 1),
                Point3::new(0, -1, -1),
                Point3::new(0, -1, 0),
                Point3::new(0, -1, 1),
                Point3::new(0, 0, -1),
                Point3::new(0, 0, 1),
                Point3::new(0, 1, -1),
                Point3::new(0, 1, 0),
                Point3::new(0, 1, 1),
                Point3::new(1, -1, -1),
                Point3::new(1, -1, 0),
                Point3::new(1, -1, 1),
                Point3::new(1, 0, -1),
                Point3::new(1, 0, 0),
                Point3::new(1, 0, 1),
                Point3::new(1, 1, -1),
                Point3::new(1, 1, 0),
                Point3::new(1, 1, 1),
            ]
        );
    }

    #[test]
    fn test_point_grid_dimensions() {
        let mut pg: PointGrid<isize, 2, bool> = PointGrid::default();
        pg.insert(Point2::new(0, 0), true);
        pg.insert(Point2::new(-20, 20), true);
        pg.insert(Point2::new(20, -10), true);
        assert_eq!(
            pg.dimensions(),
            (Point2::new(-20, -10), Point2::new(20, 20))
        );
    }

    #[test]
    fn test_point_grid_iterator() {
        let pgi: PointGridIterator<isize, 2> =
            PointGridIterator::new(Point2::new(-2, -1), Point2::new(2, 3));
        assert_eq!(
            pgi.collect_vec(),
            vec![
                Point2::new(-2, -1),
                Point2::new(-2, 0),
                Point2::new(-2, 1),
                Point2::new(-2, 2),
                Point2::new(-1, -1),
                Point2::new(-1, 0),
                Point2::new(-1, 1),
                Point2::new(-1, 2),
                Point2::new(0, -1),
                Point2::new(0, 0),
                Point2::new(0, 1),
                Point2::new(0, 2),
                Point2::new(1, -1),
                Point2::new(1, 0),
                Point2::new(1, 1),
                Point2::new(1, 2),
            ]
        );
    }
}

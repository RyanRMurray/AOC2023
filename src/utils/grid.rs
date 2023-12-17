use std::collections::HashSet;
use std::fmt::Debug;
use std::vec::IntoIter;

use indexmap::IndexMap;
use itertools::Itertools;

use super::point::Pt;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<T: Copy, const DIMS: usize> {
    /// neighbour offsets for points in this N dimensions
    pub offsets: HashSet<Pt<DIMS>>,
    /// cardinal offsets for points in this N dimensions
    pub card_offsets: HashSet<Pt<DIMS>>,
    default_val: T,
    pub grid: IndexMap<Pt<DIMS>, T>,
}

impl<T: Default + Copy, const DIMS: usize> Default for Grid<T, DIMS> {
    fn default() -> Self {
        Self {
            offsets: Pt::<DIMS>::neighbour_offsets(),
            card_offsets: Pt::<DIMS>::card_offsets(),
            default_val: T::default(),
            grid: Default::default(),
        }
    }
}

impl<T: Default + Copy, const DIMS: usize> From<Vec<(Pt<DIMS>, T)>> for Grid<T, DIMS> {
    fn from(v: Vec<(Pt<DIMS>, T)>) -> Self {
        Self {
            offsets: Pt::<DIMS>::neighbour_offsets(),
            card_offsets: Pt::<DIMS>::card_offsets(),
            default_val: T::default(),
            grid: v.into_iter().collect(),
        }
    }
}

impl<T: Default + Copy, const DIMS: usize> From<Vec<(Vec<isize>, T)>> for Grid<T, DIMS> {
    fn from(v: Vec<(Vec<isize>, T)>) -> Self {
        Self {
            offsets: Pt::<DIMS>::neighbour_offsets(),
            card_offsets: Pt::<DIMS>::card_offsets(),
            default_val: T::default(),
            grid: v
                .into_iter()
                .map(|(k, v)| (Pt(k.try_into().unwrap()), v))
                .collect(),
        }
    }
}

#[allow(dead_code)]
impl<T: Copy, const DIMS: usize> Grid<T, DIMS> {
    /// get a value at the specified coordinates or the default
    pub fn get_def(&self, pt: &Pt<DIMS>) -> T {
        *self.grid.get(pt).unwrap_or(&self.default_val)
    }

    /// Change the default value
    pub fn set_default(&mut self, new_default: T) {
        self.default_val = new_default;
    }

    /// merge one grid into this one, using the specified merge_function
    pub fn merge(&mut self, other: Grid<T, DIMS>, merge_function: fn(&T, &T) -> T) {
        other.grid.into_iter().for_each(|(k, v)| {
            let new_val = merge_function(self.grid.get(&k).unwrap_or(&self.default_val), &v);
            self.grid.insert(k, new_val);
        });
    }

    /// apply a transformation to every point in a grid
    pub fn transform(mut self, transformation: fn(Pt<DIMS>) -> Pt<DIMS>) -> Self {
        let mut new_grid = IndexMap::default();
        self.grid.into_iter().for_each(|(k, v)| {
            new_grid.insert(transformation(k), v);
        });
        self.grid = new_grid;
        self
    }

    /// get the min and max values of each dimension
    pub fn bounds(&self) -> ([isize; DIMS], [isize; DIMS]) {
        let mut mins = [isize::MAX; DIMS];
        let mut maxs = [isize::MIN; DIMS];

        for k in self.grid.keys() {
            for i in 0..DIMS {
                mins[i] = isize::min(mins[i], k.0[i]);
                maxs[i] = isize::max(maxs[i], k.0[i]);
            }
        }
        (mins, maxs)
    }

    pub fn iter_linear(&self) -> IntoIter<&Pt<DIMS>> {
        self.grid.keys().sorted_by(|a, b| {
            (1..DIMS + 1)
                .rev()
                .map(|d| a.0[d - 1].cmp(&b.0[d - 1]))
                .find_or_last(|o| o.is_ne())
                .unwrap()
        })
    }
}

impl<T: Copy> Grid<T, 2> {
    /// print a 2d grid using a given function for representing points
    pub fn print(&self, to_printable: fn(T) -> char) -> String {
        let mut res = String::from("\n");

        let ([min_x, min_y], [max_x, max_y]) = self.bounds();

        for y in min_y..max_y + 1 {
            for x in min_x..max_x + 1 {
                res.push(to_printable(self.get_def(&Pt([x, y]))));
            }
            res.push('\n');
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Grid;
    use crate::utils::point::Pt;

    #[test]
    fn test_transform() {
        let expected = Grid::<i32, 2>::from(vec![
            (Pt([50, 50]), 10),
            (Pt([25, 50]), 204),
            (Pt([0, 0]), 66),
        ]);

        let input = Grid::<i32, 2>::from(vec![
            (Pt([25, 25]), 10),
            (Pt([0, 25]), 204),
            (Pt([-25, -25]), 66),
        ]);

        let result = input.transform(|pt| pt + Pt([25, 25]));

        assert_eq!(expected, result);
    }

    #[test]
    fn test_merge() {
        let mut target = Grid::<i32, 2>::from(vec![(Pt([50, 50]), 10), (Pt([25, 50]), 204)]);
        let to_merge = Grid::<i32, 2>::from(vec![(Pt([25, 50]), 5000), (Pt([0, 0]), 60)]);

        let expected = Grid::<i32, 2>::from(vec![
            (Pt([50, 50]), 10),
            (Pt([25, 50]), 5000),
            (Pt([0, 0]), 60),
        ]);

        target.merge(to_merge, |_, x| *x);

        assert_eq!(expected, target);
    }

    #[test]
    fn test_print() {
        let expected = r#"
123
456
789
"#
        .to_string();

        let grid = Grid::<u32, 2>::from(vec![
            (Pt([0, 0]), 1),
            (Pt([1, 0]), 2),
            (Pt([2, 0]), 3),
            (Pt([0, 1]), 4),
            (Pt([1, 1]), 5),
            (Pt([2, 1]), 6),
            (Pt([0, 2]), 7),
            (Pt([1, 2]), 8),
            (Pt([2, 2]), 9),
        ]);

        let result = grid.print(|x| char::from_digit(x, 10).unwrap());

        assert_eq!(expected, result);
    }

    #[test]
    fn test_linear_iter() {
        let expected = vec![&Pt([0, 0]), &Pt([1, 0]), &Pt([0, 1]), &Pt([1, 1])];

        let grid = Grid::<bool, 2>::from(vec![
            (Pt([1, 0]), true),
            (Pt([0, 0]), true),
            (Pt([0, 1]), true),
            (Pt([1, 1]), true),
        ]);

        let result: Vec<_> = grid.iter_linear().collect();

        assert_eq!(expected, result);
    }
}

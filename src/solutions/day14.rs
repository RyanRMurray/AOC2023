use std::collections::{HashMap, HashSet};

use crate::utils::{
    point::Pt,
    solver_types::{solve_linear, SolutionLinear},
};
use anyhow::Result;
use indexmap::IndexMap;
use itertools::Itertools;

pub struct Day14Solution {}

#[derive(Clone)]
struct Dish {
    cubes: HashSet<Pt<2>>,
    rocks: HashSet<Pt<2>>,
    max_x: isize,
    max_y: isize,
    orders: HashMap<Pt<2>, Vec<Pt<2>>>,
}

pub fn day14(input: &str) -> Result<f32> {
    solve_linear::<Day14Solution, _, _, _>(input)
}

impl Dish {
    fn roll_rocks(&mut self, direction: Pt<2>) {
        let order = self.orders.get(&direction).unwrap();

        order.iter().for_each(|pt| {
            if !self.rocks.contains(pt) {
                return;
            }
            self.rocks.remove(pt);
            let mut new_location = *pt;
            let mut next = new_location + direction;

            while !self.cubes.contains(&next)
                && !self.rocks.contains(&next)
                && next.0[0] != self.max_x
                && next.0[0] >= 0
                && next.0[1] != self.max_y
                && next.0[1] >= 0
            {
                new_location = next;
                next = new_location + direction;
            }
            self.rocks.insert(new_location);
        });
    }

    fn strain(&self) -> isize {
        self.rocks.iter().map(|Pt([_, iy])| self.max_y - iy).sum()
    }
}

impl SolutionLinear<Dish, isize, isize> for Day14Solution {
    fn load(input: &str) -> Result<Dish> {
        let mut x = 0;
        let mut y = 0;
        let mut rocks = HashSet::new();
        let mut cubes = HashSet::new();

        for l in input.lines() {
            x = 0;
            for c in l.chars() {
                match c {
                    'O' => {
                        rocks.insert(Pt([x, y]));
                    }
                    '#' => {
                        cubes.insert(Pt([x, y]));
                    }
                    _ => (),
                }
                x += 1;
            }
            y += 1;
        }

        let n_order = (0..y)
            .flat_map(|y| (0..x).map(move |x| Pt([x, y])))
            .collect_vec();

        let w_order = (0..x)
            .flat_map(|x| (0..y).map(move |y| Pt([x, y])))
            .collect_vec();

        // lol. lmao.
        let orders = HashMap::from([
            (Pt([0, -1]), n_order.clone()),
            (Pt([0, 1]), n_order.into_iter().rev().collect_vec()),
            (Pt([-1, 0]), w_order.clone()),
            (Pt([1, 0]), w_order.into_iter().rev().collect_vec()),
        ]);

        Ok(Dish {
            cubes,
            rocks,
            max_x: x,
            max_y: y,
            orders,
        })
    }

    fn part1(input: &mut Dish) -> Result<isize> {
        let mut p1 = input.clone();
        p1.roll_rocks(Pt([0, -1]));

        Ok(p1.strain())
    }

    // in short, at some point we enter a loop of possible states. we want to find when that loop starts, and its period
    // then we can just modulo to find where in a loop the millionth cycle should be
    fn part2(input: &mut Dish, _part_1_solution: isize) -> Result<isize> {
        let ds: [Pt<2>; 4] = [Pt([0, -1]), Pt([-1, 0]), Pt([0, 1]), Pt([1, 0])];
        let mut seen: IndexMap<Vec<Pt<2>>, (i32, isize)> = IndexMap::new();
        let mut repeat_starts = 0;
        let mut loop_size = 0;

        for x in 0..9999 {
            for d in ds {
                input.roll_rocks(d);
            }

            let mut key = input.rocks.clone().into_iter().collect_vec();
            key.sort();

            if let Some((v, _)) = seen.get(&key) {
                repeat_starts = *v;
                loop_size = x - repeat_starts;
                break;
            }
            seen.insert(key, (x, input.strain()));
        }

        let last = (repeat_starts - 1) + (1_000_000_000 - repeat_starts) % (loop_size);

        let sol = seen.iter().find(|(_, (idx, _))| *idx == last).unwrap().1 .1;

        Ok(sol)
    }
}

#[cfg(test)]
mod tests {
    use super::Day14Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
",
        136,
        64
    )]
    fn validate_day14(#[case] input: &str, #[case] expected_1: isize, #[case] expected_2: isize) {
        let mut input = Day14Solution::load(input).unwrap();

        let p1 = Day14Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);

        let p2 = Day14Solution::part2(&mut input, p1).unwrap();
        assert_eq!(expected_2, p2);
    }
}

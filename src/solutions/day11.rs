use std::collections::{HashMap, HashSet};

use crate::utils::{
    point::Pt,
    solver_types::{solve_linear, SolutionLinear},
};
use anyhow::Result;
use itertools::Itertools;

// TODO
pub struct Day11Solution {}

pub fn day11(input: &str) -> Result<f32> {
    solve_linear::<Day11Solution, _, _, _>(input)
}

fn create_offsets(
    seen: &HashSet<isize>,
    max_val: isize,
    expansion: isize,
) -> HashMap<isize, isize> {
    let mut offset = 0;
    (0..max_val + 1).fold(HashMap::new(), |mut hm, val| {
        if !seen.contains(&val) {
            offset += expansion - 1;
        }
        hm.insert(val, val + offset);
        hm
    })
}

fn do_thing(input: &[Pt<2>], expansion: isize) -> usize {
    // find unseen vecs
    let mut max_x: isize = 0;
    let mut max_y: isize = 0;
    let mut seen_x = HashSet::new();
    let mut seen_y = HashSet::new();

    input.iter().for_each(|Pt([x, y])| {
        seen_x.insert(*x);
        seen_y.insert(*y);
        max_x = max_x.max(*x);
        max_y = max_y.max(*y);
    });

    let map_x = create_offsets(&seen_x, max_x, expansion);
    let map_y = create_offsets(&seen_y, max_y, expansion);

    // expand galaxy
    let expanded = input
        .iter()
        .map(|Pt([x, y])| Pt([*map_x.get(x).unwrap(), *map_y.get(y).unwrap()]))
        .collect_vec();

    let ixs = (0..expanded.len()).flat_map(|x| (x + 1..expanded.len()).map(move |y| (x, y)));

    ixs.map(|(a, b)| {
        let Pt([ax, ay]) = expanded.get(a).unwrap();
        let Pt([bx, by]) = expanded.get(b).unwrap();

        ((ax.max(bx) - ax.min(bx)) + (ay.max(by) - ay.min(by))) as usize
    })
    .sum()
}

impl SolutionLinear<Vec<Pt<2>>, usize, usize> for Day11Solution {
    fn load(input: &str) -> Result<Vec<Pt<2>>> {
        Ok((0..)
            .zip(input.lines())
            .flat_map(|(y, l)| {
                (0..).zip(l.chars()).map(move |(x, c)| match c {
                    '#' => Some(Pt([x, y])),
                    _ => None,
                })
            })
            .flatten()
            .collect_vec())
    }

    fn part1(input: &mut Vec<Pt<2>>) -> Result<usize> {
        Ok(do_thing(input, 2))
    }

    fn part2(input: &mut Vec<Pt<2>>, _part_1_solution: usize) -> Result<usize> {
        Ok(do_thing(input, 1_000_000))
    }
}

#[cfg(test)]
mod tests {
    use super::Day11Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
",
        374,
        82000210
    )]
    fn validate_day11(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let mut input = Day11Solution::load(input).unwrap();

        let p1 = Day11Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);

        let p2 = Day11Solution::part2(&mut input, p1).unwrap();
        assert_eq!(expected_2, p2);
    }
}

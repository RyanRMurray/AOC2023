use std::collections::{HashMap, VecDeque};

use crate::utils::{
    grid::Grid,
    load_input::load_2d_grid,
    point::Pt,
    solver_types::{solve_linear, SolutionLinear},
};
use anyhow::{Ok, Result};
use indexmap::IndexMap;
use itertools::Itertools;

// TODO
pub struct Day17Solution {}

pub fn day17(input: &str) -> Result<f32> {
    solve_linear::<Day17Solution, _, _, _>(input)
}

impl SolutionLinear<IndexMap<(Pt<2>, Pt<2>), usize>, usize, usize> for Day17Solution {
    fn load(input: &str) -> Result<IndexMap<(Pt<2>, Pt<2>), usize>> {
        let g: Grid<usize, 2> = load_2d_grid(input, |v| v.to_digit(10).unwrap() as usize);
        let adjs = Pt::<2>::card_offsets();

        Ok(g.grid
            .keys()
            .flat_map(|pt| {
                adjs.iter().filter_map(|d| {
                    let next = *pt + *d;
                    if g.grid.contains_key(&next) {
                        Some(((*pt, next), *g.grid.get(&next).unwrap()))
                    } else {
                        None
                    }
                })
            })
            .collect())
    }

    fn part1(input: &mut IndexMap<(Pt<2>, Pt<2>), usize>) -> Result<usize> {
        let target = input.keys().last().unwrap().0;
        let dirs = [Pt([0, -1]), Pt([0, 1]), Pt([-1, 0]), Pt([1, 0])];
        // (heat, coord, steps in a direction) - points to explore
        let mut ptrs = VecDeque::from([(0, (Pt([-1, 0]), Pt([0, 0])), [0, 0, 0, 0])]);
        // points we've expanded from
        let mut visited: HashMap<(Pt<2>, Pt<2>, [usize; 4]), usize> = HashMap::new();
        let mut max: usize = 1000; // this is a bodge, deal with it

        while let Some(pt) = ptrs.pop_front() {
            if visited
                .get(&(pt.1 .0, pt.1 .1, pt.2))
                .is_some_and(|v| v < &pt.0)
            {
                continue;
            }
            visited.insert((pt.1 .0, pt.1 .1, pt.2), pt.0);
            if pt.0 > max {
                continue;
            }
            if pt.1 .1 == target {
                max = max.max(pt.0);
                continue;
            }

            let ns = (0..4).filter_map(|i| {
                let neighbour = pt.1 .1 + dirs[i];
                if input.contains_key(&(pt.1 .1, neighbour)) && pt.2[i] < 3 && neighbour != pt.1 .0
                {
                    let dist = pt.0 + input.get(&(pt.1 .1, neighbour)).unwrap();
                    let steps = (0..4)
                        .map(|j| if j == i { pt.2[i] + 1 } else { 0 })
                        .collect_vec()
                        .try_into()
                        .unwrap();
                    Some((dist, (pt.1 .1, neighbour), steps))
                } else {
                    None
                }
            });
            for n in ns {
                //overwrite to-visit if this would be closer
                if let Some(i) = ptrs
                    .iter()
                    .position(|(_, pt, sts)| pt == &n.1 && sts == &n.2)
                {
                    if ptrs[i].0 > n.0 {
                        ptrs[i] = n;
                    }
                } else {
                    // place at sorted location
                    let i = ptrs
                        .binary_search_by(|(d, _, _)| d.cmp(&n.0))
                        .unwrap_or_else(|i| i);
                    ptrs.insert(i, n);
                }
            }
        }

        Ok(*visited
            .iter()
            .filter(|((_, end, _), _)| end == &target)
            .map(|(_, v)| v)
            .min()
            .unwrap())
    }

    fn part2(
        _input: &mut IndexMap<(Pt<2>, Pt<2>), usize>,
        _part_1_solution: usize,
    ) -> Result<usize> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Day17Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[ignore = "delete to test solution"]
    #[case(
        "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
",
        102,
        2
    )]
    fn validate_day17(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let mut input = Day17Solution::load(input).unwrap();

        let p1 = Day17Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);

        let p2 = Day17Solution::part2(&mut input, p1).unwrap();
        assert_eq!(expected_2, p2);
    }
}

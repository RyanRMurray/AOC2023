use std::fmt::Debug;

use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::Result;
use itertools::{repeat_n, Itertools};

// TODO
pub struct Day12Solution {}

pub fn day12(input: &str) -> Result<f32> {
    solve_linear::<Day12Solution, _, _, _>(input)
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Gear {
    Operational,
    Broken,
    Unknown,
}

impl Debug for Gear {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Gear::Operational => ".",
                Gear::Broken => "#",
                Gear::Unknown => "?",
            }
        )
    }
}

fn to_nums(gears: &[Gear]) -> Vec<usize> {
    gears
        .iter()
        .group_by(|g| **g)
        .into_iter()
        .filter_map(|(ty, gr)| match ty {
            Gear::Broken => Some(gr.count()),
            _ => None,
        })
        .collect_vec()
}

fn generate_possible_vecs(gears: &Vec<Gear>, broken: usize) -> Vec<Vec<Gear>> {
    let unknowns = gears.iter().filter(|g| **g == Gear::Unknown).count();
    let recorded_broken = gears.iter().filter(|g| **g == Gear::Broken).count();
    println!("{:?} - {:?} - {:?}", gears, broken, recorded_broken);

    repeat_n([Gear::Broken, Gear::Operational], unknowns)
        .multi_cartesian_product()
        .filter_map(|mut replacements| {
            if replacements.iter().filter(|r| **r == Gear::Broken).count() != broken - recorded_broken
            {
                None
            } else {
                Some(
                    gears
                        .iter()
                        .map(|g| match g {
                            Gear::Unknown => replacements.pop().unwrap(),
                            other => *other,
                        })
                        .collect_vec(),
                )
            }
        })
        .collect_vec()
}

impl SolutionLinear<Vec<(Vec<Gear>, Vec<usize>)>, usize, usize> for Day12Solution {
    fn load(input: &str) -> Result<Vec<(Vec<Gear>, Vec<usize>)>> {
        Ok(input
            .lines()
            .map(|l| {
                let (gears, nums) = l.split_once(' ').unwrap();
                (
                    gears
                        .chars()
                        .map(|c| match c {
                            '.' => Gear::Operational,
                            '#' => Gear::Broken,
                            _ => Gear::Unknown,
                        })
                        .collect_vec(),
                    nums.split(',').map(|v| v.parse().unwrap()).collect_vec(),
                )
            })
            .collect_vec())
    }

    fn part1(input: &mut Vec<(Vec<Gear>, Vec<usize>)>) -> Result<usize> {
        Ok(input
            .iter()
            .map(|(gears, nums)| {
                let broken = nums.iter().sum();
                generate_possible_vecs(gears, broken)
                    .iter()
                    .filter(|gs| to_nums(gs) == *nums)
                    .count()
            })
            .sum())
    }

    fn part2(input: &mut Vec<(Vec<Gear>, Vec<usize>)>, _part_1_solution: usize) -> Result<usize> {
        Ok(input
            .iter()
            .map(|(gears, nums)| {
                #[allow(unstable_name_collisions)]
                let unfolded_gears = repeat_n(gears, 5)
                    .intersperse(&vec![Gear::Unknown])
                    .flatten()
                    .map(|v| *v)
                    .collect_vec();
                let unfolded_nums = nums.repeat(5);
                let broken = unfolded_nums.iter().sum();

                generate_possible_vecs(&unfolded_gears, broken)
                    .iter()
                    .filter(|gs| to_nums(gs) == *unfolded_nums)
                    .count()
            })
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::Day12Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
",
        21,
        525152
    )]
    fn validate_day12(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let mut input = Day12Solution::load(input).unwrap();

        let p1 = Day12Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);

        let p2 = Day12Solution::part2(&mut input, p1).unwrap();
        assert_eq!(expected_2, p2);
    }
}

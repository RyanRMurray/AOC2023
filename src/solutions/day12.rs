use std::{collections::HashMap, fmt::Debug};

use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::Result;
use itertools::{repeat_n, Itertools};

// very messy dynamic approach - essentially, recur from 0 to end of a gear list:
// if we've fit the pattern, return 1
// if we cannot finish the pattern, return 0
// if we hit a #, make sure it and subsequent #'s fits the pattern else return 0
// when we hit a ?, recur down two branches: one for the ? being a #, one for the ? being a .
// we only memoize interesting paths (branching), but this brings the runtime down from possibly hours to 31ms on a real input!
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

// check if a run of broken can fit starting from `start`
fn fits_run(gears: &[Gear], start: usize, run: usize) -> bool {
    for off in 0..run {
        if gears[start + off] == Gear::Operational {
            return false;
        }
    }

    if start + run >= gears.len() {
        return true;
    }
    gears[start + run] != Gear::Broken
}

fn recursive_solve(
    gears: &[Gear],
    memo: &mut HashMap<(usize, Vec<usize>), usize>,
    idx: usize,
    remaining_runs: Vec<usize>,
) -> usize {
    if let Some(v) = memo.get(&(idx, remaining_runs.clone())) {
        return *v;
    }

    // if we're out of gears but have runs remaining, fail
    if idx >= gears.len() {
        if remaining_runs.is_empty() {
            return 1;
        } else {
            return 0;
        }
    }

    if remaining_runs.is_empty() {
        // if we're out of runs, check if there's any gears left
        if gears[idx] == Gear::Broken {
            return 0;
        }
        return recursive_solve(gears, memo, idx + 1, remaining_runs);
    }

    if remaining_runs.iter().sum::<usize>() > gears.len() - idx {
        // if we dont have enough gears to satisfy the runs
        return 0;
    }

    // if we have runs,
    match gears[idx] {
        Gear::Operational => recursive_solve(gears, memo, idx + 1, remaining_runs), // just continue
        Gear::Broken => {
            // if we can fit the next run here, continue. otherwise, fail
            let (run, runs) = remaining_runs.split_first().unwrap();
            if fits_run(gears, idx, *run) {
                recursive_solve(gears, memo, idx + run + 1, runs.to_vec())
            } else {
                0
            }
        }
        Gear::Unknown => {
            // branch - either resolve to `.` or start new run
            // if we can fit the next run here, continue. otherwise, fail
            let (run, runs) = remaining_runs.split_first().unwrap();
            let is_hash = if fits_run(gears, idx, *run) {
                recursive_solve(gears, memo, idx + run + 1, runs.to_vec())
            } else {
                0
            };

            let res = is_hash + recursive_solve(gears, memo, idx + 1, remaining_runs.clone());

            memo.insert((idx, remaining_runs), res);
            res
        }
    }
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
                let mut memo = HashMap::new();
                recursive_solve(gears, &mut memo, 0, nums.to_vec())
            })
            .sum())
    }

    fn part2(input: &mut Vec<(Vec<Gear>, Vec<usize>)>, _part_1_solution: usize) -> Result<usize> {
        Ok(input
            .iter()
            .map(|(gears, nums)| {
                let mut memo = HashMap::new();
                #[allow(unstable_name_collisions)]
                let unfolded_gears = repeat_n(gears, 5)
                    .intersperse(&vec![Gear::Unknown])
                    .flatten()
                    .copied()
                    .collect_vec();
                let unfolded_nums = nums.repeat(5);
                recursive_solve(&unfolded_gears, &mut memo, 0, unfolded_nums)
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

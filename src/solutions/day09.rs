use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::Result;
use itertools::Itertools;

// TODO
pub struct Day09Solution {}

pub fn day09(input: &str) -> Result<f32> {
    solve_linear::<Day09Solution, _, _, _>(input)
}

fn extrapolate(seq: &[isize], forward: bool) -> isize {
    let mut reduced = seq.to_owned();
    if !forward {
        reduced.reverse();
    }
    let mut next = *reduced.last().unwrap();

    while reduced.iter().any(|v| *v != 0) {
        reduced = reduced
            .windows(2)
            .map(|v| {
                if v.len() != 2 {
                    panic!("invalid sequence")
                } else {
                    v[1] - v[0]
                }
            })
            .collect_vec();
        next += reduced.last().unwrap();
    }

    next
}

impl SolutionLinear<Vec<Vec<isize>>, isize, isize> for Day09Solution {
    fn load(input: &str) -> Result<Vec<Vec<isize>>> {
        Ok(input
            .lines()
            .map(|l| {
                l.split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect_vec()
            })
            .collect_vec())
    }

    fn part1(input: &mut Vec<Vec<isize>>) -> Result<isize> {
        Ok(input.iter().map(|v| extrapolate(v, true)).sum())
    }

    fn part2(input: &mut Vec<Vec<isize>>, _part_1_solution: isize) -> Result<isize> {
        Ok(input.iter().map(|v| extrapolate(v, false)).sum())
    }
}

#[cfg(test)]
mod tests {
    use super::Day09Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
",
        114,
        2
    )]
    fn validate(#[case] input: &str, #[case] expected_1: isize, #[case] expected_2: isize) {
        let mut input = Day09Solution::load(input).unwrap();

        let p1 = Day09Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);

        let p2 = Day09Solution::part2(&mut input, p1).unwrap();
        assert_eq!(expected_2, p2);
    }
}

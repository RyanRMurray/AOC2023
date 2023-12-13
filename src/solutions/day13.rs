use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::Result;
use itertools::Itertools;

// TODO
pub struct Day13Solution {}

pub fn day13(input: &str) -> Result<f32> {
    solve_linear::<Day13Solution, _, _, _>(input)
}

type Mirror = Vec<Vec<bool>>;

// stolen from https://stackoverflow.com/a/64499219/22029215
fn transpose(mirror: Mirror) -> Mirror {
    let len = mirror[0].len();
    let mut iters: Vec<_> = mirror.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| iters.iter_mut().map(|n| n.next().unwrap()).collect_vec())
        .collect_vec()
}

// measure likeness between 0:1.., ..1:2.. etc. return left side or none if no reflection found
fn measure_reflection(mirror: &Mirror) -> Option<usize> {
    for top in 0..mirror.len() - 1 {
        if (0..top + 1)
            .rev()
            .zip(top + 1..mirror.len())
            .all(|(a, b)| mirror[a] == mirror[b])
        {
            return Some(top + 1);
        }
    }
    None
}

// take the first instance where exactly one difference stops a reflection from being valid
fn smudged_reflection(mirror: &Mirror) -> Option<usize> {
    for top in 0..mirror.len() - 1 {
        let diffs: usize = (0..top + 1)
            .rev()
            .zip(top + 1..mirror.len())
            .map(|(a, b)| {
                mirror[a]
                    .iter()
                    .zip(mirror[b].clone())
                    .filter(|(aa, bb)| **aa != *bb)
                    .count()
            })
            .sum();
        if diffs == 1 {
            return Some(top + 1);
        }
    }
    None
}

impl SolutionLinear<Vec<Mirror>, usize, usize> for Day13Solution {
    fn load(input: &str) -> Result<Vec<Mirror>> {
        Ok(input
            .split("\n\n")
            .map(|square| {
                square
                    .lines()
                    .map(|l| l.chars().map(|c| c == '#').collect_vec())
                    .collect_vec()
            })
            .collect_vec())
    }

    fn part1(input: &mut Vec<Mirror>) -> Result<usize> {
        Ok(input
            .iter()
            .map(|m| {
                if let Some(v) = measure_reflection(m) {
                    v * 100
                } else {
                    measure_reflection(&transpose(m.to_vec())).unwrap()
                }
            })
            .sum())
    }

    fn part2(input: &mut Vec<Mirror>, _part_1_solution: usize) -> Result<usize> {
        Ok(input
            .iter()
            .map(|m| {
                if let Some(v) = smudged_reflection(m) {
                    v * 100
                } else {
                    smudged_reflection(&transpose(m.to_vec())).unwrap()
                }
            })
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::Day13Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
",
        405,
        400
    )]
    fn validate_day13(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let mut input = Day13Solution::load(input).unwrap();

        let p1 = Day13Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);

        let p2 = Day13Solution::part2(&mut input, p1).unwrap();
        assert_eq!(expected_2, p2);
    }
}

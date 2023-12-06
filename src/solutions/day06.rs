use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::Result;

pub struct Day06Solution {}

pub fn day06(input: &str) -> Result<f32> {
    solve_linear::<Day06Solution, _, _, _>(input)
}

fn wait_this_is_just_quadratic_formulas(t: usize, d: usize) -> (usize, usize) {
    let s = ((t * t - 4 * d) as f64).sqrt();

    (
        (((t as f64 + s) / 2.0).ceil() - 1.0) as usize,
        (((t as f64 - s) / 2.0).floor() + 1.0) as usize,
    )
}

impl SolutionLinear<Vec<(usize, usize)>, usize, usize> for Day06Solution {
    fn load(input: &str) -> Result<Vec<(usize, usize)>> {
        let (times, distances) = input.split_once('\n').unwrap();
        Ok(times
            .split_whitespace()
            .skip(1)
            .map(|v| v.parse::<usize>().unwrap())
            .zip(
                distances
                    .split_whitespace()
                    .skip(1)
                    .map(|v| v.parse::<usize>().unwrap()),
            )
            .collect())
    }

    fn part1(input: &mut Vec<(usize, usize)>) -> Result<usize> {
        Ok(input.iter().fold(1, |prod, (t, d)| {
            let (a, b) = wait_this_is_just_quadratic_formulas(*t, *d);
            prod * (a - b + 1)
        }))
    }

    fn part2(input: &mut Vec<(usize, usize)>, _part_1_solution: usize) -> Result<usize> {
        let (t, d) = input.iter().fold((0, 0), |(t, d), (tx, dx)| {
            (
                t * 10_usize.pow(tx.checked_ilog10().unwrap() + 1) + tx,
                d * 10_usize.pow(dx.checked_ilog10().unwrap() + 1) + dx,
            )
        });

        let (a, b) = wait_this_is_just_quadratic_formulas(t, d);
        Ok(a - b + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::Day06Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "Time:      7  15   30
Distance:  9  40  200
",
        288,
        71503
    )]
    fn validate(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let mut input = Day06Solution::load(input).unwrap();

        let p1 = Day06Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);

        let p2 = Day06Solution::part2(&mut input, p1).unwrap();
        assert_eq!(expected_2, p2);
    }
}

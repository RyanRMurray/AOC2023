use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::Result;

// TODO
pub struct Day24Solution {}

pub fn day24(input: &str) -> Result<f32> {
    solve_linear::<Day24Solution, _, _, _>(input)
}

impl SolutionLinear<Vec<usize>, usize, usize> for Day24Solution {
    fn load(_input: &str) -> Result<Vec<usize>> {
        todo!()
    }

    fn part1(_input: &mut Vec<usize>) -> Result<usize> {
        todo!()
    }

    fn part2(_input: &mut Vec<usize>, _part_1_solution: usize) -> Result<usize> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Day24Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case("input", 1, 2)]
    fn validate(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let mut input = Day24Solution::load(input).unwrap();
        let p1 = Day24Solution::part1(&mut input).unwrap();
        let p2 = Day24Solution::part2(&mut input, p1).unwrap();

        assert_eq!(expected_1, p1);
        assert_eq!(expected_2, p2);
    }
}

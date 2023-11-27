use crate::utils::solver_types::SolutionLinear;
use anyhow::Result;

// Example:
// input: [1,2,3,4,5]
// part 1: sum up these numbers
// part 2: multiply the result of part 1 by the number of numbers in the input
pub struct ExampleSolutionLinear {}

impl SolutionLinear<Vec<usize>, usize, usize> for ExampleSolutionLinear {
    fn load(input: &str) -> Result<Vec<usize>> {
        Ok(input
            .replace(['[', ']'], "")
            .split(',')
            .map(|num| num.parse().unwrap())
            .collect())
    }

    fn part1(input: &mut Vec<usize>) -> Result<usize> {
        Ok(input.iter().sum())
    }

    fn part2(input: &mut Vec<usize>, part_1_solution: usize) -> Result<usize> {
        Ok(input.len() * part_1_solution)
    }
}

#[cfg(test)]
mod tests {
    use super::ExampleSolutionLinear;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case("[1,2,3]", 6, 18)]
    #[case("[0,7,13,20,1,100]", 141, 846)]
    #[case("[6000]", 6000, 6000)]
    fn validate_linear(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let mut input = ExampleSolutionLinear::load(input).unwrap();
        let p1 = ExampleSolutionLinear::part1(&mut input).unwrap();
        let p2 = ExampleSolutionLinear::part2(&mut input, p1).unwrap();

        assert_eq!(expected_1, p1);
        assert_eq!(expected_2, p2);
    }
}

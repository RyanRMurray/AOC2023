use crate::utils::solver_types::SolutionSimultaneous;
use anyhow::anyhow;
use itertools::Itertools;

// Example:
// input: [6,5,4,2,3,5,8]
// part 1: get the first number that's higher than the previous
// part 2: get the number after the first number that's higher than the previous

#[derive(Default)]
pub struct ExampleSolutionSimultaneous {}

impl SolutionSimultaneous<Vec<usize>, usize, usize> for ExampleSolutionSimultaneous {
    fn load(input: &str) -> anyhow::Result<Vec<usize>> {
        Ok(input
            .replace(['[', ']'], "")
            .split(',')
            .map(|num| num.parse().unwrap())
            .collect())
    }

    fn solve(input: Vec<usize>) -> anyhow::Result<(usize, usize)> {
        let mut prev: usize = usize::MAX;
        for (x, y) in input.iter().tuple_windows() {
            if x > &prev {
                return Ok((*x, *y));
            }
            prev = *x;
        }
        Err(anyhow!("Invalid input - check and try again"))
    }
}

#[cfg(test)]
mod tests {
    use super::ExampleSolutionSimultaneous;
    use crate::utils::solver_types::SolutionSimultaneous;
    use rstest::rstest;

    #[rstest]
    #[case("[6,5,4,2,3,5,8]", 3, 5)]
    #[case("[1,2,6,4,100]", 2, 6)]
    #[case("[5,4,3,1,2,7]", 2, 7)]
    fn validate_simul(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let input = ExampleSolutionSimultaneous::load(input).unwrap();
        let (p1, p2) = ExampleSolutionSimultaneous::solve(input).unwrap();

        assert_eq!(expected_1, p1);
        assert_eq!(expected_2, p2);
    }
}

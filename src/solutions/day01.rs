use crate::utils::{
    load_input::load_lines,
    solver_types::{solve_linear, SolutionLinear},
};
use anyhow::Result;
use fancy_regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref RE_PART2: Regex =
        Regex::new(r"(?=(\d|one|two|three|four|five|six|seven|eight|nine|zero))").unwrap();
}
pub struct Day01Solution {}

pub fn day01(input: &str) -> Result<f32> {
    solve_linear::<Day01Solution, _, _, _>(input)
}

impl SolutionLinear<Vec<String>, u32, u32> for Day01Solution {
    fn load(input: &str) -> Result<Vec<String>> {
        Ok(load_lines(input, |l| l.to_owned()))
    }

    fn part1(input: &mut Vec<String>) -> Result<u32> {
        Ok(input
            .iter()
            .map(|l| l.chars().filter(|c| c.is_ascii_digit()).collect())
            .map(|ns: Vec<char>| {
                10 * ns.first().unwrap().to_digit(10).unwrap()
                    + ns.last().unwrap().to_digit(10).unwrap()
            })
            .sum())
    }

    fn part2(input: &mut Vec<String>, _part_1_solution: u32) -> Result<u32> {
        Ok(input
            .iter()
            .map(|l| {
                RE_PART2
                    .captures_iter(l)
                    .map(|c| {
                        let s = c.unwrap().get(1).unwrap().as_str();
                        match s {
                            "one" => 1,
                            "two" => 2,
                            "three" => 3,
                            "four" => 4,
                            "five" => 5,
                            "six" => 6,
                            "seven" => 7,
                            "eight" => 8,
                            "nine" => 9,
                            _ => s.parse().unwrap(),
                        }
                    })
                    .collect()
            })
            .map(|ns: Vec<u32>| 10 * ns.first().unwrap() + ns.last().unwrap())
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::Day01Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
",
        142,
        142
    )]
    #[case( //note: slightly bungled - the second line has no integers, so i've added a zero.
        "two1nine
eightwo0three
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
",
        209,
        281
    )]
    fn validate(#[case] input: &str, #[case] expected_1: u32, #[case] expected_2: u32) {
        let mut input = Day01Solution::load(input).unwrap();
        let p1 = Day01Solution::part1(&mut input).unwrap();
        let p2 = Day01Solution::part2(&mut input, p1).unwrap();

        assert_eq!(expected_1, p1);
        assert_eq!(expected_2, p2);
    }
}

use std::collections::HashSet;

use crate::utils::{
    load_input::load_lines,
    solver_types::{solve_linear, SolutionLinear},
};
use anyhow::Result;

// TODO
pub struct Day04Solution {}

pub fn day04(input: &str) -> Result<f32> {
    solve_linear::<Day04Solution, _, _, _>(input)
}

struct Card {
    winning: HashSet<usize>,
    nums: HashSet<usize>,
    wins: usize,
}

fn parse_nums(nums: &str) -> HashSet<usize> {
    nums.split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn parse_card(card: &str) -> Card {
    let (_, ns) = card.split_once(':').unwrap();
    let (winning, nums) = ns.split_once('|').unwrap();

    Card {
        winning: parse_nums(winning),
        nums: parse_nums(nums),
        wins: 0,
    }
}

impl SolutionLinear<Vec<Card>, usize, usize> for Day04Solution {
    fn load(input: &str) -> Result<Vec<Card>> {
        Ok(load_lines(input, parse_card))
    }

    fn part1(input: &mut Vec<Card>) -> Result<usize> {
        Ok(input
            .iter_mut()
            .map(|c| {
                let count = c.winning.intersection(&c.nums).count();
                c.wins = count;
                if count > 0 {
                    usize::pow(2, (count - 1).try_into().unwrap())
                } else {
                    0
                }
            })
            .sum())
    }

    fn part2(input: &mut Vec<Card>, _part_1_solution: usize) -> Result<usize> {
        let mut counts = vec![1; input.len()];

        for i in 0..input.len() {
            let wins = input.get(i).unwrap().wins;
            (i + 1..i + 1 + wins).for_each(|j| counts[j] += counts[i]);
        }

        Ok(counts.into_iter().sum())
    }
}

#[cfg(test)]
mod tests {
    use super::Day04Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
",
        13,
        30
    )]
    fn validate(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let mut input = Day04Solution::load(input).unwrap();
        let p1 = Day04Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);

        let p2 = Day04Solution::part2(&mut input, p1).unwrap();

        assert_eq!(expected_2, p2);
    }
}

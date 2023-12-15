use std::{collections::HashMap, mem::replace};

use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::Result;
use itertools::Itertools;

// TODO
pub struct Day15Solution {}

pub fn day15(input: &str) -> Result<f32> {
    solve_linear::<Day15Solution, _, _, _>(input)
}

fn hash(s: &[char]) -> u8 {
    s.iter().fold(0u8, |hash, c| {
        (hash.overflowing_add(*c as u8).0).overflowing_mul(17).0
    })
}

impl SolutionLinear<Vec<Vec<char>>, usize, usize> for Day15Solution {
    fn load(input: &str) -> Result<Vec<Vec<char>>> {
        Ok(input
            .split(',')
            .map(|s| s.chars().collect_vec())
            .collect_vec())
    }

    fn part1(input: &mut Vec<Vec<char>>) -> Result<usize> {
        let mut note = HashMap::new();

        Ok(input
            .iter()
            .map(|s| {
                if let Some(v) = note.get(s) {
                    *v
                } else {
                    let v = hash(s) as usize;
                    note.insert(s, v);
                    v as usize
                }
            })
            .sum())
    }

    fn part2(input: &mut Vec<Vec<char>>, _part_1_solution: usize) -> Result<usize> {
        let mut boxes: HashMap<u8, Vec<(String, usize)>> = HashMap::new();

        for i in input {
            match i.last() {
                Some('-') => {
                    let tag = &i[0..i.len() - 1];
                    let b = hash(tag);
                    let contents = boxes.entry(b).or_default();
                    let tag_str = tag.iter().collect::<String>();
                    contents.retain(|(t, _)| t != &tag_str);
                }
                _ => {
                    let num = i.pop().unwrap().to_digit(10).unwrap() as usize;
                    i.pop();
                    let b = hash(i);
                    let contents = boxes.entry(b).or_default();
                    let tag_str = i.iter().collect::<String>();
                    match contents.iter().position(|(t, _)| t == &tag_str) {
                        Some(idx) => {
                            let _ = replace(&mut contents[idx], (tag_str, num));
                        }
                        None => contents.push((tag_str, num)),
                    }
                }
            }
        }

        Ok((0..255)
            .map(|idx| {
                if let Some(lenses) = boxes.get(&idx) {
                    if lenses.is_empty() {
                        return 0;
                    }
                    lenses
                        .iter()
                        .enumerate()
                        .map(|(i, (_, power))| (idx as usize + 1) * (i + 1) * power)
                        .sum()
                } else {
                    0
                }
            })
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::Day15Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7", 1320, 145)]
    fn validate_day15(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let mut input = Day15Solution::load(input).unwrap();

        let p1 = Day15Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);

        let p2 = Day15Solution::part2(&mut input, p1).unwrap();
        assert_eq!(expected_2, p2);
    }
}

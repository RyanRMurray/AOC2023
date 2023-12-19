use std::{cmp::Ordering, collections::HashMap};

use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::Result;
use itertools::Itertools;

// TODO
pub struct Day19Solution {}

pub fn day19(input: &str) -> Result<f32> {
    solve_linear::<Day19Solution, _, _, _>(input)
}

type Ranges = [(usize, usize); 4];
type Obj = [usize; 4];
#[derive(Debug)]
enum Action {
    To(String),
    Reject,
    Accept,
}
#[derive(Debug)]
enum Instr {
    Cmp(usize, Ordering, usize, Action),
    Just(Action),
}

const IDX: [char; 4] = ['x', 'm', 'a', 's'];

fn analyze(instrs: &HashMap<String, Vec<Instr>>, obj: Obj) -> Option<Obj> {
    let mut i = "in".to_string();

    loop {
        for ii in instrs.get(&i).unwrap() {
            match ii {
                Instr::Cmp(idx, ord, val, a) => {
                    let res = match ord {
                        Ordering::Less => obj[*idx] < *val,
                        Ordering::Greater => obj[*idx] > *val,
                        Ordering::Equal => panic!("unexpected comp"),
                    };
                    if res {
                        match a {
                            Action::To(new_i) => {
                                i = new_i.clone();
                                break;
                            }
                            Action::Reject => return None,
                            Action::Accept => return Some(obj),
                        }
                    }
                }
                Instr::Just(a) => match a {
                    Action::To(new_i) => {
                        i = new_i.clone();
                        break;
                    }
                    Action::Reject => return None,
                    Action::Accept => return Some(obj),
                },
            }
        }
    }
}

// left = matches, right = doesnt match
fn split_range(ranges: Ranges, idx: usize, val: usize, cmp: Ordering) -> (Ranges, Ranges) {
    let mut left = ranges;
    let mut right = ranges;

    match cmp {
        Ordering::Less => {
            left[idx] = (left[idx].0, val - 1);
            right[idx] = (val, right[idx].1);
        }
        Ordering::Greater => {
            left[idx] = (val + 1, left[idx].1);
            right[idx] = (right[idx].0, val);
        }
        Ordering::Equal => panic!("unexpected operand"),
    }

    (left, right)
}

fn sum_items(ranges: Ranges) -> usize {
    ranges.iter().map(|(a, b)| b - a + 1).product()
}

fn enumerate(instrs: &HashMap<String, Vec<Instr>>, i: String, idx: usize, ranges: Ranges) -> usize {
    match instrs.get(&i).unwrap().get(idx).unwrap() {
        Instr::Just(a) => match a {
            Action::Reject => 0,
            Action::Accept => sum_items(ranges),
            Action::To(new_i) => enumerate(instrs, new_i.to_string(), 0, ranges),
        },
        // we can always assume a cmp is not a terminator
        Instr::Cmp(c_idx, ord, val, a) => {
            let (left, right) = split_range(ranges, *c_idx, *val, *ord);
            let left_result = match a {
                Action::Reject => 0,
                Action::Accept => sum_items(left),
                Action::To(new_i) => enumerate(instrs, new_i.to_string(), 0, left),
            };
            let right_result = enumerate(instrs, i, idx + 1, right);

            left_result + right_result
        }
    }
}

impl SolutionLinear<(HashMap<String, Vec<Instr>>, Vec<Obj>), usize, usize> for Day19Solution {
    fn load(input: &str) -> Result<(HashMap<String, Vec<Instr>>, Vec<Obj>)> {
        let (instrs, objs) = input.split_once("\n\n").unwrap();

        let parsed_instrs = instrs
            .lines()
            .map(|l| {
                let (tag, steps) = l.split_once('{').unwrap();
                let parsed_steps = steps[0..steps.len() - 1]
                    .split(',')
                    .map(|st| match st.split_once(':') {
                        Some((ab, c)) => {
                            let mut chars = ab.chars();
                            let x = chars.next().unwrap();
                            let cmp = match chars.next().unwrap() {
                                '>' => Ordering::Greater,
                                '<' => Ordering::Less,
                                _ => panic!("unexpected character"),
                            };
                            let val =
                                chars.fold(0, |acc, v| acc * 10 + v.to_digit(10).unwrap()) as usize;
                            let i = match c {
                                "R" => Action::Reject,
                                "A" => Action::Accept,
                                _ => Action::To(c.to_string()),
                            };
                            Instr::Cmp(IDX.iter().position(|i| i == &x).unwrap(), cmp, val, i)
                        }
                        None => match st {
                            "R" => Instr::Just(Action::Reject),
                            "A" => Instr::Just(Action::Accept),
                            _ => Instr::Just(Action::To(st.to_string())),
                        },
                    })
                    .collect_vec();
                (tag.to_string(), parsed_steps)
            })
            .collect();

        let parsed_objs = objs
            .lines()
            .map(|l| {
                l[1..l.len() - 1]
                    .split(',')
                    .map(|v| v[2..].parse().unwrap())
                    .collect_vec()
                    .try_into()
                    .unwrap()
            })
            .collect_vec();

        Ok((parsed_instrs, parsed_objs))
    }

    fn part1((instrs, objs): &mut (HashMap<String, Vec<Instr>>, Vec<Obj>)) -> Result<usize> {
        Ok(objs
            .iter()
            .filter_map(|o| analyze(instrs, *o))
            .map(|o| o.iter().sum::<usize>())
            .sum())
    }

    fn part2(
        (instrs, _): &mut (HashMap<String, Vec<Instr>>, Vec<Obj>),
        _part_1_solution: usize,
    ) -> Result<usize> {
        Ok(enumerate(instrs, "in".to_string(), 0, [(1, 4000); 4]))
    }
}

#[cfg(test)]
mod tests {
    use super::Day19Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
",
        19114,
        167_409_079_868_000
    )]
    fn validate_day19(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let mut input = Day19Solution::load(input).unwrap();

        let p1 = Day19Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);

        let p2 = Day19Solution::part2(&mut input, p1).unwrap();
        assert_eq!(expected_2, p2);
    }
}

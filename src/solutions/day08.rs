use std::{collections::HashMap, iter::successors};

use crate::utils::{
    maths::lcm,
    solver_types::{solve_linear, SolutionLinear},
};
use anyhow::Result;
use itertools::Itertools;

pub struct Day08Solution {}

pub fn day08(input: &str) -> Result<f32> {
    solve_linear::<Day08Solution, _, _, _>(input)
}

// we translate strings into chars cus string comparisons are cringe
type N = [char; 3];
type Nodes = HashMap<N, (N, N)>;

fn to_n(s: &str) -> N {
    s.chars().take(3).collect_vec().try_into().unwrap()
}

fn traverse_while(
    directions: Vec<char>,
    start: N,
    while_term: fn(&N) -> bool,
    map: &Nodes,
) -> usize {
    let mut dirs = directions.iter().cycle();

    successors(Some(start), |n| {
        let (l, r) = map.get(n).unwrap();
        match dirs.next().unwrap() {
            'L' => Some(*l),
            'R' => Some(*r),
            _ => panic!("unexpected direction"),
        }
    })
    .take_while(while_term)
    .count()
}

impl SolutionLinear<(Vec<char>, Nodes), usize, u128> for Day08Solution {
    fn load(input: &str) -> Result<(Vec<char>, Nodes)> {
        let (dirs, map) = input.split_once("\n\n").unwrap();

        let parsed_map = map.lines().fold(HashMap::new(), |mut hm, l| {
            // x -> (y,z)
            let (x, yz) = l.split_once(" = (").unwrap(); // doing silly splits here cus cant be bothered to regex - the game awards bombed my sleep schedule
            let (y, z) = yz.split_once(", ").unwrap();

            let _ = hm.insert(to_n(x), (to_n(y), to_n(z)));
            hm
        });

        Ok((dirs.chars().collect_vec(), parsed_map))
    }

    fn part1((dirs, nodes): &mut (Vec<char>, Nodes)) -> Result<usize> {
        Ok(traverse_while(
            dirs.clone(),
            ['A', 'A', 'A'],
            |n| n != &['Z', 'Z', 'Z'],
            nodes,
        ))
    }

    fn part2((dirs, nodes): &mut (Vec<char>, Nodes), _part_1_solution: usize) -> Result<u128> {
        // parse out a's
        Ok(
            nodes
                .keys()
                .filter(|[_, _, c]| c == &'A')
                .map(|start| {
                    traverse_while(dirs.clone(), *start, |[_, _, c]| c != &'Z', nodes) as u128
                })
                .reduce(lcm)
                .unwrap(), // its the lowest common multiple babey!!!!
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Day08Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
",
        2,
        2
    )]
    #[case(
        "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
",
        6,
        6
    )]
    #[case(
        "LR

AAA = (11B, XXX)
11B = (XXX, ZZZ)
ZZZ = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
",
        2,
        6
    )]
    fn validate(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: u128) {
        let mut input = Day08Solution::load(input).unwrap();

        let p1 = Day08Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);

        let p2 = Day08Solution::part2(&mut input, p1).unwrap();
        assert_eq!(expected_2, p2);
    }
}

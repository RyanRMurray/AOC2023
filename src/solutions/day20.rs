use std::collections::{HashMap, HashSet, VecDeque};

use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::Result;
use itertools::Itertools;

pub struct Day20Solution {}

pub fn day20(input: &str) -> Result<f32> {
    solve_linear::<Day20Solution, _, _, _>(input)
}

#[derive(Default, Debug, Clone)]
struct Memory {
    pub count: usize,
    pub inputs: HashMap<String, bool>,
}

#[derive(Debug, Clone)]
enum Station {
    Broadcast(Vec<String>),
    FlipFlop(bool, Vec<String>),
    Conjunction(Memory, Vec<String>),
}

impl Station {
    fn children(&self) -> &Vec<String> {
        match self {
            Self::Broadcast(cs) | Self::Conjunction(_, cs) | Self::FlipFlop(_, cs) => cs,
        }
    }
}

type Stations = HashMap<String, Station>;

fn first_traverse(stations: &mut Stations) {
    let mut visited = HashSet::new();
    let mut to_visit = vec!["roadcaster".to_string()];

    while let Some(next) = to_visit.pop() {
        visited.insert(next.clone());
        if let Some(st) = stations.get(&next) {
            to_visit.extend(
                st.children()
                    .iter()
                    .filter(|s| !visited.contains(*s))
                    .map(|s| s.to_string()),
            );
            for c in st.children().clone() {
                if let Some(Station::Conjunction(mem, _)) = stations.get_mut(&c) {
                    mem.inputs.insert(next.clone(), false);
                }
            }
        }
    }
}

fn pwadd([a, b]: [usize; 2], [c, d]: [usize; 2]) -> [usize; 2] {
    [a + c, b + d]
}

/// we record whether a station has produced a high signal this run
fn pulse(stations: &mut Stations, been_live: &mut HashSet<String>) -> [usize; 2] {
    let mut sum = [0, 0];
    let mut to_visit = VecDeque::from(["roadcaster".to_string()]);
    let mut to_do: HashMap<String, VecDeque<(String, bool)>> = HashMap::from([(
        "roadcaster".to_string(),
        VecDeque::from([("button".to_string(), false)]),
    )]);

    while let Some(at) = to_visit.pop_front() {
        for (from, p) in to_do.get(&at).unwrap().clone() {
            if p {
                been_live.insert(from.clone());
            }
            if p {
                sum[1] += 1;
            } else {
                sum[0] += 1;
            };
            match stations.get_mut(&at) {
                None => (),
                Some(Station::Broadcast(cs)) => {
                    for c in cs {
                        if !to_visit.contains(c) {
                            to_visit.push_back(c.to_string());
                        }
                        to_do
                            .entry(c.to_string())
                            .or_default()
                            .push_back((at.to_string(), p));
                    }
                }
                Some(Station::FlipFlop(on, cs)) => {
                    if !p {
                        *on = !(*on);
                        for c in cs {
                            if !to_visit.contains(c) {
                                to_visit.push_back(c.to_string());
                            }
                            to_do
                                .entry(c.to_string())
                                .or_default()
                                .push_back((at.to_string(), *on));
                        }
                    }
                }
                Some(Station::Conjunction(mem, cs)) => {
                    match (p, *mem.inputs.get(&from).unwrap()) {
                        (true, true) | (false, false) => (),
                        (true, false) => {
                            mem.inputs.insert(from.to_string(), true);
                            mem.count += 1;
                        }
                        (false, true) => {
                            mem.inputs.insert(from.to_string(), false);
                            mem.count -= 1;
                        }
                    }
                    let sig = mem.count != mem.inputs.len();
                    for c in cs {
                        if !to_visit.contains(c) {
                            to_visit.push_back(c.to_string());
                        }
                        to_do
                            .entry(c.to_string())
                            .or_default()
                            .push_back((at.to_string(), sig));
                    }
                }
            }
        }
        to_do.remove(&at);
    }

    sum
}

impl SolutionLinear<Stations, usize, usize> for Day20Solution {
    fn load(input: &str) -> Result<Stations> {
        // parse stations
        let mut stations = input
            .lines()
            .map(|l| {
                let (st, children) = l.split_once(" -> ").unwrap();
                let parsed_children = children.split(", ").map(|s| s.to_string()).collect_vec();
                let (ty, tag) = st.split_at(1);
                let parsed_ty = match ty {
                    "b" => Station::Broadcast(parsed_children),
                    "%" => Station::FlipFlop(false, parsed_children),
                    "&" => Station::Conjunction(Memory::default(), parsed_children),
                    _ => panic!("Unexpected symbol {}", ty),
                };
                (tag.to_string(), parsed_ty)
            })
            .collect();

        // traverse once to establish memory
        first_traverse(&mut stations);
        Ok(stations)
    }

    fn part1(input: &mut Stations) -> Result<usize> {
        let mut p1 = input.to_owned();
        let mut been_live = HashSet::new();
        let mut sum = [0, 0];
        for _ in 0..1000 {
            sum = pwadd(sum, pulse(&mut p1, &mut been_live));
        }
        Ok(sum[0] * sum[1])
    }

    /// the target rx is the sole reciever of a conj station. this conj station receives N inputs, each of which fires on a prime-length period
    /// so to find when rx recieves a low signal, we need to multiply the length of these periods to find their LCM.
    fn part2(input: &mut Stations, _part_1_solution: usize) -> Result<usize> {
        let mut been_live = HashSet::new();
        let (p, _) = input
            .iter()
            .find(|(_, s)| s.children().contains(&"rx".to_string()))
            .unwrap();
        let ins = match input.get(p).unwrap() {
            Station::Conjunction(mem, _) => mem.inputs.keys().cloned().collect_vec(),
            _ => panic!("unexpected parent"),
        };
        let mut hits = vec![0; ins.len()];

        let mut x = 0;

        while hits.contains(&0) {
            x += 1;
            pulse(input, &mut been_live);
            for (i, feed) in ins.iter().enumerate() {
                if been_live.contains(feed) && hits[i] == 0 {
                    hits[i] = x;
                }
            }
        }
        Ok(hits.iter().product())
    }
}

#[cfg(test)]
mod tests {
    use super::Day20Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[ignore = "no answer for part 2"]
    #[case(
        "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
",
        32000000,
        2
    )]
    #[ignore = "no answer for part 2"]
    #[case(
        "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
",
        11687500,
        2
    )]
    fn validate_day20(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let mut input = Day20Solution::load(input).unwrap();

        let p1 = Day20Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);

        let p2 = Day20Solution::part2(&mut input, p1).unwrap();
        assert_eq!(expected_2, p2);
    }
}

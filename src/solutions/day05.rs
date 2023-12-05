use std::collections::HashSet;

use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::Result;
use itertools::Itertools;

pub struct Day05Solution {}

pub fn day05(input: &str) -> Result<f32> {
    solve_linear::<Day05Solution, _, _, _>(input)
}

struct Guide {
    seeds: Vec<usize>,
    maps: Vec<Vec<[usize; 3]>>, //[destination range start, source range start, range magnitude]
}

fn map_to_destination(num: usize, ranges: &Vec<[usize; 3]>) -> usize {
    for r in ranges {
        if num >= r[1] && num < r[1] + r[2] {
            let depth = num - r[1];
            return r[0] + depth;
        }
    }
    num
}

fn map_range(input: [usize; 2], ranges: &Vec<[usize; 3]>) -> Vec<[usize; 2]> {
    let mut new_ranges = vec![];
    let mut start = input[0];
    let end = input[1];

    for r in ranges {
        if start < r[1] {
            new_ranges.push([start, r[1] - 1]);
            start = r[1];
        }

        if start >= r[1] && start < r[1] + r[2] - 1 {
            if end < r[1] + r[2] {
                let depth1 = start - r[1];
                let depth2 = end - r[1];
                new_ranges.push([r[0] + depth1, r[0] + depth2]);
                start = end;
                break;
            } else {
                let depth = start - r[1];
                new_ranges.push([r[0] + depth, r[0] + r[2]]);
                start = r[1] + r[2];
            }
        }
    }
    if start != end {
        new_ranges.push([start, end]);
    }
    new_ranges
}

impl SolutionLinear<Guide, usize, usize> for Day05Solution {
    fn load(input: &str) -> Result<Guide> {
        let (seeds, maps) = input.split_once("\n\n").unwrap();
        let seed_nums = seeds
            .split(' ')
            .skip(1)
            .map(|n| n.parse().unwrap())
            .collect();

        let maps_parsed = maps
            .split("\n\n")
            .map(|gr| {
                gr.lines()
                    .skip(1)
                    .map(|l| {
                        l.split(' ')
                            .map(|n| n.parse().unwrap())
                            .collect_vec()
                            .try_into()
                            .unwrap()
                    })
                    .sorted_by(|a: &[usize; 3], b: &[usize; 3]| a[1].cmp(&b[1]))
                    .collect_vec()
            })
            .collect_vec();

        Ok(Guide {
            seeds: seed_nums,
            maps: maps_parsed,
        })
    }

    fn part1(input: &mut Guide) -> Result<usize> {
        let mut min = usize::MAX;

        for seed in &input.seeds {
            let mut result = *seed;
            for m in &input.maps {
                result = map_to_destination(result, m);
            }
            min = min.min(result);
        }

        Ok(min)
    }

    fn part2(input: &mut Guide, _part_1_solution: usize) -> Result<usize> {
        let ranges: HashSet<[usize; 2]> = input
            .seeds
            .chunks_exact(2)
            .map(|c| [c[0], c[0] + c[1] - 1])
            .collect();

        Ok(input
            .maps
            .iter()
            // for each map, create the ranges mapped from the prior set of ranges
            .fold(ranges, |rs, m| {
                rs.into_iter().flat_map(|r| map_range(r, m)).collect()
            })
            // now find the minimum amongst those ranges
            .into_iter()
            .map(|[min, _]| min)
            .min()
            .unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::Day05Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
",
        35,
        46
    )]
    fn validate(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let mut input = Day05Solution::load(input).unwrap();
        let p1 = Day05Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);

        let p2 = Day05Solution::part2(&mut input, p1).unwrap();
        assert_eq!(expected_2, p2);
    }
}

use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet, VecDeque},
};

use crate::utils::{
    grid::Grid,
    load_input::load_2d_grid,
    point::Pt,
    solver_types::{solve_linear, SolutionLinear},
};
use anyhow::Result;
use itertools::Itertools;
use priority_queue::PriorityQueue;

pub struct Day23Solution {}

pub fn day23(input: &str) -> Result<f32> {
    solve_linear::<Day23Solution, _, _, _>(input)
}

#[derive(Default, Clone, Copy, PartialEq)]
enum Space {
    #[default]
    Forest,
    Path,
    Slope(Pt<2>),
}

const DIRS: [Pt<2>; 4] = [Pt([0, 1]), Pt([-1, 0]), Pt([1, 0]), Pt([0, -1])];

fn get_neighbours(
    grid: &Grid<Space, 2>,
    visited: &HashSet<Pt<2>>,
    dry_slopes: bool,
    at: &Pt<2>,
) -> Vec<Pt<2>> {
    match (dry_slopes, grid.get_def(at)) {
        (_, Space::Forest) => vec![],
        (false, Space::Slope(p)) => {
            if visited.contains(&(at + &p)) {
                vec![]
            } else {
                vec![at + &p]
            }
        }
        _ => DIRS
            .iter()
            .filter_map(|d| {
                if visited.contains(&(at + d)) || grid.get_def(&(at + d)) == Space::Forest {
                    None
                } else {
                    Some(at + d)
                }
            })
            .collect_vec(),
    }
}

/// find all spaces with 3 or more valid neighbours
fn find_junctions(input: &Grid<Space, 2>, dry_slopes: bool) -> HashSet<Pt<2>> {
    let mut juncts = HashSet::new();
    for p in input.iter_linear() {
        if get_neighbours(input, &HashSet::new(), dry_slopes, p).len() > 2 {
            juncts.insert(*p);
        }
    }
    juncts
}

/// find distance to neighbour junctions - discard if already seen
fn find_junction_distances(
    input: &Grid<Space, 2>,
    dry_slopes: bool,
    juncts: &HashSet<Pt<2>>,
    start: Pt<2>,
) -> Vec<(usize, Pt<2>)> {
    let mut distances = vec![];
    let mut visited = HashSet::new();
    let mut to_visit = VecDeque::from([(0, start)]);

    while let Some((cost, visiting)) = to_visit.pop_front() {
        visited.insert(visiting);
        if visiting != start && juncts.contains(&visiting) {
            distances.push((cost, visiting));
            continue;
        }

        // get next steps
        to_visit.extend(
            get_neighbours(input, &visited, dry_slopes, &visiting)
                .into_iter()
                .map(|n| (cost + 1, n)),
        );
    }
    distances
}

fn find_longest_path(input: &Grid<Space, 2>, dry_slopes: bool) -> usize {
    let (_, [_, max_y]) = input.bounds();

    // find all junctions/goals
    let mut juncts = find_junctions(input, dry_slopes);
    let start = *input
        .grid
        .iter()
        .find(|(Pt([_, y]), s)| y == &0 && s == &&Space::Path)
        .unwrap()
        .0;
    let end = *input
        .grid
        .iter()
        .find(|(Pt([_, y]), s)| y == &max_y && s == &&Space::Path)
        .unwrap()
        .0;
    juncts.insert(start);
    juncts.insert(end);

    // find all edges between junctions
    let mut edges = HashMap::new();
    for j in juncts.iter() {
        let distances = find_junction_distances(input, dry_slopes, &juncts, *j);
        edges.insert(*j, distances);
    }

    // use djikstra's to find the longest distance between the start and end
    let mut to_visit = PriorityQueue::new();
    let mut distances: HashMap<Pt<2>, usize> = HashMap::new();
    to_visit.push((start, vec![]), Reverse(0));

    while let Some(((visiting, mut visited), Reverse(cost))) = to_visit.pop() {
        visited.push(visiting);
        let recorded = distances.get(&visiting).unwrap_or(&0);
        if recorded < &cost {
            distances.insert(visiting, cost);
        }

        let ns = edges.get(&visiting).unwrap().iter().filter_map(|(v, pt)| {
            if visited.contains(pt) {
                None
            } else {
                Some((*pt, v + cost))
            }
        });

        for (n, new_cost) in ns {
            to_visit.push((n, visited.clone()), Reverse(new_cost));
        }
    }

    *distances.get(&end).unwrap()
}

impl SolutionLinear<Grid<Space, 2>, usize, usize> for Day23Solution {
    fn load(input: &str) -> Result<Grid<Space, 2>> {
        Ok(load_2d_grid(input, |c| match c {
            '>' => Space::Slope(Pt([1, 0])),
            '<' => Space::Slope(Pt([-1, 0])),
            '^' => Space::Slope(Pt([0, -1])),
            'v' => Space::Slope(Pt([0, 1])),
            '.' => Space::Path,
            _ => Space::Forest,
        }))
    }

    fn part1(input: &mut Grid<Space, 2>) -> Result<usize> {
        Ok(find_longest_path(input, false))
    }

    fn part2(input: &mut Grid<Space, 2>, _part_1_solution: usize) -> Result<usize> {
        Ok(find_longest_path(input, true))
    }
}

#[cfg(test)]
mod tests {
    use super::Day23Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
",
        94,
        154
    )]
    fn validate_day23(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let mut input = Day23Solution::load(input).unwrap();

        let p1 = Day23Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);

        let p2 = Day23Solution::part2(&mut input, p1).unwrap();
        assert_eq!(expected_2, p2);
    }
}

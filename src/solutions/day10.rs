use std::{
    collections::{HashMap, HashSet},
    convert::identity,
};

use crate::utils::{
    grid::Grid,
    load_input::load_2d_grid,
    point::Pt,
    solver_types::{solve_linear, SolutionLinear},
};
use anyhow::Result;
use itertools::Itertools;
use lazy_static::lazy_static;

// TODO
pub struct Day10Solution {}

pub fn day10(input: &str) -> Result<f32> {
    solve_linear::<Day10Solution, _, _, _>(input)
}

lazy_static! {
    static ref PIPES: HashMap<char, Vec<Pt<2>>> = HashMap::from([
        ('|', vec![Pt([0, -1]), Pt([0, 1])]),
        ('-', vec![Pt([1, 0]), Pt([-1, 0])]),
        ('L', vec![Pt([0, -1]), Pt([1, 0])]),
        ('J', vec![Pt([0, -1]), Pt([-1, 0])]),
        ('7', vec![Pt([0, 1]), Pt([-1, 0])]),
        ('F', vec![Pt([0, 1]), Pt([1, 0])]),
        ('.', vec![]),
        ('S', vec![]),
    ]);
}

type Maze = Grid<char, 2>;

fn find_start(maze: &Maze) -> Pt<2> {
    *maze
        .grid
        .iter()
        .find_map(|(k, &v)| if v == 'S' { Some(k) } else { None })
        .unwrap()
}

/// check a point (to) has an exit to a point (from)
fn can_enter(maze: &Maze, from: &Pt<2>, to: &Pt<2>) -> bool {
    let to_shape = match maze.grid.get(to) {
        None => return false,
        Some(s) => s,
    };
    PIPES
        .get(to_shape)
        .unwrap()
        .iter()
        .map(|offset| offset + to)
        .contains(from)
}

fn adjs(maze: &Maze, pt: &Pt<2>) -> Vec<Pt<2>> {
    PIPES
        .get(maze.grid.get(pt).unwrap())
        .unwrap()
        .iter()
        .map(|offset| offset + pt)
        .collect_vec()
}

fn find_loop(maze: &Maze, start: &Pt<2>) -> Vec<(Pt<2>, usize)> {
    // first, find starting directions (Vec<(from, here)>)
    let starts = Pt::<2>::card_offsets()
        .iter()
        .map(|offset| (*start, offset + start))
        .filter(|(_, to)| can_enter(maze, start, to))
        .collect_vec();

    // notate distances of points in loop from start
    let mut distances: HashMap<Pt<2>, usize> = HashMap::from([(*start, 0)]);

    for s in starts {
        let mut ptr = s;
        let mut d = 1;
        while ptr.1 != *start {
            // note distance if shorter
            match distances.get(&ptr.1) {
                Some(dist) => distances.insert(ptr.1, *dist.min(&d)),
                None => distances.insert(ptr.1, d),
            };
            // get next step
            ptr = (
                ptr.1,
                *adjs(maze, &ptr.1).iter().find(|p| **p != ptr.0).unwrap(),
            );
            d += 1;
        }
    }

    distances.iter().map(|(k, v)| (*k, *v)).collect_vec()
}

fn valid_ns(
    to_explore: &[Pt<2>],
    boundary: &HashSet<Pt<2>>,
    visited: &HashSet<Pt<2>>,
    visit: Pt<2>,
) -> Vec<Pt<2>> {
    Pt::<2>::card_offsets()
        .into_iter()
        .filter_map(|offset| {
            let v = &(offset + visit);
            if to_explore.contains(v) && !boundary.contains(v) && !visited.contains(v) {
                Some(*v)
            } else {
                None
            }
        })
        .collect_vec()
}

fn outside_loop(ps: &HashSet<Pt<2>>, max_x: isize, max_y: isize) -> bool {
    ps.iter()
        .any(|Pt([x, y])| *x == 0 || *y == 0 || *x == max_x || *y == max_y)
}

// make a higher-resolution of the boundary
fn resolve_boundary(maze: &Maze, boundary: HashSet<Pt<2>>) -> HashSet<Pt<2>> {
    boundary.iter().fold(HashSet::new(), |mut hs, p| {
        let new_pos = *p * 2;
        let c = maze.grid.get(p).unwrap();
        hs.extend(PIPES.get(c).unwrap().iter().map(|offset| offset + &new_pos));
        hs.insert(new_pos);
        hs
    })
}

impl SolutionLinear<Maze, usize, usize> for Day10Solution {
    fn load(input: &str) -> Result<Maze> {
        Ok(load_2d_grid(input, identity))
    }

    fn part1(input: &mut Maze) -> Result<usize> {
        Ok(*find_loop(input, &find_start(input))
            .iter()
            .map(|(_, v)| v)
            .max()
            .unwrap())
    }

    fn part2(input: &mut Maze, _part_1_solution: usize) -> Result<usize> {
        // oh god im too tired for flood fill. OH WELL.
        let mut contained = 0;
        let b: HashSet<Pt<2>> = find_loop(input, &find_start(input))
            .iter()
            .map(|(k, _)| *k)
            .collect();
        let boundary = resolve_boundary(input, b);

        let ([_, _], [max_x, max_y]) = input.bounds();

        let mut to_explore = (0..max_x * 2 + 1)
            .flat_map(|x| (0..max_y * 2 + 1).map(move |y| Pt([x, y])))
            .filter(|p| !boundary.contains(p))
            .collect_vec();

        while !to_explore.is_empty() {
            // flood fill
            let mut to_visit = vec![to_explore.pop().unwrap()];
            let mut visited = HashSet::new();

            while let Some(v) = to_visit.pop() {
                visited.insert(v);
                to_visit.extend(valid_ns(&to_explore, &boundary, &visited, v));
            }

            // drop visited from to_expore
            to_explore = to_explore
                .into_iter()
                .filter(|p| !visited.contains(p))
                .collect_vec();

            // count if within loop
            if !outside_loop(&visited, max_x * 2, max_y * 2) {
                contained += visited
                    .iter()
                    .filter(|Pt([x, y])| (x % 2 == 0) && (y % 2 == 0))
                    .count();
            }
        }

        Ok(contained)
    }
}

#[cfg(test)]
mod tests {
    use super::Day10Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "-L|F7
7S-7|
L|7||
-L-J|
L|-JF
",
        4,
        1
    )]
    #[case(
        "..F7.
.FJ|.
SJ.L7
|F--J
LJ...
",
        8,
        1
    )]
    #[case(
        "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
",
        23,
        4
    )]
    #[case(
        ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
",
        70,
        8
    )]
    #[case(
        "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
",
        80,
        10
    )]
    fn validate_day10(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let mut input = Day10Solution::load(input).unwrap();

        let p1 = Day10Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);

        let p2 = Day10Solution::part2(&mut input, p1).unwrap();
        assert_eq!(expected_2, p2);
    }
}

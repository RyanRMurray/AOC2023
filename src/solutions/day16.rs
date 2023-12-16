use std::collections::{HashSet, VecDeque};

use crate::utils::{
    grid::Grid,
    load_input::load_2d_grid,
    point::{Pt, D},
    solver_types::{solve_linear, SolutionLinear},
};
use anyhow::Result;
use itertools::Itertools;

pub struct Day16Solution {}

//Spaces have Inputs and Outputs.
//if the approaching beam isnt coming from an Input, it goes right through
//otherwise, it's mapped to the corresponding Output
#[derive(Clone, Copy, Default)]
enum Space {
    #[default]
    Empty,
    ForwardSlash,
    BackSlash,
    Minus,
    Pipe,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Beam {
    pos: Pt<2>,
    dir: D,
}

impl Space {
    fn enter(&self, beam: &Beam) -> Vec<Beam> {
        let pos = beam.pos + beam.dir.val();
        match self {
            Space::Empty => vec![Beam { pos, dir: beam.dir }],
            Space::ForwardSlash => match beam.dir {
                D::Up => vec![Beam { pos, dir: D::Right }],
                D::Down => vec![Beam { pos, dir: D::Left }],
                D::Left => vec![Beam { pos, dir: D::Down }],
                D::Right => vec![Beam { pos, dir: D::Up }],
            },
            Space::BackSlash => match beam.dir {
                D::Up => vec![Beam { pos, dir: D::Left }],
                D::Down => vec![Beam { pos, dir: D::Right }],
                D::Left => vec![Beam { pos, dir: D::Up }],
                D::Right => vec![Beam { pos, dir: D::Down }],
            },
            Space::Minus => match beam.dir {
                D::Up | D::Down => vec![Beam { pos, dir: D::Left }, Beam { pos, dir: D::Right }],
                D::Left | D::Right => vec![Beam { pos, dir: beam.dir }],
            },
            Space::Pipe => match beam.dir {
                D::Up | D::Down => vec![Beam { pos, dir: beam.dir }],
                D::Left | D::Right => vec![Beam { pos, dir: D::Up }, Beam { pos, dir: D::Down }],
            },
        }
    }
}

pub fn day16(input: &str) -> Result<f32> {
    solve_linear::<Day16Solution, _, _, _>(input)
}

fn energize(input: &Grid<Space, 2>, start: Beam) -> usize {
    let mut active_beams = VecDeque::from([start]);
    let mut seen = HashSet::new();

    while let Some(b) = active_beams.pop_front() {
        seen.insert(b);
        if let Some(s) = input.grid.get(&(b.pos + b.dir.val())) {
            active_beams.extend(
                s.enter(&b)
                    .into_iter()
                    .filter(|new_b| !seen.contains(new_b)),
            );
        }
    }

    let visited: HashSet<_> = seen.iter().map(|b| b.pos).collect();
    visited.len() - 1
}

impl SolutionLinear<Grid<Space, 2>, usize, usize> for Day16Solution {
    fn load(input: &str) -> Result<Grid<Space, 2>> {
        Ok(load_2d_grid(input, |c| match c {
            '.' => Space::Empty,
            '/' => Space::ForwardSlash,
            '\\' => Space::BackSlash,
            '-' => Space::Minus,
            '|' => Space::Pipe,
            _ => panic!("unexpected character"),
        }))
    }

    fn part1(input: &mut Grid<Space, 2>) -> Result<usize> {
        Ok(energize(
            input,
            Beam {
                pos: Pt([-1, 0]),
                dir: D::Right,
            },
        ))
    }

    fn part2(input: &mut Grid<Space, 2>, _part_1_solution: usize) -> Result<usize> {
        let (_, [max_x, max_y]) = input.bounds();

        let binding = vec![
            (0..max_x + 1)
                .map(|x| Beam {
                    pos: Pt([x, -1]),
                    dir: D::Down,
                })
                .collect_vec(),
            (0..max_x + 1)
                .map(|x| Beam {
                    pos: Pt([x, max_y + 1]),
                    dir: D::Up,
                })
                .collect_vec(),
            (0..max_y + 1)
                .map(|y| Beam {
                    pos: Pt([-1, y]),
                    dir: D::Right,
                })
                .collect_vec(),
            (0..max_y + 1)
                .map(|y| Beam {
                    pos: Pt([max_x + 1, y]),
                    dir: D::Left,
                })
                .collect_vec(),
        ];
        let starts = binding.iter().flat_map(|it| it.clone());

        let mut max = 0;
        for start in starts {
            max = max.max(energize(input, start));
        }
        Ok(max)
    }
}

#[cfg(test)]
mod tests {
    use super::Day16Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
",
        46,
        51
    )]
    fn validate_day16(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let mut input = Day16Solution::load(input).unwrap();

        let p1 = Day16Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);

        let p2 = Day16Solution::part2(&mut input, p1).unwrap();
        assert_eq!(expected_2, p2);
    }
}

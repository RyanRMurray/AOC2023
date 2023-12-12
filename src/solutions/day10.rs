use std::{collections::HashMap, convert::identity};

use crate::utils::{
    grid::Grid,
    load_input::load_2d_grid,
    point::Pt,
    solver_types::{solve_linear, SolutionLinear},
};
use anyhow::Result;
use itertools::Itertools;
use lazy_static::lazy_static;

// implementation based on non-zero winding as described here: https://old.reddit.com/r/adventofcode/comments/18eza5g/2023_day_10_animated_visualization/kcqwjon/
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

fn measure_loop(maze: &Maze, start: &Pt<2>) -> (HashMap<Pt<2>, isize>, isize) {
    // notate distances of points in loop from start
    let mut distances: HashMap<Pt<2>, isize> = HashMap::from([(*start, 1)]);

    // find start
    let mut ptr = [Pt([0, -1]), Pt([1, 0]), Pt([0, 1]), Pt([-1, 0])]
        .iter()
        .map(|offset| (*start, offset + start))
        .find(|(_, to)| can_enter(maze, start, to))
        .unwrap();
    let mut d = 2;

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
    (distances, d - 1)
}

impl SolutionLinear<Maze, isize, usize> for Day10Solution {
    fn load(input: &str) -> Result<Maze> {
        Ok(load_2d_grid(input, identity))
    }

    fn part1(input: &mut Maze) -> Result<isize> {
        Ok(measure_loop(input, &find_start(input)).1 / 2)
    }

    fn part2(input: &mut Maze, _part_1_solution: isize) -> Result<usize> {
        let ([_, _], [max_x, _]) = input.bounds();
        let (loop_steps, size) = measure_loop(input, &find_start(input));
        let mut contained = 0;
        let mut inside = 0;

        for pt in input.iter_linear() {
            let below = pt + &Pt([0, 1]);
            let steps = loop_steps
                .get(pt)
                .and_then(|st1| loop_steps.get(&below).map(|st2| st1 - st2 % size));

            if pt.0[0] == max_x {
                inside = 0;
            } else {
                match steps {
                    Some(v) => {
                        if v.abs() == 1 {
                            inside += v
                        }
                    }
                    _ => {
                        if inside != 0 && !loop_steps.contains_key(pt) {
                            contained += 1;
                        }
                    }
                }
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
    fn validate_day10(#[case] input: &str, #[case] expected_1: isize, #[case] expected_2: usize) {
        let mut input = Day10Solution::load(input).unwrap();

        let p1 = Day10Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);

        let p2 = Day10Solution::part2(&mut input, p1).unwrap();
        assert_eq!(expected_2, p2);
    }
}

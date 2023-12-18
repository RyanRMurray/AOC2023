use crate::utils::{
    grid::Grid,
    load_input::load_lines,
    point::{Pt, D},
    solver_types::{solve_linear, SolutionLinear},
};
use anyhow::Result;
use itertools::Itertools;

pub struct Day18Solution {}

pub fn day18(input: &str) -> Result<f32> {
    solve_linear::<Day18Solution, _, _, _>(input)
}

type Command = (D, usize, (D, usize));

// stolen from https://stackoverflow.com/a/52992629/22029215
// expects "(#FFFFFF)"
fn to_hex(s: &str) -> (D, usize) {
    let (a, b) = s.split_at(7);
    let (_, dist) = a.split_at(2);
    (
        match b.chars().next().unwrap() {
            '0' => D::Right,
            '1' => D::Down,
            '2' => D::Left,
            '3' => D::Up,
            _ => panic!("unexpected hex val"),
        },
        usize::from_str_radix(dist, 16).unwrap(),
    )
}

fn carve(commands: &Vec<Command>) -> Grid<isize, 2> {
    let mut ptr = Pt([0, 0]);
    let mut grid = Grid::from(vec![(ptr, 1)]);
    let mut distance = 2isize;

    for &(dir, steps, _) in commands {
        for _ in 1..steps + 1 {
            ptr += dir.val();

            if !grid.grid.contains_key(&ptr) {
                grid.grid.insert(ptr, distance);
            }

            distance += 1;
        }
    }
    grid
}

// unwind method again
fn fill(grid: &mut Grid<isize, 2>) {
    let ([min_x, min_y], [max_x, max_y]) = grid.bounds();
    let mut inside = 0;
    let size = *grid.grid.iter().last().unwrap().1;

    for y in min_y..max_y + 1 {
        for x in min_x..max_x + 1 {
            let pt = Pt([x, y]);
            let below = pt + D::Down.val();
            let diff = grid
                .grid
                .get(&pt)
                .and_then(|d1| grid.grid.get(&below).map(|d2| d1 - d2 % size));

            if x == max_x {
                inside = 0;
            } else {
                match diff {
                    Some(v) => {
                        if v.abs() == 1 {
                            inside += v
                        }
                    }
                    _ => {
                        if inside != 0 {
                            grid.grid.insert(pt, -1);
                        }
                    }
                }
            }
        }
    }
}

fn fix_commands(commands: Vec<Command>) -> Vec<Command> {
    commands
        .into_iter()
        .map(|(_, _, (dir, dist))| (dir, dist, (D::Up, 0)))
        .collect_vec()
}

impl SolutionLinear<Vec<Command>, usize, usize> for Day18Solution {
    fn load(input: &str) -> Result<Vec<Command>> {
        Ok(load_lines(input, |l| {
            let mut parts = l.split_whitespace();

            (
                match parts.next().unwrap() {
                    "R" => D::Right,
                    "L" => D::Left,
                    "U" => D::Up,
                    "D" => D::Down,
                    _ => panic!("bad letter, not nice :("),
                },
                parts.next().unwrap().parse().unwrap(),
                to_hex(parts.next().unwrap()),
            )
        }))
    }

    fn part1(input: &mut Vec<Command>) -> Result<usize> {
        let mut g = carve(input);
        fill(&mut g);
        Ok(g.grid.len())
    }

    fn part2(input: &mut Vec<Command>, _part_1_solution: usize) -> Result<usize> {
        let input = fix_commands(input.to_vec());
        let mut g = carve(&input);
        fill(&mut g);
        Ok(g.grid.len())
    }
}

#[cfg(test)]
mod tests {
    use super::Day18Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[ignore = "delete to test solution"]
    #[case(
        "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
",
        62,
        952408144115
    )]
    fn validate_day18(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let mut input = Day18Solution::load(input).unwrap();

        let p1 = Day18Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);

        let p2 = Day18Solution::part2(&mut input, p1).unwrap();
        assert_eq!(expected_2, p2);
    }
}

use std::collections::HashSet;

use crate::utils::{
    grid::Grid,
    point::Pt,
    solver_types::{solve_simultaneous, SolutionSimultaneous},
};
use anyhow::{Ok, Result};

pub struct Day03Solution {}

pub fn day03(input: &str) -> Result<f32> {
    solve_simultaneous::<Day03Solution, _, _, _>(input)
}

#[derive(Clone, Copy, PartialEq)]
enum Obj {
    Symbol(char),
    Number(u32),
    None,
}

impl Default for Obj {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Default, Debug, Clone)]
struct Num {
    val: u32,
    pts: Vec<Pt<2>>,
}

impl SolutionSimultaneous<Grid<Obj, 2>, u32, u32> for Day03Solution {
    fn load(input: &str) -> Result<Grid<Obj, 2>> {
        let mut grid = Grid::default();

        for (y, line) in (0..).zip(input.split('\n')) {
            for (x, c) in (0..).zip(line.chars()) {
                match c {
                    '.' => (),
                    _ => {
                        let obj = if c.is_ascii_digit() {
                            Obj::Number(c.to_digit(10).unwrap())
                        } else {
                            Obj::Symbol(c)
                        };
                        grid.grid.insert(Pt([x, y]), obj);
                    }
                }
            }
        }
        Ok(grid)
    }

    fn solve(input: Grid<Obj, 2>) -> Result<(u32, u32)> {
        // parse numbers, symbols
        let mut nums = vec![];
        let mut symbols = HashSet::new();
        let mut gears = HashSet::new();
        let mut n = Num::default();

        for pt in input.iter_linear() {
            match input.get_def(pt) {
                Obj::None => (),
                Obj::Symbol(s) => {
                    if s == '*' {
                        gears.insert(pt);
                    }
                    symbols.insert(pt);
                    if n.val != 0 {
                        nums.push(n);
                        n = Num::default();
                    }
                }
                Obj::Number(v) => {
                    if let Some(p) = n.pts.last() {
                        if p.0[0] == pt.0[0] - 1 && p.0[1] == pt.0[1] {
                            // add to num
                            n.val = n.val * 10 + v;
                            n.pts.push(*pt);
                        } else {
                            // store and start new num
                            nums.push(n.clone());
                            n = Num {
                                val: v,
                                pts: vec![*pt],
                            }
                        }
                    } else {
                        // start new num
                        n = Num {
                            val: v,
                            pts: vec![*pt],
                        }
                    }
                }
            };
        }
        nums.push(n);

        // collect numbers that are part numbers.
        let mut sum = 0;
        let offsets = input.offsets.clone();
        for n in &nums {
            if n.pts
                .iter()
                .flat_map(|pt| offsets.iter().map(|ne| ne + pt))
                .any(|ne| symbols.contains(&ne))
            {
                sum += n.val;
            }
        }

        // collect numbers that are gear-adjacent
        let mut gear_sum = 0;
        for g in gears {
            let mut n_nums = vec![];
            let ns: HashSet<_> = offsets.iter().map(|ne| ne + g).collect();
            for n in &nums {
                if n.pts.iter().any(|n_pt| ns.contains(n_pt)) {
                    n_nums.push(n.val);
                }
            }

            if n_nums.len() == 2 {
                gear_sum += n_nums.iter().product::<u32>();
            }
        }

        Ok((sum, gear_sum))
    }
}

#[cfg(test)]
mod tests {
    use super::Day03Solution;
    use crate::utils::solver_types::SolutionSimultaneous;
    use rstest::rstest;

    #[rstest]
    #[case(
        "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
",
        4361,
        467835
    )]
    fn validate(#[case] input: &str, #[case] expected_1: u32, #[case] expected_2: u32) {
        let input = Day03Solution::load(input).unwrap();
        let (p1, p2) = Day03Solution::solve(input).unwrap();
        assert_eq!(expected_1, p1);
        assert_eq!(expected_2, p2);
    }
}

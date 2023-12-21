use crate::utils::{
    point::Pt,
    solver_types::{solve_linear, SolutionLinear},
};
use anyhow::Result;
use indexmap::IndexSet;
use itertools::Itertools;

pub struct Day21Solution {}

pub fn day21(input: &str) -> Result<f32> {
    solve_linear::<Day21Solution, _, _, _>(input)
}

const DIRS: [Pt<2>; 4] = [Pt([0, 1]), Pt([-1, 0]), Pt([1, 0]), Pt([0, -1])];

fn steppin(pts: &IndexSet<Pt<2>>, start: Pt<2>, steps: usize) -> usize {
    let Pt([max_x, max_y]) = pts.last().unwrap();
    (0..steps)
        .fold(vec![start], |acc, _| {
            acc.iter()
                .flat_map(|p| {
                    DIRS.iter().filter_map(|d| {
                        let new_d = *p + *d;
                        // https://www.youtube.com/watch?v=kpk2tdsPh0A&t=638s
                        let parallel = Pt([
                            new_d.0[0].rem_euclid(max_x + 1),
                            new_d.0[1].rem_euclid(max_y + 1),
                        ]);
                        if pts.contains(&parallel) {
                            Some(new_d)
                        } else {
                            None
                        }
                    })
                })
                .unique()
                .collect_vec()
        })
        .len()
}

fn find_best_fit(x1: usize, y1: usize, x2: usize, y2: usize, c: usize) -> (usize, usize) {
    for (a, b) in (10_000..30_000).cartesian_product(10_000..30_000) {
        if (a * x1.pow(2) + b * x1 + c) == y1 && (a * x2.pow(2) + b * x2 + c) == y2 {
            return (a, b);
        }
    }
    panic!("terms not in range")
}

impl SolutionLinear<(IndexSet<Pt<2>>, Pt<2>), usize, usize> for Day21Solution {
    fn load(input: &str) -> Result<(IndexSet<Pt<2>>, Pt<2>)> {
        let mut start = Pt([0, 0]);
        let mut set = IndexSet::new();

        for (y, l) in input.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                match c {
                    '#' => (),
                    _ => {
                        if c == 'S' {
                            start = Pt([x as isize, y as isize]);
                        }
                        set.insert(Pt([x as isize, y as isize]));
                    }
                }
            }
        }
        Ok((set, start))
    }

    fn part1((pts, start): &mut (IndexSet<Pt<2>>, Pt<2>)) -> Result<usize> {
        Ok(steppin(pts, *start, 64))
    }

    // the number of areas reachable increases quadratically with steps. SUSPICIOUSLY, the number 26501365 is equal to (grid_size * 202300) + 65.
    // so we can brute force finding the terms for our input, then find the 202300th term where t1 = c, t2 = c+grid_size, t3 = c+grid_size*2....
    fn part2(
        (pts, start): &mut (IndexSet<Pt<2>>, Pt<2>),
        _part_1_solution: usize,
    ) -> Result<usize> {
        let grid_size = pts.last().unwrap().0[0] as usize + 1;
        let t1 = steppin(pts, *start, 65);
        let t2 = steppin(pts, *start, 65 + grid_size);
        let t3 = steppin(pts, *start, 65 + grid_size + grid_size);

        let (a, b) = find_best_fit(1, t2, 2, t3, t1);

        Ok((202300_usize.pow(2) * a) + (202300 * b) + t1)
    }
}

#[cfg(test)]
mod tests {
    use super::Day21Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[ignore = "no answer for part 2"]
    #[case(
        "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........",
        16,
        1594
    )]
    fn validate_day21(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let mut input = Day21Solution::load(input).unwrap();

        let p1 = Day21Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);

        let p2 = Day21Solution::part2(&mut input, p1).unwrap();
        assert_eq!(expected_2, p2);
    }
}

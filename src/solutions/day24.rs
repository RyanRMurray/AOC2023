use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::{Ok, Result};
use itertools::Itertools;

pub struct Day24Solution {}

type HailStone = ([f64; 3], [f64; 3]);

pub fn day24(input: &str) -> Result<f32> {
    solve_linear::<Day24Solution, _, _, _>(input)
}

fn find_collision(
    ([x1, y1, _], [xv1, yv1, _]): HailStone,
    ([x2, y2, _], [xv2, yv2, _]): HailStone,
) -> Option<(f64, f64)> {
    let m1 = yv1 / xv1;
    let c1 = y1 - (m1 * x1);
    let m2 = yv2 / xv2;
    let c2 = y2 - (m2 * x2);
    if m1 == m2 {
        None
    } else {
        let d = (c2 - c1) / (m1 - m2);
        Some((d, (m1 * d) + c1))
    }
}

fn will_collide(
    ([x1, y1, _], [xv1, yv1, _]): HailStone,
    ([x2, y2, _], [xv2, yv2, _]): HailStone,
    x: f64,
    y: f64,
) -> bool {
    (x - x1).is_sign_negative() == xv1.is_sign_negative()
        && (x - x2).is_sign_negative() == xv2.is_sign_negative()
        && (y - y1).is_sign_negative() == yv1.is_sign_negative()
        && (y - y2).is_sign_negative() == yv2.is_sign_negative()
}

impl SolutionLinear<Vec<HailStone>, usize, usize> for Day24Solution {
    fn load(input: &str) -> Result<Vec<HailStone>> {
        Ok(input
            .lines()
            .map(|l| {
                let (at, vel) = l.split_once(" @ ").unwrap();
                (
                    at.split(", ")
                        .map(|v| v.trim().parse::<f64>().unwrap())
                        .collect_vec()
                        .try_into()
                        .unwrap(),
                    vel.split(", ")
                        .map(|v| v.trim().parse::<f64>().unwrap())
                        .collect_vec()
                        .try_into()
                        .unwrap(),
                )
            })
            .collect_vec())
    }

    fn part1(input: &mut Vec<HailStone>) -> Result<usize> {
        // let min_x = 200_000_000_000_000.0;
        // let max_x = 400_000_000_000_000.0;
        // let min_y =200_000_000_000_000.0;
        // let max_y = 400_000_000_000_000.0;
        let min_x = 7.0;
        let max_x = 27.0;
        let min_y = 7.0;
        let max_y = 27.0;
        let mut matched = 0;
        for i in 0..input.len() {
            for j in i + 1..input.len() {
                if let Some((cx, cy)) = find_collision(input[i], input[j]) {
                    if will_collide(input[i], input[j], cx, cy)
                        && cx >= min_x
                        && cx <= max_x
                        && cy >= min_y
                        && cy <= max_y
                    {
                        matched += 1;
                    }
                }
            }
        }

        Ok(matched)
    }

    fn part2(_input: &mut Vec<HailStone>, _part_1_solution: usize) -> Result<usize> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Day24Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[ignore = "not done"]
    #[case(
        "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
",
        2,
        47
    )]
    fn validate_day24(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let mut input = Day24Solution::load(input).unwrap();

        let p1 = Day24Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);

        let p2 = Day24Solution::part2(&mut input, p1).unwrap();
        assert_eq!(expected_2, p2);
    }
}

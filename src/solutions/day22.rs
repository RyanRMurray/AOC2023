use std::collections::{HashMap, HashSet};

use crate::utils::{
    point::Pt,
    solver_types::{solve_simultaneous, SolutionSimultaneous},
};
use anyhow::Result;
use itertools::Itertools;

pub struct Day22Solution {}

// bottom-left of cube, x width, y width, z height
#[derive(Debug)]
struct Block {
    k: Pt<3>,
    x_width: isize,
    y_width: isize,
    z_width: isize,
}

impl Block {
    // check if our bottom plane collides with a given block's top plane
    // assume our bottom plane z == their top plane z
    fn collides(&self, oth: &Block) -> bool {
        self.k.0[0] < oth.k.0[0] + oth.x_width
            && self.k.0[0] + self.x_width > oth.k.0[0]
            && self.k.0[1] < oth.k.0[1] + oth.y_width
            && self.k.0[1] + self.y_width > oth.k.0[1]
    }
    fn top_z(&self) -> isize {
        self.k.0[2] + self.z_width - 1
    }
}

pub fn day22(input: &str) -> Result<f32> {
    solve_simultaneous::<Day22Solution, _, _, _>(input)
}

/// from the bottom-most brick, check when its bottom plane would collide with a top plane, then set its z to the layer just above that.
/// return a map of blocks to what blocks they rest upon
fn settle(blocks: &mut Vec<Block>) -> HashMap<usize, Vec<usize>> {
    let mut m = HashMap::new();
    for i in 0..blocks.len() {
        if blocks[i].k.0[2] == 1 {
            continue;
        }

        let collides_with = blocks[0..i]
            .iter()
            .enumerate()
            .filter(|(_, b)| blocks[i].collides(b))
            .map(|(i, b)| (i, b.top_z()))
            .collect_vec();
        let c = collides_with.iter().max_by(|(_, z1), (_, z2)| z1.cmp(z2));
        if c.is_none() {
            blocks[i].k.0[2] = 1;
            continue;
        }
        let collides_at = c.unwrap().1;

        m.insert(
            i,
            collides_with
                .iter()
                .filter_map(|(j, b)| if b == &collides_at { Some(*j) } else { None })
                .collect_vec(),
        );

        blocks[i].k.0[2] = collides_at + 1;
    }
    m
}

fn cascade(a_is_on_b: &HashMap<usize, Vec<usize>>, target: usize) -> usize {
    let mut to_visit = vec![target];
    let mut falling: HashSet<usize> = HashSet::new();

    while let Some(v) = to_visit.pop() {
        falling.insert(v);
        let now_falling = a_is_on_b
            .iter()
            .filter_map(|(k, vs)| {
                if vs.iter().filter(|v| !falling.contains(v)).count() == 0 && !falling.contains(k) {
                    Some(k)
                } else {
                    None
                }
            })
            .unique();
        to_visit.extend(now_falling);
    }
    falling.len() - 1
}

impl SolutionSimultaneous<Vec<Block>, usize, usize> for Day22Solution {
    fn load(input: &str) -> Result<Vec<Block>> {
        let mut blocks = input
            .lines()
            .map(|l| {
                let (a, b) = l.split_once('~').unwrap();
                let [x1, y1, z1] = a
                    .split(',')
                    .map(|v| v.parse().unwrap())
                    .collect_vec()
                    .try_into()
                    .unwrap();
                let [x2, y2, z2]: [isize; 3] = b
                    .split(',')
                    .map(|v| v.parse().unwrap())
                    .collect_vec()
                    .try_into()
                    .unwrap();

                Block {
                    k: Pt([x1, y1, z1]),
                    x_width: x2 - x1 + 1,
                    y_width: y2 - y1 + 1,
                    z_width: z2 - z1 + 1,
                }
            })
            .collect_vec();
        blocks.sort_by(|a: &Block, b| a.k.0[2].cmp(&b.k.0[2]));

        Ok(blocks)
    }

    fn solve(mut input: Vec<Block>) -> Result<(usize, usize)> {
        let a_is_on_b = settle(&mut input);

        // create a set of all bricks that are the sole support of another - these can't be disintegrated!
        let required = a_is_on_b
            .iter()
            .filter_map(|(_, vs)| if vs.len() == 1 { Some(vs[0]) } else { None })
            .unique()
            .collect_vec();

        let cascading = required.iter().map(|i| cascade(&a_is_on_b, *i)).sum();

        Ok((input.len() - required.len(), cascading))
    }
}

#[cfg(test)]
mod tests {
    use super::Day22Solution;
    use crate::utils::solver_types::SolutionSimultaneous;
    use rstest::rstest;

    #[rstest]
    #[case(
        "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9",
        5,
        7
    )]
    fn validate_day22(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let input = Day22Solution::load(input).unwrap();

        let (p1, p2) = Day22Solution::solve(input).unwrap();
        assert_eq!(expected_1, p1);
        assert_eq!(expected_2, p2);
    }
}

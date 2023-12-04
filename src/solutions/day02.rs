use crate::utils::{
    load_input::load_lines,
    solver_types::{solve_simultaneous, SolutionSimultaneous},
};
use anyhow::Result;
use fancy_regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref RE: Regex = Regex::new(r"Game (\d+): (.+)*").unwrap();
}

pub struct Day02Solution {}

pub fn day02(input: &str) -> Result<f32> {
    solve_simultaneous::<Day02Solution, _, _, _>(input)
}

#[derive(Debug)]
struct Game {
    id: usize,
    rounds: Vec<[usize; 3]>,
}

fn parse_game(line: &str) -> Game {
    let parsed = RE.captures(line).unwrap().unwrap();
    let rounds: Vec<[usize; 3]> = parsed[2]
        .split("; ")
        .map(|round| {
            round.split(", ").fold([0; 3], |mut ctr, cubes| {
                let seen: Vec<&str> = cubes.split(' ').collect();
                let number: usize = seen.first().unwrap().parse().unwrap();
                match *seen.get(1).unwrap() {
                    "red" => ctr[0] += number,
                    "green" => ctr[1] += number,
                    "blue" => ctr[2] += number,
                    _ => panic!("unexpected colour"),
                }
                ctr
            })
        })
        .collect();

    Game {
        id: parsed[1].parse().unwrap(),
        rounds,
    }
}

fn more_than_target(target: [usize; 3], comp: [usize; 3]) -> bool {
    target[0] < comp[0] || target[1] < comp[1] || target[2] < comp[2]
}

impl SolutionSimultaneous<Vec<Game>, usize, usize> for Day02Solution {
    fn load(input: &str) -> Result<Vec<Game>> {
        Ok(load_lines(input, parse_game))
    }

    fn solve(input: Vec<Game>) -> Result<(usize, usize)> {
        let target = [12, 13, 14];
        let mut sum = 0;
        let mut powers = 0;

        for game in input {
            let mut maxes = [0; 3];
            for round in &game.rounds {
                maxes = [
                    round[0].max(maxes[0]),
                    round[1].max(maxes[1]),
                    round[2].max(maxes[2]),
                ]
            }
            if !more_than_target(target, maxes) {
                sum += game.id;
            }
            powers += maxes[0] * maxes[1] * maxes[2];
        }
        Ok((sum, powers))
    }
}

#[cfg(test)]
mod tests {
    use super::Day02Solution;
    use crate::utils::solver_types::SolutionSimultaneous;
    use rstest::rstest;

    #[rstest]
    #[case(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        8,
        2286
    )]
    fn validate(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let input = Day02Solution::load(input).unwrap();
        let (p1, p2) = Day02Solution::solve(input).unwrap();
        assert_eq!(expected_1, p1);
        assert_eq!(expected_2, p2);
    }
}

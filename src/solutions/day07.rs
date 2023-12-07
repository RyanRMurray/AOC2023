use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::Result;
use itertools::Itertools;

pub struct Day07Solution {}

pub fn day07(input: &str) -> Result<f32> {
    solve_linear::<Day07Solution, _, _, _>(input)
}

const RANKS: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

const JOKER_RANKS: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

type Hand = ([char; 5], usize);

fn hand_type(hand: &[char; 5]) -> usize {
    let mut idx = [' '; 5];
    let mut counts = [0; 5];

    for card in hand {
        if let Some(j) = idx.iter().position(|c| c == card) {
            counts[j] += 1;
        } else {
            let j = idx.iter().position(|c| *c == ' ').unwrap();
            idx[j] = *card;
            counts[j] += 1;
        }
    }
    counts.sort();

    match counts {
        [0, 0, 0, 0, 5] => 0, // five of a kind
        [0, 0, 0, 1, 4] => 1, // four of a kind
        [0, 0, 0, 2, 3] => 2, // full house
        [0, 0, 1, 1, 3] => 3, // three of a kind
        [0, 0, 1, 2, 2] => 4, // two pair
        [0, 1, 1, 1, 2] => 5, // one pir
        _ => 6,               // high card
    }
}

fn joker_hand_type(hand: &[char; 5]) -> usize {
    // check no jokers
    if !hand.iter().contains(&'J') {
        hand_type(hand)
    } else {
        // brute force it lol - THIS ONLY ADDS 0.1ms TO THE RUNTIME ON A REAL INPUT WE STAY WINNING
        RANKS
            .iter()
            .map(|replacement| {
                hand_type(
                    &hand
                        .iter()
                        .map(|c| if c == &'J' { *replacement } else { *c })
                        .collect_vec()
                        .try_into()
                        .unwrap(),
                )
            })
            .min()
            .unwrap()
    }
}

fn to_ranks(hand: &[char; 5], ranks: &[char; 13]) -> [usize; 5] {
    hand.iter()
        .map(|c| ranks.iter().position(|r| r == c).unwrap())
        .collect_vec()
        .try_into()
        .unwrap()
}

fn do_thing(hands: Vec<Hand>, hand_comp: fn(&[char; 5]) -> usize, rank_comp: &[char; 13]) -> usize {
    hands
        .iter()
        .map(|(hand, bid)| {
            // create a comparator index of hand type - [array of card ranks]
            ((hand_comp(hand), to_ranks(hand, rank_comp)), hand, bid)
        })
        .sorted_by(|(a, _, _), (b, _, _)| b.cmp(a)) // use that comparator (note this is a reverse order comparison)
        .zip(1..)
        .map(|((_, _, bid), i)| i * bid)
        .sum()
}

impl SolutionLinear<Vec<Hand>, usize, usize> for Day07Solution {
    fn load(input: &str) -> Result<Vec<Hand>> {
        Ok(input
            .lines()
            .map(|l| {
                let (c, b) = l.split_once(' ').unwrap();
                (
                    c.chars().collect_vec().try_into().unwrap(),
                    b.parse().unwrap(),
                )
            })
            .collect())
    }

    fn part1(input: &mut Vec<Hand>) -> Result<usize> {
        Ok(do_thing(input.to_vec(), hand_type, &RANKS))
    }

    fn part2(input: &mut Vec<Hand>, _part_1_solution: usize) -> Result<usize> {
        // do the same as part 1, but every time we get a card with a joker, brute force every type of hand it could be and pick the best

        Ok(do_thing(input.to_vec(), joker_hand_type, &JOKER_RANKS))
    }
}

#[cfg(test)]
mod tests {
    use super::Day07Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
",
        6440,
        5905
    )]
    fn validate(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let mut input = Day07Solution::load(input).unwrap();

        let p1 = Day07Solution::part1(&mut input).unwrap();
        assert_eq!(expected_1, p1);

        let p2 = Day07Solution::part2(&mut input, p1).unwrap();
        assert_eq!(expected_2, p2);
    }
}

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
pub mod templates;

use anyhow::Result;

/// Add new solutions to this const
pub const SOLUTIONS: [fn(&str) -> Result<f32>; 5] = [
    day01::day01,
    day02::day02,
    day03::day03,
    day04::day04,
    day05::day05,
];

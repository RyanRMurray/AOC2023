mod day01;
mod day02;
mod day03;
mod day04;
pub mod templates;

use anyhow::Result;

/// Add new solutions to this const
pub const SOLUTIONS: [fn(&str) -> Result<f32>; 4] =
    [day01::day01, day02::day02, day03::day03, day04::day04];

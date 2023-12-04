mod day01;
mod day02;
mod day03;
pub mod templates;

use anyhow::Result;

/// Add new solutions to this const
pub const SOLUTIONS: [fn(&str) -> Result<f32>; 3] = [day01::day01, day02::day02, day03::day03];

use anyhow::Result;
use std::{fmt::Display, time::Instant};

////////////// SOLUTION LINEAR
/// When a day has two parts that must be solved sequentially
pub trait SolutionLinear<I, S1: Display, S2: Display> {
    fn load(input: &str) -> Result<I>;
    fn part1(input: &mut I) -> Result<S1>;
    fn part2(input: &mut I, part_1_solution: S1) -> Result<S2>;
}

/// Solve a day where part 2 depends on the output of part 1.
/// Returns the total time elapsed in milliseconds
pub fn solve_linear<S: SolutionLinear<I, S1, S2>, I, S1: Display, S2: Display>(
    input: &str,
) -> Result<f32> {
    let start = Instant::now();

    let mut input = S::load(input)?;

    let input_loaded = start.elapsed().as_secs_f32() * 1000.0;

    println!("Parsed input in:\t{}ms", input_loaded);

    let p1_start = Instant::now();

    let p1 = S::part1(&mut input)?;

    let p1_end = p1_start.elapsed().as_secs_f32() * 1000.0;

    println!("Part 1 Solution: \t{}", p1);
    println!("Part 1 solved in:\t{}ms", p1_end);

    let p2_start = Instant::now();

    let p2 = S::part2(&mut input, p1)?;

    let p2_end = p2_start.elapsed().as_secs_f32() * 1000.0;

    println!("Part 2 Solution: \t{}", p2);
    println!("Part 2 solved in:\t{}ms", p2_end);

    let solved_in = input_loaded + p1_end + p2_end;

    println!("Overall time:\t\t{}ms", solved_in);

    Ok(solved_in)
}

////////////// SOLUTION SIMULTANEOUS
/// When a day has two parts that can be solved simultaneously
pub trait SolutionSimultaneous<I, S1: Display, S2: Display> {
    fn load(input: &str) -> Result<I>;
    fn solve(input: I) -> Result<(S1, S2)>;
}

/// Solve a day where part 1 and part 2 can be solved simultaneously
/// Returns the total time elapsed in milliseconds
pub fn solve_simultaneous<S: SolutionSimultaneous<I, S1, S2>, I, S1: Display, S2: Display>(
    input: &str,
) -> Result<f32> {
    let start = Instant::now();

    let input = S::load(input)?;

    let input_loaded = start.elapsed().as_secs_f32() * 1000.0;

    println!("Parsed input in:\t{}ms", input_loaded);

    let start_solving = Instant::now();

    let (p1, p2) = S::solve(input)?;

    let solved = start_solving.elapsed().as_secs_f32() * 1000.0;

    println!("Part 1 Solution: \t{}", p1);
    println!("Part 2 Solution: \t{}", p2);

    println!("Solved in:\t\t{}ms", solved);

    let solved_in = input_loaded + solved;

    println!("Overall time:\t\t{}ms", solved_in);

    Ok(solved_in)
}

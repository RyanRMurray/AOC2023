use std::{fs, path::Path};

use anyhow::{anyhow, Ok, Result};
use clap::{Parser, ValueEnum};
use solutions::SOLUTIONS;

use crate::{
    solutions::templates::{
        linear_template::ExampleSolutionLinear, simultaneous_template::ExampleSolutionSimultaneous,
    },
    utils::solver_types::{solve_linear, solve_simultaneous},
};

pub mod solutions;
pub mod utils;

#[derive(Debug, Clone, ValueEnum)]
enum RunMode {
    Example,
    Single,
    All,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, short, default_value = "example")]
    /// Example: run an example. Single: run a single day's solution. All: Run all solutions sequentially.
    mode: RunMode,
    /// Specify which day's solution to run - only used when --mode is single.
    #[arg(long, short, required_if_eq("mode", "single"))]
    day: Option<usize>,
    /// Specify the filepath to the day's input - only used when --mode is single
    #[arg(long, short)]
    input: Option<String>,
}

fn main() {
    let args = Args::parse();

    match args.mode {
        RunMode::Example => run_example(),
        RunMode::All => {
            let result = run_all();
            if let Err(err) = result {
                println!("Failed to run solutions. Reason: {}", err)
            }
        }
        RunMode::Single => {
            let result = run_single(args.day.unwrap(), args.input);

            if let Err(err) = result {
                println!(
                    "Failed to run solution for Day {}. Reason: {}",
                    args.day.unwrap(),
                    err
                )
            }
        }
    }
}

/// Load a puzzle input from a .txt file
fn load_from_file(file_path: &Path) -> Result<String> {
    if !file_path.is_file()
        || file_path.extension().is_none()
        || file_path.extension().unwrap() != "txt"
    {
        return Err(anyhow!(
            "input path '{:?}' is not valid. Please provide a path to a valid text file.",
            file_path.to_str()
        ));
    }

    match fs::read_to_string(file_path) {
        Err(error) => Err(anyhow!(
            "Failed to read file. Reason: {}",
            error.to_string()
        )),
        Result::Ok(text) => Ok(text),
    }
}

/// run a single specified day's solution
fn run_single(day: usize, input_path: Option<String>) -> Result<f32> {
    if day < 1 || day > SOLUTIONS.len() {
        return Err(anyhow!("Day '{}' is invalid or not yet solved", day));
    }

    let unwrapped_path = input_path.unwrap_or(format!("./inputs/input_{:02}.txt", day));
    let file_path = Path::new(&unwrapped_path);

    let input = load_from_file(file_path)?;

    SOLUTIONS[day - 1](&input)
}

/// run all solutions
fn run_all() -> Result<()> {
    let mut time_total = 0.0;

    for i in 0..SOLUTIONS.len() {
        println!("\nDay {:02}:\n", i + 1);
        time_total += run_single(i + 1, None)?;
    }

    println!("\nSolved all problems in: {}ms", time_total);

    Ok(())
}

fn run_example() {
    println!("Here's an example of a linear solution:");
    print!(
        r"
input: [1,2,3,4,5]
part 1: sum up these numbers
part 2: multiply the result of part 1 by the number of numbers in the input
"
    );

    let solved_in_1 = solve_linear::<ExampleSolutionLinear, _, _, _>("[1,2,3,4,5]").unwrap();

    println!("\nHere's an example of a simultaneous solution:");
    print!(
        r"
input: [6,5,4,2,3,5,8]
part 1: get the first number that's higher than the previous
part 2: get the number after the first number that's higher than the previous
"
    );

    let solved_in_2 =
        solve_simultaneous::<ExampleSolutionSimultaneous, _, _, _>("[6,5,4,2,3,5,8]").unwrap();

    println!("Example time:\t\t{}ms", solved_in_1 + solved_in_2);
}

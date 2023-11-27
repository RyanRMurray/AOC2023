# AOC 2023

Advent of Code is an event run by Eric Wastl. For each day leading up to Christmas, programmers are tasked with solving two small problems each day.
[Find more information here.](https://adventofcode.com/about)

## Using this project

You can run this project by calling

```bash
cargo build
./target/debug/AOC2023.exe -d [day number]  -i [input file path]
```

or

```bash
cargo run -- -d [day number] -i [input file path]
```

By default, `./inputs/input_{day}.txt` will be used as the input file path. For example, day 01 will use `./inputs/input_01.txt`

For more uses, run `cargo run -- --help`.

## Contribution

Before contributing, run the following:
```
pip install pre-commit
pre-commit install
```
This will run a number of linting actions and tests before you can commit to ensure your code works.

## Utils

### Pt
An point with an arbitrary number of dimensions. AoC frequently features both 2D and 2D spaces that need to be simulated. Occasionally, there's a curveball in the form of a 4D space.
These points assume that the spaces we need to represent are discrete - which is a safe bet since floating point operations in AoC are super rare.

Currently implements:
 - addition, multiplication, magnitude
 - Associated functions that return the offsets required to reach a point's neighbours:
   - `card_offsets` returns all cardinal (non-diagonal) offsets
   - 'neighbour_offsets` returns all offsets

### Grid
A grid with an arbitrary number of dimensions - using the above `Pt` as a key in a Hashmap of arbitrary items. Includes some helpful features:
 - Stores copies of `Pt`'s neighbour offsets so we don't have to recalculate them each time.
 - Generator functions:
   - `Default`, which creates an empty `Grid` of the specified dimensions
   - `From` a vector of `Pt`/values
   - `From` a vector of vectors/values. NOTE! We assume the vectors are of the same dimension as the `Grid`!
 - Default value that is returned if a point does not exist in a grid - helpful when we want to represent infinite space
 - `merge` for combining `Grid`s
 - `transform` for applying a transformation to all `Pt`s in a `Grid` (e.g. translation, multiplication, and God forbid, rotation.)
 - `bounds` for getting the minimum and maximum coordinate in each dimension
 - `print_2d` that creates a string representation of a 2D grid. Only 2D for now.

use super::grid::Grid;

/**
Helper functions for loading common input styles
*/

/// Load a string representation of a 2D grid into a Grid object
/// NOTE: We always assume the top-left elemnt is 0,0, and that x increases rightward and y increases downward
pub fn load_2d_grid<T: Copy + Default>(input: &str, to_value: fn(char) -> T) -> Grid<T, 2> {
    let mut pairs = vec![];
    for (y, line) in (0..).zip(input.split('\n')) {
        for (x, c) in (0..).zip(line.chars()) {
            pairs.push((vec![x, y], to_value(c)));
        }
    }

    Grid::from(pairs)
}

/// load values from an \n-seperated list
pub fn load_lines<T>(input: &str, to_value: fn(&str) -> T) -> Vec<T> {
    input.lines().map(to_value).collect()
}

/// load values from a list of \n-seperated list
pub fn load_segmented_lines<T>(
    input: &str,
    split_on: &str,
    to_value: fn(&str) -> T,
) -> Vec<Vec<T>> {
    input
        .split(split_on)
        .map(|seg| load_lines(seg, to_value))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::utils::{grid::Grid, load_input::load_2d_grid, point::Pt};

    #[test]
    fn test_load_2d_grid() {
        // note: ugly format so we don't lead with a \n
        let input = r#"123
456
789
"#
        .to_string();

        let expected = Grid::<u32, 2>::from(vec![
            (Pt([0, 0]), 1),
            (Pt([1, 0]), 2),
            (Pt([2, 0]), 3),
            (Pt([0, 1]), 4),
            (Pt([1, 1]), 5),
            (Pt([2, 1]), 6),
            (Pt([0, 2]), 7),
            (Pt([1, 2]), 8),
            (Pt([2, 2]), 9),
        ]);

        let result = load_2d_grid(&input, |v| v.to_digit(10).unwrap());

        assert_eq!(expected.grid, result.grid);
    }
}

use std::iter::zip;

use fxhash::FxHashMap;
use grid::{
    grid_array::GridArray,
    grid_iteration::adjacent_cell,
    grid_types::{Coor2DMut, Direction, Neighborhood, Topology},
};
use itertools::{EitherOrBoth, Itertools};
use num_traits::ToPrimitive;
use rayon::prelude::*;

use crate::custom_error::AocError;

//#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let grid =
        GridArray::from_newline_separated_string(Topology::Bounded, Neighborhood::Square, input);

    let search = "XMAS";

    let counter = grid
        .all_indexes()
        .map(|coor| {
            grid.all_adjacent_directions()
                .filter(|direction| check(search, &coor, &grid, *direction))
                .count()
        })
        .sum::<usize>();

    Ok(counter.to_string())
}

fn check(
    search: &str,
    coor: &Coor2DMut<usize>,
    grid: &GridArray<char>,
    direction: Direction,
) -> bool {
    let coor_sequence = (1..=search.len()).scan(Some(coor.clone()), |cur_coor_opt, index| {
        let last_corr_opt = cur_coor_opt.as_ref().cloned();
        *cur_coor_opt = match (&cur_coor_opt, index) {
            (None, _) => None,
            (Some(cur_coor), _) => grid.adjacent_cell(cur_coor.x, cur_coor.y, direction),
        };

        last_corr_opt
    });
    let char_sequence = coor_sequence
        .flat_map(|coor| grid.get(coor.x, coor.y))
        .collect::<String>();
    char_sequence == search
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() -> miette::Result<()> {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("18", process(input)?);
        Ok(())
    }

    #[test]
    fn test_part1() -> miette::Result<()> {
        let input = include_str!("../input1.txt");
        assert_eq!("2447", process(input)?);
        Ok(())
    }
}

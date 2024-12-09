use std::iter::zip;

use fxhash::FxHashMap;
use grid::{
    grid_array::GridArray,
    grid_iteration::adjacent_cell,
    grid_types::{Coor2DMut, Neighborhood, Topology},
};
use itertools::{EitherOrBoth, Itertools};
use num_traits::ToPrimitive;
use rayon::prelude::*;

use crate::custom_error::AocError;

//#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let grid =
        GridArray::from_newline_separated_string(Topology::Bounded, Neighborhood::Square, input);

    let search_grids = create_search_grids("M.S\n.A.\nM.S");

    let counter = grid
        .all_indexes()
        .filter(|coor| {
            search_grids.iter().any(|search_grid| {
                search_grid
                    .all_cells()
                    .all(|(search_grid_coor, search_grid_char)| {
                        check(&grid, coor.clone() + search_grid_coor, search_grid_char)
                    })
            })
        })
        .count();

    Ok(counter.to_string())
}

fn check(grid: &GridArray<char>, coor: Coor2DMut<usize>, search_grid_char: &char) -> bool {
    let found_char_opt = grid.get(coor.x, coor.y);
    matches!(found_char_opt,
             Some(found_char)
               if (found_char == search_grid_char) || (search_grid_char == &'.'))
}

fn create_search_grids(pattern: &str) -> Vec<GridArray<char>> {
    let mut search_grid =
        GridArray::from_newline_separated_string(Topology::Bounded, Neighborhood::Square, pattern);
    let mut search_grids = vec![search_grid.clone()];
    search_grid.rotate_ccw();
    search_grids.push(search_grid.clone());
    search_grid.rotate_ccw();
    search_grids.push(search_grid.clone());
    search_grid.rotate_ccw();
    search_grids.push(search_grid);
    search_grids
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_example() -> miette::Result<()> {
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
        assert_eq!("9", process(input)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> miette::Result<()> {
        let input = include_str!("../input2.txt");
        assert_eq!("1868", process(input)?);
        Ok(())
    }
}

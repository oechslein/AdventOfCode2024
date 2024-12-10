use std::collections::{HashMap, HashSet};

use fxhash::FxHashMap;
use grid::{
    grid_array::GridArray,
    grid_types::{Neighborhood, Topology, UCoor2D},
};
use itertools::Itertools;
use num_traits::ToPrimitive;
use rayon::prelude::*;

use crate::custom_error::AocError;

//#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let grid = GridArray::from_newline_separated_string_into(
        Topology::Bounded,
        Neighborhood::Orthogonal,
        input,
        |ch| ch.to_digit(10).unwrap() as usize,
    );
    let trail_head_positions = grid.all_cells().filter(|(_coor, height)| *height == &0);

    let result: usize = trail_head_positions
        .map(|(trail_head_pos, trail_head_height)| {
            let mut hiking_trails_ends_set = HashSet::new();
            hiking_trails_ends(
                &grid,
                trail_head_pos,
                *trail_head_height,
                &mut hiking_trails_ends_set,
            );
            hiking_trails_ends_set.len()
        })
        .sum();

    Ok(result.to_string())
}

fn hiking_trails_ends(
    grid: &GridArray<usize>,
    pos: UCoor2D,
    height: usize,
    hiking_trails_ends_set: &mut HashSet<UCoor2D>,
) {
    if height == 9 {
        hiking_trails_ends_set.insert(pos);
        return;
    }

    for (neighbor_coor, neighbor_height) in grid.neighborhood_cells(pos.x, pos.y) {
        if *neighbor_height == height + 1 {
            hiking_trails_ends(
                grid,
                neighbor_coor,
                *neighbor_height,
                hiking_trails_ends_set,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() -> miette::Result<()> {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!("36", process(input)?);
        Ok(())
    }

    #[test]
    fn test_part1() -> miette::Result<()> {
        let input = include_str!("../input1.txt");
        assert_eq!("468", process(input)?);
        Ok(())
    }
}

use std::collections::{HashMap, HashSet};

use fxhash::FxHashMap;
use grid::{
    grid_array::GridArray,
    grid_types::{Neighborhood, Topology, UCoor2D},
};
use itertools::Itertools;
use num_traits::ToPrimitive;
use rayon::prelude::*;



//#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let grid = GridArray::from_newline_separated_string_into(
        Topology::Bounded,
        Neighborhood::Orthogonal,
        input,
        |ch| ch.to_digit(10).unwrap() as usize,
    );
    let trail_head_positions = grid.all_cells().filter(|(_coor, height)| *height == &0);

    let mut cache: FxHashMap<UCoor2D, usize> = FxHashMap::default();
    let result: usize = trail_head_positions
        .map(|(trail_head_pos, trail_head_height)| {
            hiking_trails_count(&grid, trail_head_pos, *trail_head_height, &mut cache)
        })
        .sum();

    Ok(result.to_string())
}

fn hiking_trails_count(
    grid: &GridArray<usize>,
    pos: UCoor2D,
    height: usize,
    cache: &mut FxHashMap<UCoor2D, usize>,
) -> usize {
    if height == 9 {
        return 1;
    }

    if let Some(&count) = cache.get(&pos) {
        return count;
    }
    let count = grid.neighborhood_cells(pos.x, pos.y)
        .filter(|(_neighbor_coor, neighbor_height)| **neighbor_height == height + 1)
        .map(|(neighbor_coor, neighbor_height)| {
            hiking_trails_count(grid, neighbor_coor, *neighbor_height, cache)
        })
        .sum();
    cache.insert(pos, count);
    count
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
        assert_eq!("81", process(input)?);
        Ok(())
    }

    #[test]
    fn test_part1() -> miette::Result<()> {
        let input = include_str!("../input1.txt");
        assert_eq!("966", process(input)?);
        Ok(())
    }
}

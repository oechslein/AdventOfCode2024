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

pub fn process2(input: &str) -> miette::Result<String> {
    let grid = GridArray::from_newline_separated_string_into(
        Topology::Bounded,
        Neighborhood::Orthogonal,
        input,
        |ch| ch.to_digit(10).unwrap() as usize,
    );
    let summit_positions = grid.all_cells().filter(|(_coor, height)| *height == &9).collect_vec();
    let trail_head_positions = grid.all_cells().filter(|(_coor, height)| *height == &0);

    let mut cache: FxHashMap<UCoor2D, bool> = FxHashMap::default();
    let result: usize = trail_head_positions
        .map(|(trail_head_pos, _trail_head_height)| {
            summit_positions.iter().filter(|(summit_pos, _)| {
                hiking_trail_to_zero_exists(&grid, &trail_head_pos, summit_pos, &9,  &mut cache)
            }).count()
        })
        .sum();

    Ok(result.to_string())
}

fn hiking_trail_to_zero_exists(
    grid: &GridArray<usize>,
    goal: &UCoor2D,
    pos: &UCoor2D,
    height: &usize,
    cache: &mut FxHashMap<UCoor2D, bool>
) -> bool {
    if height == &0 {
        return goal == pos ;
    }

    if let Some(&count) = cache.get(pos) {
        return count;
    }
    let exists = grid.neighborhood_cells(pos.x, pos.y)
        .filter(|(_neighbor_coor, neighbor_height)| **neighbor_height == height - 1)
        .any(|(neighbor_coor, neighbor_height)| {
            hiking_trail_to_zero_exists(grid, goal, &neighbor_coor, neighbor_height, cache)
        });
    cache.insert(pos.clone(), exists);
    exists
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

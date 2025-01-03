use std::{collections::HashSet, os::windows::process};

use derive_more::derive::Display;
use fxhash::{FxHashMap, FxHashSet};
use grid::{
    grid_array::{GridArray, GridArrayBuilder},
    grid_iteration::{adjacent_cell, all_adjacent_directions},
    grid_types::{Direction, Neighborhood, Topology, UCoor2D},
};
use itertools::Itertools;
use num_traits::ToPrimitive;
use rayon::prelude::*;



#[derive(Debug, Clone)]
struct Region {
    plant: char,
    plots: FxHashSet<UCoor2D>,
}

//#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let grid =
        GridArray::from_newline_separated_string(Topology::Bounded, Neighborhood::Orthogonal, input);
    let mut processed_map: FxHashSet<UCoor2D> = FxHashSet::default();
    let all_plant_types = grid.iter().unique();
    let all_regions = all_plant_types
        .flat_map(|plant_type| create_regions(&grid, *plant_type, &mut processed_map));
    let result = all_regions
        .map(|region| region.area() * region.perimeter())
        .sum::<usize>();

    Ok(result.to_string())
}

impl Region {
    fn area(&self) -> usize {
        self.plots.len()
    }

    fn _contains(&self, coor: &UCoor2D, direction: Direction) -> bool {
        (coor.to_icoor2d().unwrap() + direction.diff_coor()).to_ucoor2d().map_or(false, |converted_corr| self.plots.contains(&converted_corr))
    }

    fn perimeter(&self) -> usize {
        self.plots.iter().map(|coor| self._perimiter_for_coor(coor)).sum()
    }

    fn _perimiter_for_coor(&self, coor: &UCoor2D) -> usize {
        all_adjacent_directions(Neighborhood::Orthogonal)
            .filter(|direction| !self._contains(coor, *direction))
            .count()
    }
}

fn create_regions(
    grid: &GridArray<char>,
    plant_type: char,
    processed_map: &mut FxHashSet<UCoor2D>,
) -> Vec<Region> {
    let mut regions = vec![];
    while let Some((coor, _plot)) = grid
        .all_cells()
        .find(|(coor, plot)| **plot == plant_type && !processed_map.contains(coor))
    {
        let region = create_region(grid, plant_type, coor, processed_map);
        //println!("{} {} {}", region.plant, region.area(), region.perimeter());
        regions.push(region);
    }
    regions
}

fn create_region(
    grid: &GridArray<char>,
    plant_type: char,
    coor: UCoor2D,
    processed_map: &mut FxHashSet<UCoor2D>,
) -> Region {
    let mut region = Region {
        plant: plant_type,
        plots: FxHashSet::default(),
    };
    region.plots.insert(coor.clone());
    processed_map.insert(coor.clone());
    let mut stack = vec![coor];
    while let Some(coor) = stack.pop() {
        for (neighbor_coor, neighbor_plot) in grid.neighborhood_cells(coor.x, coor.y) {
            if *neighbor_plot == plant_type && !processed_map.contains(&neighbor_coor) {
                processed_map.insert(neighbor_coor.clone());
                stack.push(neighbor_coor.clone());
                region.plots.insert(neighbor_coor);
            }
        }
    }
    region
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() -> miette::Result<()> {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!("1930", process(input)?);
        Ok(())
    }

    #[test]
    fn test_part1() -> miette::Result<()> {
        let input = include_str!("../input1.txt");
        assert_eq!("1473408", process(input)?);
        Ok(())
    }
}

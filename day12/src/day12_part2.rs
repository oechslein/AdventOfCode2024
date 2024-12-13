use derive_more::derive::Display;
use fxhash::{FxHashMap, FxHashSet};
use grid::{
    grid_array::{GridArray, GridArrayBuilder},
    grid_iteration::{adjacent_cell, all_adjacent_directions},
    grid_types::{Direction, Neighborhood, Topology, UCoor2D},
};
use itertools::Itertools;
use lazy_static::lazy_static;
use num_traits::ToPrimitive;
use rayon::prelude::*;

use crate::custom_error::AocError;

#[derive(Debug, Clone)]
struct Region {
    plant: char,
    plots: FxHashSet<UCoor2D>,
}

//#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let grid = GridArray::from_newline_separated_string(
        Topology::Bounded,
        Neighborhood::Orthogonal,
        input,
    );
    let mut processed_map: FxHashSet<UCoor2D> = FxHashSet::default();
    let all_plant_types = grid.iter().unique();
    let all_regions = all_plant_types
        .flat_map(|plant_type| create_regions(&grid, *plant_type, &mut processed_map));
    let result = all_regions
        .map(|region| region.area() * region.sides())
        .sum::<usize>();

    Ok(result.to_string())
}




lazy_static! {
    static ref CORNER_DIRECTIONS: Vec<(Direction, Direction)> = [Direction::West, Direction::East].into_iter().cartesian_product([Direction::North, Direction::South]).collect_vec();
}


impl Region {
    fn area(&self) -> usize {
        self.plots.len()
    }

    fn _contains(&self, coor: &UCoor2D, direction: Direction) -> bool {
        (coor.to_icoor2d().unwrap() + direction.diff_coor())
            .to_ucoor2d()
            .map_or(false, |converted_corr| self.plots.contains(&converted_corr))
    }


    fn sides(&self) -> usize {

        fn _combine_dirs(dir1: Direction, dir2: Direction) -> Direction {
            match (dir1, dir2) {
                (Direction::West, Direction::North) => Direction::NorthWest,
                (Direction::East, Direction::North) => Direction::NorthEast,
                (Direction::West, Direction::South) => Direction::SouthWest,
                (Direction::East, Direction::South) => Direction::SouthEast,
                _ => panic!("Invalid directions"),
            }
        }

        self.plots.iter().map(|coor| {
            let outer_corner_count = CORNER_DIRECTIONS.iter().filter(|(dir1, dir2)| {
                !self._contains(coor, *dir1) && !self._contains(coor, *dir2)
            }).count();
            let count = CORNER_DIRECTIONS.iter().filter(|(dir1, dir2)| {
                self._contains(coor, *dir1) && self._contains(coor, *dir2) && !self._contains(coor, _combine_dirs(*dir1, *dir2))
            }).count();
            outer_corner_count +
            count
        }).sum()
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
/*         println!(
            "{} {} sides: {}",
            region.plant,
            region.area(),
            region.sides()
        ); */
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
        assert_eq!("1206", process(input)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> miette::Result<()> {
        let input = include_str!("../input2.txt");
        assert_eq!("886364", process(input)?);
        Ok(())
    }
}

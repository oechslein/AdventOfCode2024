use fxhash::FxHashMap;
use grid::{
    grid_array::GridArray,
    grid_types::{Neighborhood, Topology},
};
use itertools::Itertools;
use num_traits::ToPrimitive;
use pathfinding::prelude::dijkstra;
use rayon::prelude::*;

use miette::{miette, Error, Result};

//#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    Ok(solve(input, 100).to_string())
}

fn solve(input: &str, min_saving_time: i32) -> usize {
    let grid = GridArray::from_newline_separated_string(
        Topology::Bounded,
        Neighborhood::Orthogonal,
        input,
    );

    let start_pos = grid
        .all_cells()
        .filter(|(_, &ch)| ch == 'S')
        .map(|(coor, _)| coor)
        .next()
        .unwrap();
    let end_pos = grid
        .all_cells()
        .filter(|(_, &ch)| ch == 'E')
        .map(|(coor, _)| coor)
        .next()
        .unwrap();
    let min_costs = dijkstra(
        &start_pos,
        |pos| {
            grid.neighborhood_cells(pos.x, pos.y)
                .filter(|(_, &neighbor_cell)| neighbor_cell != '#')
                .map(|(neighbor_coor, _)| (neighbor_coor, 1))
        },
        |pos| pos == &end_pos,
    )
    .unwrap()
    .1;

    let result = grid
        .all_cells()
        .par_bridge()
        .filter(|(_, &ch)| ch == '#')
        .map(|(coor, _)| coor)
        .filter(|wall_coor| {
            if let Some(result) = dijkstra(
                &start_pos,
                |pos| {
                    grid.neighborhood_cells(pos.x, pos.y)
                        .filter(|(neighbor_coor, &neighbor_cell)| {
                            neighbor_cell != '#' || neighbor_coor == wall_coor
                        })
                        .map(|(neighbor_coor, _)| (neighbor_coor, 1))
                },
                |pos| pos == &end_pos,
            ) {
                // println!(
                //     "wall_coor: {wall_coor:?} => {} {}",
                //     result.1,
                //     min_costs - result.1
                // );
                min_costs - result.1 >= min_saving_time
            } else {
                // println!("wall_coor: {wall_coor:?} => no solution");
                false
            }
        })
        .count();

    // 1396 too low
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> miette::Result<()> {
        let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        assert_eq!(14 + 14 + 2 + 4 + 2 + 3 + 1 + 1 + 1 + 1 + 1, solve(input, 1));
        Ok(())
    }

    #[test]
    fn test_input() -> miette::Result<()> {
        let input = include_str!("../input1.txt");
        assert_eq!("1422", process(&input.replace('\r', ""))?);
        Ok(())
    }
}

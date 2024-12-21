use fxhash::FxHashMap;
use grid::{
    grid_array::GridArray,
    grid_types::{Direction, Neighborhood, Topology},
};
use itertools::Itertools;
use pathfinding::prelude::astar_bag;

use miette::Result;

use crate::cache_it;

type MinimumPathType = FxHashMap<(char, char), (Vec<String>, usize)>;

//#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let numeric_paths = create_numeric_paths();
    let direction_paths = create_direction_paths();

    let result = input
        .lines()
        .map(|line| {
            let line = line.trim_end_matches('A');
            let min_costs = get_min_costs(
                line,
                &numeric_paths,
                |(shortest_paths, _costs): &(Vec<String>, usize)| -> usize {
                    shortest_paths
                        .iter()
                        .map(|path| {
                            find_shortest_sequence_directional_cached(path, 25, &direction_paths)
                        })
                        .min()
                        .unwrap()
                },
            );
            min_costs * line.parse::<usize>().unwrap()
        })
        .sum::<usize>();

    Ok(result.to_string())
}

fn create_numeric_paths() -> MinimumPathType {
    let numerical_keypad_grid = GridArray::from_newline_separated_string(
        Topology::Bounded,
        Neighborhood::Orthogonal,
        "789\n456\n123\n 0A",
    );

    find_shortest_paths(&numerical_keypad_grid)
}

fn create_direction_paths() -> MinimumPathType {
    let directional_keypad_grid = GridArray::from_newline_separated_string(
        Topology::Bounded,
        Neighborhood::Orthogonal,
        " ^A\n<v>",
    );

    find_shortest_paths(&directional_keypad_grid)
}

fn find_shortest_paths(grid: &GridArray<char>) -> MinimumPathType {
    let mut cache = FxHashMap::default();
    let all_indexes = grid
        .all_cells()
        .filter(|(_coor, cell)| cell != &&' ')
        .collect_vec();

    for ((start_coor, start), (end_coor, end)) in
        all_indexes.iter().cartesian_product(all_indexes.iter())
    {
        let (solutions, costs) = astar_bag(
            start_coor,
            |coor| {
                grid.neighborhood_cells(coor.x, coor.y)
                    .filter(|(_neighbor_coor, neighbor_cell)| neighbor_cell != &&' ')
                    .map(|(neighbor_coor, _neighbor_cell)| (neighbor_coor, 1))
            },
            |coor| coor.manhattan_distance(end_coor),
            |coor| coor == end_coor,
        )
        .unwrap();
        let solutions = solutions
            .map(|solution| {
                solution
                    .iter()
                    .tuple_windows()
                    .filter_map(|(coor1, coor2)| coor1.direction(coor2))
                    .map(|dir| match dir {
                        Direction::North => '^',
                        Direction::South => 'v',
                        Direction::West => '<',
                        Direction::East => '>',
                        _ => panic!("Invalid direction: {dir:?}"),
                    })
                    .collect::<String>()
            })
            .collect_vec();
        cache.insert((**start, **end), (solutions, costs));
    }
    cache
}

fn find_shortest_sequence_directional_cached(
    sequence: &str,
    depth: usize,
    minimum_directional_paths: &MinimumPathType,
) -> usize {
    cache_it!(
        FxHashMap<(String, usize), usize>,
        FxHashMap::default(),
        (sequence.to_string(), depth),
        {
            find_shortest_sequence_directional(sequence, depth, minimum_directional_paths)
        }
    )
}

fn find_shortest_sequence_directional(
    sequence: &str,
    depth: usize,
    minimum_directional_paths: &MinimumPathType,
) -> usize {
    get_min_costs(
        sequence,
        minimum_directional_paths,
        |(shortest_paths, costs): &(Vec<String>, usize)| -> usize {
            if depth == 1 {
                costs + 1 // Add 1 for the final 'A'
            } else {
                shortest_paths
                    .iter()
                    .map(|path| {
                        find_shortest_sequence_directional_cached(
                            path,
                            depth - 1,
                            minimum_directional_paths,
                        )
                    })
                    .min()
                    .unwrap()
            }
        },
    )
}

fn get_min_costs(
    sequence: &str,
    minimum_numeric_paths: &MinimumPathType,
    map_fn: impl Fn(&(Vec<String>, usize)) -> usize,
) -> usize {
    format!("A{sequence}A")
        .chars()
        .tuple_windows()
        .map(|(a, b)| r#map_fn(minimum_numeric_paths.get(&(a, b)).unwrap()))
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> miette::Result<()> {
        let input = "029A
980A
179A
456A
379A";
        assert_eq!("154115708116294", process(&input.replace('\r', ""))?);
        Ok(())
    }

    #[test]
    fn test_input() -> miette::Result<()> {
        let input = include_str!("../input2.txt");
        assert_eq!("231309103124520", process(&input.replace('\r', ""))?);
        Ok(())
    }
}

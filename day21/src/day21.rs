use fxhash::FxHashMap;
use grid::grid_array::GridArray;
use itertools::Itertools;
use pathfinding::prelude::astar_bag;

use grid::grid_types::Direction;
use grid::grid_types::Neighborhood;
use grid::grid_types::Topology;

use crate::cache_it;

type ShortestPathCacheType = FxHashMap<(char, char), (Vec<String>, usize)>;

pub(crate) fn solve(input: &str, depth: usize) -> usize {
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
                            find_shortest_sequence_directional_cached(path, depth, &direction_paths)
                        })
                        .min()
                        .unwrap()
                },
            );
            min_costs * line.parse::<usize>().unwrap()
        })
        .sum::<usize>();
    result
}

pub(crate) fn create_numeric_paths() -> ShortestPathCacheType {
    let numerical_keypad_grid = GridArray::from_newline_separated_string(
        Topology::Bounded,
        Neighborhood::Orthogonal,
        "789\n456\n123\n 0A",
    );

    find_shortest_paths(&numerical_keypad_grid)
}

pub(crate) fn create_direction_paths() -> ShortestPathCacheType {
    let directional_keypad_grid = GridArray::from_newline_separated_string(
        Topology::Bounded,
        Neighborhood::Orthogonal,
        " ^A\n<v>",
    );

    find_shortest_paths(&directional_keypad_grid)
}

pub(crate) fn find_shortest_paths(grid: &GridArray<char>) -> ShortestPathCacheType {
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

pub(crate) fn find_shortest_sequence_directional_cached(
    sequence: &str,
    depth: usize,
    minimum_directional_paths: &ShortestPathCacheType,
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

pub(crate) fn find_shortest_sequence_directional(
    sequence: &str,
    depth: usize,
    minimum_directional_paths: &ShortestPathCacheType,
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

pub(crate) fn get_min_costs(
    sequence: &str,
    minimum_numeric_paths: &ShortestPathCacheType,
    map_fn: impl Fn(&(Vec<String>, usize)) -> usize,
) -> usize {
    format!("A{sequence}A")
        .chars()
        .tuple_windows()
        .map(|(a, b)| r#map_fn(minimum_numeric_paths.get(&(a, b)).unwrap()))
        .sum::<usize>()
}

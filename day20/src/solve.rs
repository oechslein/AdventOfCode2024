use fxhash::FxHashMap;
use grid::grid_array::GridArray;
use grid::grid_types::Neighborhood;
use grid::grid_types::Topology;
use grid::grid_types::UCoor2D;
use pathfinding::prelude::dijkstra;
use rayon::prelude::*;

pub(crate) fn solve(input: &str, min_saving_time: usize, cheat_length: usize) -> usize {
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

    let (path, min_costs_without_cheat) = dijkstra(
        &start_pos,
        |pos| {
            grid.neighborhood_cells(pos.x, pos.y)
                .filter(|(_, &neighbor_cell)| neighbor_cell != '#')
                .map(|(neighbor_coor, _)| (neighbor_coor, 1))
        },
        |pos| pos == &end_pos,
    )
    .unwrap();

    let coor_to_costs: FxHashMap<UCoor2D, usize> = path
        .iter()
        .rev()
        .enumerate()
        .map(|(costs, coor)| (coor.clone(), costs))
        .collect();
    debug_assert_eq!(coor_to_costs[&end_pos], 0);
    debug_assert_eq!(coor_to_costs[&start_pos], min_costs_without_cheat);

    path.par_iter()
        .enumerate()
        .map(|(index, cheat_start_pos)| {
            path.iter()
                .skip(index + 2)
                .filter(|cheat_end_pos| {
                    let shortcut_costs = cheat_start_pos.manhattan_distance(cheat_end_pos);
                    (2..=cheat_length).contains(&shortcut_costs) && {
                        let cheat_costs: usize = coor_to_costs[cheat_end_pos] + shortcut_costs;
                        coor_to_costs[cheat_start_pos] - cheat_costs >= min_saving_time
                    }
                })
                .count()
        })
        .sum()
}

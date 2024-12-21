use fxhash::FxHashMap;
use grid::grid_array::GridArray;
use grid::grid_types::Neighborhood;
use grid::grid_types::Topology;
use grid::grid_types::UCoor2D;
use itertools::Itertools;
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

    let coor_to_costs: FxHashMap<&UCoor2D, usize> = path
        .iter()
        .rev()
        .enumerate()
        .map(|(costs, coor)| (coor, costs))
        .collect();
    debug_assert_eq!(coor_to_costs[&end_pos], 0);
    debug_assert_eq!(coor_to_costs[&start_pos], min_costs_without_cheat);

    path.par_iter()
        .enumerate()
        .map(|(index, cheat_start_pos)| {
            let cheat_start_costs = coor_to_costs[cheat_start_pos];
            path.iter()
                .skip(index + 2)
                .filter(|cheat_end_pos| {
                    let shortcut_costs = cheat_start_pos.manhattan_distance(cheat_end_pos);
                    (2..=cheat_length).contains(&shortcut_costs) && {
                        let cheat_end_costs: usize = coor_to_costs[cheat_end_pos];
                        let cheat_costs: usize = cheat_end_costs + shortcut_costs;
                        cheat_start_costs - cheat_costs >= min_saving_time
                    }
                })
                .count()
        })
        .sum()

    // path.iter()
    // .tuple_combinations()
    // .par_bridge()
    // .map(|(cheat_start_pos, cheat_end_pos)| {
    //     let shortcut_costs = cheat_start_pos.manhattan_distance(cheat_end_pos);
    //     (2..=cheat_length).contains(&shortcut_costs) && {
    //         let cheat_start_costs = coor_to_costs[cheat_end_pos];
    //         let cheat_end_costs: usize = coor_to_costs[cheat_end_pos];
    //         let cheat_costs: usize = cheat_end_costs + shortcut_costs;
    //         cheat_start_costs - cheat_costs >= min_saving_time
    //     }
    // })
    // .count()

    // path.par_iter()
    //     .map(|cheat_start_pos| {
    //         let cheat_start_costs = coor_to_costs[cheat_start_pos];
    //         get_positions_around(cheat_length, cheat_start_pos)
    //             .filter_map(|coor| coor_to_costs.get(&coor).map(|costs| (coor, costs)))
    //             .filter(|(_cheat_end_pos, &cheat_end_costs)| cheat_end_costs < cheat_start_costs)
    //             .filter(|(cheat_end_pos, &cheat_end_costs)| {
    //                 let shortcut_costs = cheat_start_pos.manhattan_distance(cheat_end_pos);
    //                 (2..=cheat_length).contains(&shortcut_costs) && {
    //                     let cheat_costs: usize = cheat_end_costs + shortcut_costs;
    //                     cheat_start_costs >= min_saving_time + cheat_costs
    //                 }
    //             })
    //             .count()
    //     })
    //     .sum()
}

fn get_positions_around(
    cheat_length: usize,
    cheat_start_pos: &UCoor2D,
) -> impl Iterator<Item = UCoor2D> {
    let map = (cheat_start_pos.x.saturating_sub(cheat_length)..=cheat_start_pos.x + cheat_length)
        .cartesian_product(
            cheat_start_pos.y.saturating_sub(cheat_length)..=cheat_start_pos.y + cheat_length,
        )
        .map(|(x, y)| UCoor2D::new(x, y));
    map
}

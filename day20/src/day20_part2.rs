use std::collections::VecDeque;

use fxhash::FxHashMap;
use grid::{
    grid_array::GridArray,
    grid_types::{Neighborhood, Topology, UCoor2D},
};
use itertools::Itertools;
use num_traits::ToPrimitive;
use pathfinding::prelude::{bfs_reach, dijkstra};
use rayon::prelude::*;

use miette::{miette, Error, Result};

//#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    Ok(solve(input, 50).to_string())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    cheat_start_pos: Option<UCoor2D>,
    cheat_end_pos: Option<UCoor2D>,
    cheat_time_left: usize,

    current_pos: UCoor2D,
}

impl Node {
    fn new(current_pos: UCoor2D) -> Self {
        Self {
            cheat_start_pos: None,
            cheat_end_pos: None,
            cheat_time_left: 20 - 1,
            current_pos,
        }
    }

    fn successors(&self, grid: &GridArray<char>) -> Vec<(Self, usize)> {
        let mut result = vec![];
        for (neighbor_coor, &neighbor_cell) in
            grid.neighborhood_cells(self.current_pos.x, self.current_pos.y)
        {
            let mut neighboor_node = self.clone();
            neighboor_node.current_pos = neighbor_coor.clone();

            if neighbor_cell != '#' {
                // no wall
                if self.cheat_start_pos.is_some() && self.cheat_end_pos.is_none() {
                    // cheat running
                    neighboor_node.cheat_time_left -= 1;
                    if neighboor_node.cheat_time_left == 0 {
                        neighboor_node.cheat_end_pos = Some(neighbor_coor.clone());
                    }
                }

            //// from here on we have a wall
            } else if self.cheat_end_pos.is_some() {
                // cheat activated and ended, skip neighboor/wall
                continue;
            } else if self.cheat_start_pos.is_some() {
                // we started the cheat before, but it is still running
                debug_assert!(self.cheat_time_left > 0);
                neighboor_node.cheat_time_left -= 1;
                if neighboor_node.cheat_time_left == 0 {
                    neighboor_node.cheat_end_pos = Some(neighbor_coor.clone());
                }
            } else {
                // we can start the cheat now
                neighboor_node.cheat_start_pos = Some(neighbor_coor.clone());
                neighboor_node.cheat_time_left -= 1;
                if neighboor_node.cheat_time_left == 0 {
                    neighboor_node.cheat_end_pos = Some(neighbor_coor.clone());
                }
            }

            result.push((neighboor_node, 1));
        }
        result
    }

    fn success(&self, end_coor: &UCoor2D) -> bool {
        self.current_pos == *end_coor
    }
}

fn solve(input: &str, min_saving_time: usize) -> usize {
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

    let min_costs_without_cheat = dijkstra(
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

    let mut visited: FxHashMap<Node, usize> = FxHashMap::default();
    let mut open = VecDeque::new();
    let mut success = Vec::new();
    open.push_back((Node::new(start_pos), 0));

    while let Some((node, costs)) = open.pop_front() {
        if min_costs_without_cheat - costs < min_saving_time {
            continue;
        }
        if visited.contains_key(&node) {
            // we could have visited this node before but with a higher cost
            // => reopen it
            if costs < visited[&node] {
                visited.remove(&node);
                open.push_back((node.clone(), costs));
            }
            continue;
        }

        visited.insert(node.clone(), costs);

        if node.success(&end_pos) {
            success.push((node.clone(), costs));
            continue;
        }

        for (successor, successor_cost) in node.successors(&grid) {
            open.push_back((successor, costs + successor_cost));
        }
    }

    let mut counter = FxHashMap::default();
    for (node, costs) in &success {
        if true {
            *counter
                .entry((
                    min_costs_without_cheat - costs,
                    node.cheat_start_pos.clone(),
                    node.cheat_end_pos.clone(),
                ))
                .or_insert(0) += 1;
        }
    }
    println!("{:?}", counter);

    // only count individual cheats if start and end are not the same

    counter.keys().count()
}

fn solve_slow(input: &str, min_saving_time: i32) -> usize {
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

    grid.all_cells()
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
        .count()
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
        assert_eq!(
            32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3, // 285
            solve(input, 50)
        );
        Ok(())
    }

    #[test]
    fn test_input() -> miette::Result<()> {
        let input = include_str!("../input2.txt");
        assert_eq!("285", process(&input.replace('\r', ""))?);
        Ok(())
    }
}

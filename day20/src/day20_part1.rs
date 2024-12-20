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
    Ok(solve(input, 100).to_string())
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
            cheat_time_left: 2,
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

            if neighboor_node.cheat_start_pos.is_some() && neighboor_node.cheat_end_pos.is_none() {
                // cheat is running
                neighboor_node.cheat_time_left -= 1;
                if neighboor_node.cheat_time_left == 0 {
                    neighboor_node.cheat_end_pos = Some(neighbor_coor.clone());
                }
            }

            if neighbor_cell != '#' {
                // no wall

                ////
                // here we have a wall
            } else if neighboor_node.cheat_end_pos.is_some() {
                // cheat activated and ended, skip wall
                continue;
            } else if neighboor_node.cheat_start_pos.is_some() {
                // we started the cheat before
            } else {
                // we can start the cheat now
                neighboor_node.cheat_start_pos = Some(neighbor_coor.clone());
                neighboor_node.cheat_time_left -= 1;
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
        // we could have visited this node before but with a higher cost
        if visited.contains_key(&node) {
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

    success
        .iter()
        .map(|(_, costs)| costs)
        .filter(|&costs| {
            (costs <= &min_costs_without_cheat)
                && (min_costs_without_cheat - costs >= min_saving_time)
        })
        .count()
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

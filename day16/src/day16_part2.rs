use fxhash::{FxHashMap, FxHashSet};
use grid::{
    grid_array::GridArray,
    grid_types::{Direction, Neighborhood, Topology, UCoor2D},
};
use itertools::Itertools;
use num_traits::ToPrimitive;
use pathfinding::prelude::{astar, astar_bag, dijkstra, dijkstra_all};
use rayon::prelude::*;

use crate::custom_error::AocError;

//#[tracing::instrument]

#[derive(Debug, Clone, PartialEq, Hash)]
struct Node {
    coor: UCoor2D,
    direction: Direction,
}

impl Eq for Node {}

impl Node {
    fn successors(&self, grid: &GridArray<char>) -> Vec<(Node, usize)> {
        grid.neighborhood_cells_and_dirs(self.coor.x, self.coor.y)
            .filter(|(_coor, _direction, cell)| cell != &&'#')
            .map(|(coor, direction, _cell)| {
                let costs = if direction == self.direction {
                    1
                } else if (direction == self.direction.rotate(90))
                    || (direction == self.direction.rotate(-90))
                {
                    1000 + 1
                } else {
                    debug_assert_eq!(direction, self.direction.rotate(180));
                    2 * 1000 + 1
                };
                (Node { coor, direction }, costs)
            })
            .collect_vec()
    }

    fn success(&self, grid: &GridArray<char>) -> bool {
        grid.get_unchecked(self.coor.x, self.coor.y) == &'E'
    }

    fn heuristic(&self, end_coor: &UCoor2D) -> usize {
        self.coor.manhattan_distance(end_coor)
    }
}

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let grid = GridArray::from_newline_separated_string(
        Topology::Bounded,
        Neighborhood::Orthogonal,
        input,
    );

    let start_coor = grid
        .all_cells()
        .filter(|(_coor, cell)| cell == &&'S')
        .map(|(coor, _cell)| coor)
        .next()
        .unwrap();
    let end_coor = grid
        .all_cells()
        .filter(|(_coor, cell)| cell == &&'E')
        .map(|(coor, _cell)| coor)
        .next()
        .unwrap();
    let start = Node {
        coor: start_coor,
        direction: Direction::East,
    };
    let result = astar_bag(
        &start,
        |node| node.successors(&grid),
        |node| node.heuristic(&end_coor),
        |node| node.success(&grid),
    );

    let solutions = result.unwrap().0;
    let unique_coors = solutions
        .flat_map(|path| path.into_iter().map(|n| n.coor))
        .unique()
        .count();

    Ok(unique_coors.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> miette::Result<()> {
        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        assert_eq!("45", process(&input.replace('\r', ""))?);
        Ok(())
    }

    #[test]
    fn test_example2() -> miette::Result<()> {
        let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        assert_eq!("64", process(&input.replace('\r', ""))?);
        Ok(())
    }

    #[test]
    fn test_input() -> miette::Result<()> {
        let input = include_str!("../input2.txt");
        assert_eq!("665", process(&input.replace('\r', ""))?);
        Ok(())
    }
}

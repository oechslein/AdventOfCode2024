use grid::{
    grid_array::GridArray,
    grid_types::{Direction, ICoor2D, Neighborhood, Topology, UCoor2D},
};
use itertools::Itertools;
use pathfinding::prelude::astar_bag;

use crate::custom_error::AocError;

//#[tracing::instrument]

#[derive(Debug, Clone, PartialEq, Hash)]
struct Node {
    coor: UCoor2D,
    direction: Direction,
}

impl Eq for Node {}

impl Node {
    fn get_start_node(grid: &GridArray<char>) -> Node {
        Node {
            coor: find_cells_coor(grid, 'S'),
            direction: Direction::East,
        }
    }

    fn successors(&self, grid: &GridArray<char>) -> Vec<(Node, usize)> {
        let mut result = Vec::with_capacity(3);
        result.push((
            Node {
                coor: self.coor.clone(),
                direction: self.direction.rotate(90),
            },
            1000,
        ));
        result.push((
            Node {
                coor: self.coor.clone(),
                direction: self.direction.rotate(-90),
            },
            1000,
        ));
        if let Some(new_coor) =
            (self.coor.to_icoor2d().unwrap() + self.direction.diff_coor()).to_ucoor2d()
        {
            if grid.get_unchecked(new_coor.x, new_coor.y) != &'#' {
                // we can move in current direction
                result.push((
                    Node {
                        coor: new_coor,
                        direction: self.direction,
                    },
                    1,
                ));
            }
        }
        result
    }

    fn success(&self, grid: &GridArray<char>) -> bool {
        grid.get_unchecked(self.coor.x, self.coor.y) == &'E'
    }

    fn heuristic(&self, end_coor: &UCoor2D) -> usize {
        self.coor.manhattan_distance(end_coor)
    }
}

pub fn process(input: &str) -> miette::Result<String> {
    let grid = GridArray::from_newline_separated_string(
        Topology::Bounded,
        Neighborhood::Orthogonal,
        input,
    );

    let end_coor = find_cells_coor(&grid, 'E');

    let solutions = astar_bag(
        &Node::get_start_node(&grid),
        |node| node.successors(&grid),
        |node| node.heuristic(&end_coor),
        |node| node.success(&grid),
    );

    Ok(solutions
        .map(|(solutions, _min_costs)| {
            solutions
                .flat_map(|path| path.into_iter().map(|n| n.coor))
                .unique()
                .count()
                .to_string()
        })
        .unwrap())
}

fn find_cells_coor(
    grid: &GridArray<char>,
    cell_to_find: char,
) -> grid::grid_types::Coor2DMut<usize> {
    let start_coor = grid
        .all_cells()
        .filter(|(_coor, cell)| cell == &&cell_to_find)
        .map(|(coor, _cell)| coor)
        .next()
        .unwrap();
    start_coor
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

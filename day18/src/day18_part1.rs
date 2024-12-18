use grid::{
    grid_array::{GridArray, GridArrayBuilder},
    grid_types::{Neighborhood, Topology, UCoor2D},
};
use itertools::Itertools;
use pathfinding::prelude::*;

use miette::{Error, Result};

//#[tracing::instrument]
pub fn process(input: &str, width: usize, bytes_to_take: usize) -> Result<String, Error> {
    let mut grid = GridArrayBuilder::default().topology(Topology::Bounded).neighborhood(
        Neighborhood::Orthogonal).width(width).height(width).build().unwrap();

    for x in 0..width {
        for y in 0..width {
            grid.set(x, y, '.');
        }
    }

    for line in input.lines().take(bytes_to_take) {
        let (x_str, y_str) = line.split_once(',').unwrap();
        grid.set(
            x_str.parse::<usize>().unwrap(),
            y_str.parse::<usize>().unwrap(),
            '#',
        );
    }
    //grid.println(false);

    let result = astar(
        &UCoor2D::new(0, 0),
        |coor| successors(coor, &grid),
        |coor| heuristic(coor, &grid),
        |coor| success(coor, &grid),
    );
    let min_costs = &result.unwrap().1;
    Ok(min_costs.to_string())
}

fn successors(coor: &UCoor2D, grid: &GridArray<char>) -> Vec<(UCoor2D, usize)> {
    grid.neighborhood_cells(coor.x, coor.y)
        .filter(|(_coor, cell)| cell != &&'#')
        .map(|(coor, _cell)| (coor, 1))
        .collect_vec()
}

fn success(coor: &UCoor2D, grid: &GridArray<char>) -> bool {
    *coor == end_coor(grid)
}

fn end_coor(grid: &GridArray<char>) -> UCoor2D {
    UCoor2D::new(grid.width() - 1, grid.height() - 1)
}

fn heuristic(coor: &UCoor2D, grid: &GridArray<char>) -> usize {
    coor.manhattan_distance(&end_coor(grid))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> miette::Result<()> {
        let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        assert_eq!("22", process(&input.replace('\r', ""), 6+1, 12)?);
        Ok(())
    }

    #[test]
    fn test_input() -> miette::Result<()> {
        let input = include_str!("../input1.txt");
        assert_eq!("276", process(&input.replace('\r', ""), 70+1, 1024)?);
        Ok(())
    }
}

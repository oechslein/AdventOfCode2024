use fxhash::{FxHashMap, FxHashSet};
use grid::{
    grid_array::{GridArray, GridArrayBuilder},
    grid_hashmap::GridHashMap,
    grid_types::{Neighborhood, Topology, UCoor2D},
};
use itertools::Itertools;
use num_traits::ToPrimitive;
use pathfinding::prelude::*;
use rayon::prelude::*;

use miette::{miette, Error, Result};

//#[tracing::instrument]
pub fn process(input: &str, width: usize) -> std::result::Result<String, Error> {
    //let width = 6+1;
    let mut grid = GridArrayBuilder::default().topology(Topology::Bounded).neighborhood(
        Neighborhood::Orthogonal).width(width).height(width).build().unwrap();

    for x in 0..width {
        for y in 0..width {
            grid.set(x, y, '.');
        }
    }

    let lines = input.lines().collect_vec();

    let mut last_entry = "";
    for (index, line) in lines.iter().enumerate() {
        let (x_str, y_str) = line.split_once(',').unwrap();
        grid.set(
            x_str.parse::<usize>().unwrap(),
            y_str.parse::<usize>().unwrap(),
            '#',
        );
        //grid.println(false);

        let start_coor = UCoor2D::new(0, 0);

        let result = astar(
            &start_coor,
            |coor| successors(coor, &grid),
            |coor| heuristic(coor, &grid),
            |coor| success(coor, &grid),
        );
        //println!("{:?} {:?} {:?}", bytes_to_take, input.lines().nth(bytes_to_take-1).unwrap(), result.clone().and_then(|r| Some(r.1)));
        if result.is_none() {
            last_entry = input.lines().nth(index).unwrap();
            break;
        }
    }
    //let min_costs = &result.unwrap().1;
    Ok(last_entry.to_string())
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
        assert_eq!("6,1", process(&input.replace('\r', ""), 6+1)?);
        Ok(())
    }

    #[test]
    fn test_input() -> miette::Result<()> {
        let input = include_str!("../input2.txt");
        assert_eq!("60,37", process(&input.replace('\r', ""), 70+1)?);
        Ok(())
    }
}

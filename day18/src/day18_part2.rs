use fxhash::FxHashMap;
use grid::{
    grid_iteration::{adjacent_cell, all_adjacent_directions},
    grid_types::{Neighborhood, Topology, UCoor2D},
};
use pathfinding::prelude::*;

use miette::{miette, Error, Result};

//#[tracing::instrument]
pub fn process(input: &str, width: usize) -> Result<String, Error> {
    //let width = 6+1;

    let maze = Maze::parse(input, width);
    binary_search(0, maze.walls_with_time.len(), |mid| maze.find_path(mid))
        .and_then(|found_time| input.lines().nth(found_time))
        .map(ToString::to_string)
        .ok_or(miette!("No path blocker found!"))
}

#[derive(Debug)]
struct Maze {
    width: usize,
    walls_with_time: FxHashMap<UCoor2D, usize>,
}

impl Maze {
    fn parse(input: &str, width: usize) -> Maze {
        let walls_with_time = input
            .lines()
            .enumerate()
            .map(|(i, line)| {
                let (x_str, y_str) = line.split_once(',').unwrap();

                (
                    UCoor2D::new(
                        x_str.parse::<usize>().unwrap(),
                        y_str.parse::<usize>().unwrap(),
                    ),
                    i,
                )
            })
            .collect();
        Maze {
            width,
            walls_with_time,
        }
    }

    fn find_path(&self, time: usize) -> bool {
        astar(
            &UCoor2D::new(0, 0),
            |coor| self.successors(coor, time),
            |coor| self.heuristic(coor),
            |coor| self.success(coor),
        )
        .is_some()
    }

    fn successors(&self, coor: &UCoor2D, time: usize) -> Vec<(UCoor2D, usize)> {
        all_adjacent_directions(Neighborhood::Orthogonal)
            .filter_map(|dir| {
                adjacent_cell(Topology::Bounded, self.width, self.width, coor.clone(), dir)
            })
            .filter(|new_coor| !self.is_occupied_by_wall(new_coor, time))
            .map(|new_coor| (new_coor, 1))
            .collect()
    }

    fn is_occupied_by_wall(&self, new_coor: &UCoor2D, time: usize) -> bool {
        self.walls_with_time
            .get(new_coor)
            .map_or(false, |wall_fall_time| wall_fall_time < &time)
    }

    fn success(&self, coor: &UCoor2D) -> bool {
        *coor == self.coor_goal()
    }

    fn heuristic(&self, coor: &UCoor2D) -> usize {
        coor.manhattan_distance(&self.coor_goal())
    }
    fn coor_goal(&self) -> UCoor2D {
        UCoor2D::new(self.width - 1, self.width - 1)
    }
}

fn binary_search(
    lower_end: usize,
    upper_end: usize,
    match_fn: impl Fn(usize) -> bool,
) -> Option<usize> {
    let mut lower_end = lower_end;
    let mut upper_end = upper_end;

    while lower_end < upper_end {
        let mid = (lower_end + upper_end) / 2;
        if match_fn(mid) {
            lower_end = mid + 1;
        } else {
            upper_end = mid;
        }
    }

    if match_fn(upper_end - 1) {
        Some(upper_end - 1)
    } else {
        None
    }
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
        assert_eq!("6,1", process(&input.replace('\r', ""), 6 + 1)?);
        Ok(())
    }

    #[test]
    fn test_input() -> miette::Result<()> {
        let input = include_str!("../input2.txt");
        assert_eq!("60,37", process(&input.replace('\r', ""), 70 + 1)?);
        Ok(())
    }
}

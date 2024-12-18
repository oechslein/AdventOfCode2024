use fxhash::FxHashSet;
use grid::{
    grid_iteration::adjacent_cell,
    grid_types::{Direction, Topology, UCoor2D},
};
use pathfinding::prelude::*;

use miette::{Error, Result};

//#[tracing::instrument]
pub fn process(input: &str, width: usize) -> Result<String, Error> {
    //let width = 6+1;

    let wall_coors = parse(input);
    let previous_mid = binary_search(0, wall_coors.len(), |mid| {
        find_path(width, mid, &wall_coors)
    })
    .unwrap();
    let result = input.lines().nth(previous_mid).unwrap();

    Ok(result.to_string())
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

fn parse(input: &str) -> Vec<UCoor2D> {
    input
        .lines()
        .map(|line| {
            let (x_str, y_str) = line.split_once(',').unwrap();

            UCoor2D::new(
                x_str.parse::<usize>().unwrap(),
                y_str.parse::<usize>().unwrap(),
            )
        })
        .collect()
}

fn create_wall_set(bytes_to_take: usize, wall_coors: &[UCoor2D]) -> FxHashSet<&UCoor2D> {
    wall_coors.iter().take(bytes_to_take).collect()
}

fn find_path(width: usize, bytes_to_take: usize, wall_coors: &[UCoor2D]) -> bool {
    let wall_set: FxHashSet<&UCoor2D> = create_wall_set(bytes_to_take, wall_coors);
    let result = astar(
        &UCoor2D::new(0, 0),
        |coor| successors_wall_set(coor, width, &wall_set),
        |coor| heuristic_wall_set(coor, width),
        |coor| success_wall_set(coor, width),
    );
    //println!("{:?} {:?} {:?}", bytes_to_take, input.lines().nth(bytes_to_take-1).unwrap(), result.clone().and_then(|r| Some(r.1)));
    result.is_some()
}

fn successors_wall_set(
    coor: &UCoor2D,
    width: usize,
    wall_set: &FxHashSet<&UCoor2D>,
) -> Vec<(UCoor2D, usize)> {
    vec![
        Direction::West,
        Direction::East,
        Direction::South,
        Direction::North,
    ]
    .into_iter()
    .filter_map(|dir| adjacent_cell(Topology::Bounded, width, width, coor.clone(), dir))
    .filter(|new_coor| !wall_set.contains(new_coor))
    .map(|new_coor| (new_coor, 1))
    .collect()
}

fn success_wall_set(coor: &UCoor2D, width: usize) -> bool {
    *coor == end_coor_wall_set(width)
}

fn end_coor_wall_set(width: usize) -> UCoor2D {
    UCoor2D::new(width - 1, width - 1)
}

fn heuristic_wall_set(coor: &UCoor2D, width: usize) -> usize {
    coor.manhattan_distance(&end_coor_wall_set(width))
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

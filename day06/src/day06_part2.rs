use std::collections::HashSet;


use grid::{grid_array::GridArray, grid_types::UCoor2D};

use grid::grid_types::Direction;

use grid::grid_types::{Neighborhood, Topology};

use itertools::Itertools;
use rayon::prelude::*;

pub fn process(input: &str) -> miette::Result<String> {
    let mut grid = GridArray::from_newline_separated_string(
        Topology::Bounded,
        Neighborhood::Orthogonal,
        input,
    );
    let (start_pos, start_direction) = get_start_pos_and_direction(&mut grid);

    let (has_loop, all_possible_positions) =
        get_all_possible_coors(&grid, None, &start_pos, start_direction);
    assert!(!has_loop);

    let unique = all_possible_positions
        .into_iter()
        .map(|(coor, _dir)| coor)
        .unique();
    let result = unique
        .collect_vec()
        .into_par_iter()
        .filter(|possible_coor| *possible_coor != start_pos)
        .filter(|possible_coor| {
            let (has_loop, _) =
                get_all_possible_coors(&grid, Some(possible_coor), &start_pos, start_direction);
            has_loop
        })
        .count();

    Ok(result.to_string())
}

fn get_start_pos_and_direction(grid: &mut GridArray<char>) -> (UCoor2D, Direction) {
    let (start_pos, direction_ch) = grid
        .all_cells()
        .find(|(_coor, ch)| ch != &&'.' && ch != &&'#')
        .unwrap();
    let start_direction = match direction_ch {
        '>' => Direction::East,
        '^' => Direction::North,
        '<' => Direction::West,
        _ => panic!("Invalid direction {direction_ch}"),
    };
    grid.set(start_pos.x, start_pos.y, '.');
    (start_pos, start_direction)
}

fn get_all_possible_coors(
    grid: &GridArray<char>,
    additional_obstacle_pos: Option<&UCoor2D>,
    start_pos: &UCoor2D,
    start_direction: Direction,
) -> (bool, HashSet<(UCoor2D, Direction)>) {
    let mut pos = start_pos.clone();
    let mut direction = start_direction;
    let mut all_positions = HashSet::new();
    all_positions.insert((pos.clone(), direction));
    loop {
        match grid
            .adjacent_cell(pos.x, pos.y, direction)
            .map(|next_pos| (grid.get_unchecked(next_pos.x, next_pos.y), next_pos))
        {
            Some(('.', next_pos)) if Some(&next_pos) != additional_obstacle_pos => {
                //println!("pos/dir: {pos}{direction:?} => next_pos: {next_pos}");
                let new_pos_dir_tuple = (next_pos.clone(), direction);
                if all_positions.contains(&new_pos_dir_tuple) {
                    return (true, all_positions);
                }
                all_positions.insert(new_pos_dir_tuple);
                pos = next_pos;
            }
            Some(('#', _)) => {
                direction = direction.rotate(90);
            }
            Some(('.', next_pos)) if Some(&next_pos) == additional_obstacle_pos => {
                direction = direction.rotate(90);
            }
            Some((ch, _)) => panic!("Invalid character {ch}"),
            None => return (false, all_positions),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_example() -> miette::Result<()> {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(6.to_string(), process(input)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> miette::Result<()> {
        let input = include_str!("../input2.txt");
        assert_eq!(1729.to_string(), process(input)?);
        Ok(())
    }
}

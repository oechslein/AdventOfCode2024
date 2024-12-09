use std::collections::HashMap;

use crate::custom_error::AocError;
use grid::{grid_array::GridArray, grid_iteration::adjacent_cell, grid_types::UCoor2D};

use grid::grid_types::Direction;

use grid::grid_types::{Neighborhood, Topology};

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut grid = GridArray::from_newline_separated_string(
        Topology::Bounded,
        Neighborhood::Orthogonal,
        input,
    );
    let (start_pos, start_direction) = get_start_pos_and_direction(&mut grid);
    let (is_loop, all_possible_coors_count) =
        get_all_possible_coors(&grid, &start_pos, start_direction);
    println!("is_loop: {is_loop}");

    Ok(all_possible_coors_count.to_string())
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
    start_pos: &UCoor2D,
    start_direction: Direction,
) -> (bool, usize) {
    let mut pos = start_pos.clone();
    let mut direction = start_direction;
    let mut all_positions = HashMap::new();
    all_positions.insert(pos.clone(), 1);
    loop {
        match adjacent_cell(
            grid.get_topology(),
            grid.width(),
            grid.height(),
            pos.clone(),
            direction,
        )
        .map(|next_pos| (grid.get_unchecked(next_pos.x, next_pos.y), next_pos))
        {
            Some(('.', next_pos)) => {
                println!("pos/dir: {pos}{direction:?} => next_pos: {next_pos}");
                let count = all_positions.entry(next_pos.clone()).or_insert(0);
                *count += 1;
                if *count >= 3 {
                    return (true, all_positions.len());
                }
                pos = next_pos;
            }
            Some(('#', _)) => {
                direction = direction.rotate(90);
            }
            Some((ch, _)) => panic!("Invalid character {ch}"),
            None => return (false, all_positions.len()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() -> miette::Result<()> {
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
        assert_eq!(41.to_string(), process(input)?);
        Ok(())
    }

    #[test]
    fn test_part1() -> miette::Result<()> {
        let input = include_str!("../input1.txt");
        assert_eq!(4977.to_string(), process(input)?);
        Ok(())
    }
}

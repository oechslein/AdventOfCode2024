use std::collections::HashSet;

use crate::custom_error::AocError;
use grid::{grid_array::GridArray, grid_types::UCoor2D};

use grid::grid_types::Direction;

use grid::grid_types::{Neighborhood, Topology};

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut grid = GridArray::from_newline_separated_string(
        Topology::Bounded,
        Neighborhood::Orthogonal,
        input,
    );
    let (start_pos, start_direction) = get_start_pos_and_direction(&mut grid);

    let mut result = 0;
    let (has_loop, all_possible_coors) =
        get_all_possible_coors(&grid, &start_pos, &start_direction);
    assert!(!has_loop);
    for possible_coor in all_possible_coors
        .into_iter()
        .filter(|coor| *coor != start_pos)
    {
        let mut curr_grid = grid.clone();
        assert_eq!(grid.get(possible_coor.x, possible_coor.y), Some(&'.'));
        curr_grid.set(possible_coor.x, possible_coor.y, '#');
        let (has_loop, _) = get_all_possible_coors(&curr_grid, &start_pos, &start_direction);
        if has_loop {
            //println!("Found loop for pos: {possible_coor}");
            result += 1;
        }
    }

    Ok(result.to_string())
}

fn get_start_pos_and_direction(grid: &mut GridArray<char>) -> (UCoor2D, Direction) {
    let (start_pos, direction_ch) = grid
        .all_cells()
        .filter(|(_coor, ch)| ch != &&'.' && ch != &&'#')
        .next()
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
    start_direction: &Direction,
) -> (bool, impl Iterator<Item = UCoor2D>) {
    let mut pos = start_pos.clone();
    let mut direction = start_direction.clone();
    let mut all_positions = HashSet::new();
    all_positions.insert((pos.clone(), direction.clone()));
    loop {
        match grid
            .adjacent_cell(pos.x, pos.y, direction)
            .map(|next_pos| (grid.get_unchecked(next_pos.x, next_pos.y), next_pos))
        {
            Some(('.', next_pos)) => {
                //println!("pos/dir: {pos}{direction:?} => next_pos: {next_pos}");
                let new_pos_dir_tuple = (next_pos.clone(), direction.clone());
                if all_positions.contains(&new_pos_dir_tuple) {
                    return (
                        true,
                        all_positions
                            .into_iter()
                            .map(|(coor, _dir)| coor)
                            .collect::<HashSet<UCoor2D>>()
                            .into_iter(),
                    );
                }
                all_positions.insert(new_pos_dir_tuple);
                pos = next_pos;
            }
            Some(('#', _)) => {
                direction = direction.rotate(90);
            }
            Some((ch, _)) => panic!("Invalid character {ch}"),
            None => {
                return (
                    false,
                    all_positions
                        .into_iter()
                        .map(|(coor, _dir)| coor)
                        .collect::<HashSet<UCoor2D>>()
                        .into_iter(),
                )
            }
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

use fxhash::FxHashMap;
use grid::{
    grid_array::GridArray,
    grid_types::{Direction, Neighborhood, Topology, UCoor2D},
};
use itertools::Itertools;
use num_traits::ToPrimitive;
use rayon::prelude::*;

use crate::custom_error::AocError;

//#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (map_str, movements_str) = input.split_once("\n\n").unwrap();
    let mut grid = GridArray::from_newline_separated_string(
        Topology::Bounded,
        Neighborhood::Orthogonal,
        map_str,
    );
    let mut robot_coor = grid
        .all_cells()
        .filter(|(_coor, ch)| **ch == '@')
        .map(|(coor, _h)| coor)
        .next()
        .unwrap();
    let movements_str = movements_str.replace('\n', "");
    let movements = movements_str.chars();

    for movement in movements {
        let movement_dir = match movement {
            '>' => Direction::East,
            '<' => Direction::West,
            '^' => Direction::North,
            'v' => Direction::South,
            _ => panic!(),
        };
        if check_if_movement_works(&mut grid, &robot_coor, movement_dir) {
            let new_coor = (robot_coor.to_icoor2d().unwrap() + movement_dir.diff_coor())
                .to_ucoor2d()
                .unwrap();
            robot_coor = new_coor;
        }
        //grid.println(false);
    }

    let result: usize = grid
        .all_cells()
        .filter(|(_coor, cell)| cell == &&'O')
        .map(|(coor, _cell)| coor.x + 100 * coor.y)
        .sum();

    Ok(result.to_string())
}

fn check_if_movement_works(
    grid: &mut GridArray<char>,
    coor: &UCoor2D,
    movement_dir: Direction,
) -> bool {
    let neighbor_coor = (coor.to_icoor2d().unwrap() + movement_dir.diff_coor())
        .to_ucoor2d()
        .unwrap();
    let neighbor = grid.get(neighbor_coor.x, neighbor_coor.y).unwrap_or(&'#');
    let is_possible = match neighbor {
        '.' => true,
        '#' => false,
        'O' => {
            if !check_if_movement_works(grid, &neighbor_coor, movement_dir) {
                return false;
            }
            true
        }
        _ => panic!(),
    };

    if !is_possible {
        return false;
    }
    // if check is true also perform change (assuming change was done to neighbor)
    debug_assert_eq!(
        grid.get(neighbor_coor.x, neighbor_coor.y).unwrap_or(&'#'),
        &'.'
    );

    grid.set(
        neighbor_coor.x,
        neighbor_coor.y,
        *grid.get(coor.x, coor.y).unwrap(),
    );
    grid.set(coor.x, coor.y, '.');

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> miette::Result<()> {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        assert_eq!("10092", process(&input.replace('\r', ""))?);
        Ok(())
    }

    #[test]
    fn test_input() -> miette::Result<()> {
        let input = include_str!("../input1.txt");
        assert_eq!("", process(&input.replace('\r', ""))?);
        Ok(())
    }
}

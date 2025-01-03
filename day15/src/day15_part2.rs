use std::{collections::HashMap, fs::File, sync::LazyLock};

use fxhash::{FxHashMap, FxHashSet};
use grid::{
    grid_array::GridArray,
    grid_types::{Direction, Neighborhood, Topology, UCoor2D},
};
use itertools::Itertools;
use num_traits::ToPrimitive;
use rayon::prelude::*;

use gif::{Encoder, Frame, Repeat};

fn count_chars(grid: &GridArray<char>, ch: char) -> usize {
    grid.all_cells().filter(|(_coor, &cell)| cell == ch).count()
}

//#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (mut grid, movements) = parse(input);
    let mut robot_coor = get_robot_pos(&grid);
    let mut moves = FxHashMap::default();

    let (mut _encoder, mut _color_map): (Encoder<File>, HashMap<char, u8>);
    #[cfg(not(test))]
    {
        (_encoder, _color_map) = create_image_encoder(&grid);
        grid.write_grid_as_frame(&mut _encoder, &_color_map);
    }
    let left_bracket_count = count_chars(&grid, '[');
    let right_bracket_count = count_chars(&grid, ']');
    let wall_count = count_chars(&grid, '#');
    let dot_count = count_chars(&grid, '.');

    if cfg!(debug_assertions) {
        grid.println(false);
    }
    for movement in movements {
        if cfg!(debug_assertions) {
            println!("{movement:?} {}", robot_coor);
            grid.println(false);
            debug_assert_eq!(left_bracket_count, count_chars(&grid, '['));
            debug_assert_eq!(right_bracket_count, count_chars(&grid, ']'));
            debug_assert_eq!(wall_count, count_chars(&grid, '#'));
            debug_assert_eq!(dot_count, count_chars(&grid, '.'));
        }
        if check_if_robot_movement_works(&mut grid, &robot_coor, movement, &mut moves) {
            robot_coor = add_direction(&robot_coor, movement);
            #[cfg(not(test))]
            {
                grid.write_grid_as_frame(&mut _encoder, &_color_map);
            }
        }
    }
    debug_assert_eq!(left_bracket_count, count_chars(&grid, '['));
    debug_assert_eq!(right_bracket_count, count_chars(&grid, ']'));

    if cfg!(debug_assertions) {
        grid.println(false);
    }

    let result: usize = grid
        .all_cells()
        .filter(|(_coor, cell)| cell == &&'[')
        .map(|(coor, _cell)| coor.x + 100 * coor.y)
        .sum();

    Ok(result.to_string())
}

fn create_image_encoder(grid: &GridArray<char>) -> (Encoder<File>, HashMap<char, u8>) {
    let colors = vec![
        vec![0, 0, 0],
        vec![255, 255, 255],
        vec![160, 160, 160],
        vec![255, 217, 50],
        vec![255 - 15, 217 - 15, 50 - 15],
    ];
    let color_map: HashMap<char, u8> =
        HashMap::from([('#', 2), ('[', 3), (']', 4), ('@', 1), ('.', 0)]);
    let encoder = grid.create_image_encoder(r"C:\temp\part15.gif", colors);
    (encoder, color_map)
}

fn get_robot_pos(grid: &GridArray<char>) -> UCoor2D {
    grid.all_cells()
        .filter(|(_coor, ch)| **ch == '@')
        .map(|(coor, _h)| coor)
        .next()
        .unwrap()
}

fn parse(input: &str) -> (GridArray<char>, Vec<Direction>) {
    fn double_grid(map_str: &str) -> String {
        let map_str = map_str
            .chars()
            .map(|ch| match ch {
                '#' => "##",
                'O' => "[]",
                '.' => "..",
                '@' => "@.",
                '\n' => "\n",
                _ => panic!("Unsupported char '{ch}'"),
            })
            .collect::<String>();
        map_str
    }

    let (map_str, movements_str) = input.split_once("\n\n").unwrap();
    let map_str = double_grid(map_str);
    let grid = GridArray::from_newline_separated_string(
        Topology::Bounded,
        Neighborhood::Orthogonal,
        &map_str,
    );
    let movements_str = movements_str.replace('\n', "");
    let movements = movements_str.chars().map(|movement| match movement {
        '>' => Direction::East,
        '<' => Direction::West,
        '^' => Direction::North,
        'v' => Direction::South,
        _ => panic!("Unsupported movement char '{movement}'"),
    });
    (grid, movements.collect_vec())
}

fn check_if_robot_movement_works(
    grid: &mut GridArray<char>,
    coor: &UCoor2D,
    movement_dir: Direction,
    moves: &mut FxHashMap<UCoor2D, char>,
) -> bool {
    let (ref neighbor_coor, neighbor) = look_in_dir(grid, coor, movement_dir);

    match neighbor {
        '.' => {}
        '#' => {
            return false;
        }
        _ => {
            if !check_if_box_movement_works(grid, neighbor_coor, neighbor, movement_dir, moves) {
                return false;
            }
            execute_moves(grid, moves);
        }
    };

    // if check is true also perform change (assuming change was done to neighbor)
    debug_assert_eq!(get_grid(grid, neighbor_coor), '.');

    set_grid(grid, neighbor_coor, get_grid(grid, coor));
    set_grid(grid, coor, '.');

    true
}

fn execute_moves(grid: &mut GridArray<char>, moves: &mut FxHashMap<UCoor2D, char>) {
    for (coor, cell) in moves.drain() {
        set_grid(grid, &coor, cell);
    }
}

fn check_if_box_movement_works(
    grid: &GridArray<char>,
    box_coor: &UCoor2D,
    box_cell: char,
    movement_dir: Direction,
    moves: &mut FxHashMap<UCoor2D, char>,
) -> bool {
    debug_assert!(box_cell == '[' || box_cell == ']');

    // easy if movement_dir is west or east
    if movement_dir == Direction::West || movement_dir == Direction::East {
        check_if_box_movement_works_horizontal(grid, box_coor, box_cell, movement_dir, moves)
    } else {
        debug_assert!(movement_dir == Direction::North || movement_dir == Direction::South);
        check_if_box_movement_works_vertical(grid, box_coor, box_cell, movement_dir, moves)
    }
}

fn check_if_box_movement_works_vertical(
    grid: &GridArray<char>,
    box_coor: &UCoor2D,
    box_cell: char,
    movement_dir: Direction,
    moves: &mut FxHashMap<UCoor2D, char>,
) -> bool {
    debug_assert!(movement_dir == Direction::North || movement_dir == Direction::South);
    let (ref other_box_coor, other_box) = look_in_dir(
        grid,
        box_coor,
        if box_cell == '[' {
            Direction::East
        } else {
            Direction::West
        },
    );

    let (ref new_box_coor, new_box_cell) = look_in_dir(grid, box_coor, movement_dir);
    let (ref new_other_box_coor, new_other_box_cell) =
        look_in_dir(grid, other_box_coor, movement_dir);

    match (new_box_cell, new_other_box_cell) {
        ('.', '.') => {}

        ('#', _) => {
            return false;
        }
        (_, '#') => {
            return false;
        }

        ('[', ']') | (']', '[') => {
            if !check_if_box_movement_works(grid, new_box_coor, new_box_cell, movement_dir, moves)
                || !check_if_box_movement_works(
                    grid,
                    new_other_box_coor,
                    new_other_box_cell,
                    movement_dir,
                    moves,
                )
            {
                return false;
            }
        }
        ('.', '[') | ('.', ']') => {
            if !check_if_box_movement_works(
                grid,
                new_other_box_coor,
                new_other_box_cell,
                movement_dir,
                moves,
            ) {
                return false;
            }
        }

        (']', '.') | ('[', '.') => {
            if !check_if_box_movement_works(grid, new_box_coor, new_box_cell, movement_dir, moves) {
                return false;
            }
        }

        _ => panic!(),
    };

    add_move(moves, other_box_coor, '.');
    add_move(moves, box_coor, '.');
    add_move(moves, new_other_box_coor, other_box);
    add_move(moves, new_box_coor, box_cell);

    true
}

fn check_if_box_movement_works_horizontal(
    grid: &GridArray<char>,
    box_coor: &UCoor2D,
    box_cell: char,
    movement_dir: Direction,
    moves: &mut FxHashMap<UCoor2D, char>,
) -> bool {
    debug_assert!(movement_dir == Direction::West || movement_dir == Direction::East);
    let (ref other_box_coor, other_box) = look_in_dir(grid, box_coor, movement_dir);

    debug_assert!(
        if box_cell == '[' {
            other_box == ']'
        } else {
            other_box == '['
        },
        "{box_cell}{other_box}"
    );

    let (ref new_other_box_coor, neighbor) = look_in_dir(grid, other_box_coor, movement_dir);

    match neighbor {
        '.' => {}
        '#' => {
            return false;
        }
        '[' | ']' => {
            if !check_if_box_movement_works(grid, new_other_box_coor, neighbor, movement_dir, moves)
            {
                return false;
            }
        }
        _ => panic!(),
    };

    add_move(moves, box_coor, '.');
    add_move(moves, other_box_coor, box_cell);
    add_move(moves, new_other_box_coor, other_box);

    true
}

#[inline]
fn add_move(moves: &mut FxHashMap<UCoor2D, char>, coor: &UCoor2D, cell: char) {
    if cell != '.' || !moves.contains_key(coor) {
        moves.insert(coor.clone(), cell);
    }
}

#[inline]
fn get_grid(grid: &GridArray<char>, coor: &UCoor2D) -> char {
    *grid.get(coor.x, coor.y).unwrap_or(&'#')
}

#[inline]
fn set_grid(grid: &mut GridArray<char>, box_coor: &UCoor2D, cell: char) {
    grid.set(box_coor.x, box_coor.y, cell);
}

#[inline]
fn look_in_dir(grid: &GridArray<char>, coor: &UCoor2D, dir: Direction) -> (UCoor2D, char) {
    let other_box_coor = add_direction(coor, dir);
    let other_box = get_grid(grid, &other_box_coor);
    (other_box_coor, other_box)
}

fn add_direction(coor: &UCoor2D, dir: Direction) -> UCoor2D {
    (coor.to_icoor2d().unwrap() + dir.diff_coor())
        .to_ucoor2d()
        .unwrap()
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
        assert_eq!("9021", process(&input.replace('\r', ""))?);
        Ok(())
    }

    #[test]
    fn test_input() -> miette::Result<()> {
        let input = include_str!("../input2.txt");
        assert_eq!("1544522", process(&input.replace('\r', ""))?); // 1535509
        Ok(())
    }
}

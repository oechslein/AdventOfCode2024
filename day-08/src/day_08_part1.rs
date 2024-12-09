use grid::{
    grid_array::GridArray,
    grid_types::{Neighborhood, Topology},
};
use itertools::Itertools;

use crate::custom_error::AocError;

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let grid =
        GridArray::from_newline_separated_string(Topology::Bounded, Neighborhood::Square, input);

    let antenna_types = grid
        .all_cells()
        .map(|(_coor, ch)| ch)
        .filter(|ch| **ch != '.');

    let antennas = antenna_types.map(|antenna_type| {
        grid.all_cells()
            .filter(|(_coor, ch)| *ch == antenna_type)
            .map(|(coor, _ch)| {
                let (x, y) = coor.to_tuple();
                (x as isize, y as isize)
            })
            .collect_vec()
    });

    let antinodes = antennas
        .flat_map(|antenna_coors| {
            Itertools::permutations(antenna_coors.iter(), 2)
                .filter_map(|pair| {
                    let (point0_x, point0_y) = (pair[0].0, pair[0].1);
                    let (point1_x, point1_y) = (pair[1].0, pair[1].1);
                    let (new_coor_x, new_coor_y) =
                        (2 * point0_x - point1_x, 2 * point0_y - point1_y);
                    let on_grid = (0 <= new_coor_x && (new_coor_x as usize) < grid.width())
                        && (new_coor_y >= 0 && (new_coor_y as usize) < grid.height());
                    on_grid.then(|| (new_coor_x, new_coor_y))
                })
                .collect_vec()
        })
        .unique()
        .count();

    Ok(antinodes.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() -> miette::Result<()> {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(14.to_string(), process(input)?);
        Ok(())
    }

    #[test]
    fn test_part1() -> miette::Result<()> {
        let input = include_str!("../input1.txt");
        assert_eq!(220.to_string(), process(input)?);
        Ok(())
    }
}

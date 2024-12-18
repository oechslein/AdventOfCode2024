use std::collections::HashSet;

use grid::{
    grid_array::GridArray,
    grid_types::{ICoor2D, Neighborhood, Topology},
};
use itertools::Itertools;



#[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
pub fn process(input: &str) -> miette::Result<String> {
    let grid =
        GridArray::from_newline_separated_string(Topology::Bounded, Neighborhood::Square, input);

    let antenna_types = grid
        .all_cells()
        .map(|(_coor, ch)| ch)
        .filter(|ch| **ch != '.');

    let antennas = antenna_types.map(|antenna_type| {
        grid.all_cells()
            .filter(|(_coor, ch)| *ch == antenna_type)
            .map(|(coor, _ch)| ICoor2D {
                x: coor.x as isize,
                y: coor.y as isize,
            })
            .collect_vec()
    });

    let mut antinodes = HashSet::new();
    for antenna_coors in antennas {
        for pair in Itertools::permutations(antenna_coors.iter(), 2) {
            let point0 = pair[0];
            let point1 = pair[1];
            let diff_coor = point0 - point1;
            let mut new_coor = point0.clone();
            while (0 <= new_coor.x && (new_coor.x as usize) < grid.width())
                && (0 <= new_coor.y && (new_coor.y as usize) < grid.height())
            {
                antinodes.insert(new_coor.clone());
                new_coor = &new_coor + &diff_coor;
            }
        }
    }

    Ok(antinodes.len().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_example() -> miette::Result<()> {
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
        assert_eq!(34.to_string(), process(input)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> miette::Result<()> {
        let input = include_str!("../input2.txt");
        assert_eq!(813.to_string(), process(input)?);
        Ok(())
    }
}

use fxhash::FxHashMap;
use grid::{
    grid_array::{GridArray, GridArrayBuilder},
    grid_hashmap::{GridHashMap, GridHashMapBuilder},
    grid_types::{ICoor2D, Neighborhood, Topology, UCoor2D},
};
use itertools::Itertools;
use num_traits::ToPrimitive;
use rayon::prelude::*;

//#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    process2(input, 101, 103)
    //process2(input, 11, 7)
}

struct Robot {
    pos: UCoor2D,
    vel: ICoor2D,
}

pub fn process2(input: &str, width: usize, height: usize) -> miette::Result<String> {
    let mut robots = input
        .lines()
        .map(|line| {
            let (position_str, velocity_str) = line.split_once(' ').unwrap();
            let (pos_x_str, pos_y_str) = position_str["p=".len()..].split_once(',').unwrap();
            let (pos_x, pos_y) = (pos_x_str.parse().unwrap(), pos_y_str.parse().unwrap());
            let (vel_x_str, vel_y_str) = velocity_str["v=".len()..].split_once(',').unwrap();
            let (vel_x, vel_y) = (vel_x_str.parse().unwrap(), vel_y_str.parse().unwrap());

            Robot {
                pos: UCoor2D { x: pos_x, y: pos_y },
                vel: ICoor2D { x: vel_x, y: vel_y },
            }
        })
        .collect_vec();

    println!("robots: {}", robots.len());
    //print_robots(&robots, width, height);
    for _iteration in 0..100 {
        for robot in &mut robots {
            let mut new_pos = &robot.pos.to_icoor2d().unwrap() + &robot.vel;
            while new_pos.x >= width as isize {
                new_pos.x -= width as isize;
            }
            while new_pos.y >= height as isize {
                new_pos.y -= height as isize;
            }
            while new_pos.x < 0 {
                new_pos.x += width as isize;
            }
            while new_pos.y < 0 {
                new_pos.y += height as isize;
            }
            debug_assert!(0 <= new_pos.x && new_pos.x < width as isize);
            debug_assert!(0 <= new_pos.y && new_pos.y < height as isize);
            robot.pos = new_pos.to_ucoor2d().unwrap();
        }
        //print_robots(&robots, width, height);
    }

    let filter_x = width / 2;
    let filter_y = height / 2;

    let result: usize = robots
        .into_iter()
        .filter(|robot| robot.pos.x != filter_x && robot.pos.y != filter_y)
        .sorted_by_key(|robot| quadrant(&robot.pos, width, height))
        .chunk_by(|robot| quadrant(&robot.pos, width, height))
        .into_iter()
        .map(|(_quadrant, chunk)| chunk.count())
        .product();

    Ok(result.to_string())
}

fn print_robots(robots: &[Robot], width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            let count = robots
                .iter()
                .filter(|robot| robot.pos.x == x && robot.pos.y == y)
                .count();
            if count == 0 {
                print!(".");
            } else {
                print!("{count}");
            }
        }
        println!();
    }
    println!();
}

fn quadrant(pos: &UCoor2D, width: usize, height: usize) -> usize {
    match (pos.x > width / 2, pos.y > height / 2) {
        (false, false) => 0,
        (true, false) => 1,
        (false, true) => 2,
        (true, true) => 3,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() -> miette::Result<()> {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!("12", process2(&input.replace('\r', ""), 11, 7)?);
        Ok(())
    }

    #[test]
    fn test_part1() -> miette::Result<()> {
        let input = include_str!("../input1.txt");
        assert_eq!("219150360", process(&input.replace('\r', ""))?);
        Ok(())
    }
}

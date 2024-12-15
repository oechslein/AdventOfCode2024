use std::str::FromStr;

use fxhash::FxHashMap;
use grid::{
    grid_array::{GridArray, GridArrayBuilder},
    grid_hashmap::{GridHashMap, GridHashMapBuilder},
    grid_types::{ICoor2D, Neighborhood, Topology, UCoor2D},
};
use itertools::Itertools;
use miette::Error;
use num_traits::ToPrimitive;
use rayon::prelude::*;

use crate::custom_error::AocError;

//#[tracing::instrument]
#[derive(Clone, Debug, PartialEq)]
struct Robot {
    pos: UCoor2D,
    vel: ICoor2D,
}

impl Robot {
    fn move_robot(&mut self, width: usize, height: usize) {
        let mut new_pos = &self.pos.to_icoor2d().unwrap() + &self.vel;
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

        self.pos = new_pos.to_ucoor2d().unwrap();
    }
}

impl FromStr for Robot {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (position_str, velocity_str) = line.split_once(' ').unwrap();
        let (pos_x_str, pos_y_str) = position_str["p=".len()..].split_once(',').unwrap();
        let (pos_x, pos_y) = (pos_x_str.parse().unwrap(), pos_y_str.parse().unwrap());
        let (vel_x_str, vel_y_str) = velocity_str["v=".len()..].split_once(',').unwrap();
        let (vel_x, vel_y) = (vel_x_str.parse().unwrap(), vel_y_str.parse().unwrap());

        Ok(Robot {
            pos: UCoor2D { x: pos_x, y: pos_y },
            vel: ICoor2D { x: vel_x, y: vel_y },
        })
    }
}

fn get_index_pos(pos: &UCoor2D, height: usize) -> usize {
    get_index(pos.x, pos.y, height)
}

fn get_index(x: usize, y: usize, height: usize) -> usize {
    x * height + y
}

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (width, height) = (101, 103);
    let mut position_count_vec = vec![0; width * height];
    let mut robots = input
        .lines()
        .map(|line| {
            let robot = Robot::from_str(line).unwrap();
            position_count_vec[get_index_pos(&robot.pos, height)] += 1;
            robot
        })
        .collect_vec();

    let mut iteration = 0;
    while !has_christmas_tree(&position_count_vec, width, height) {
        for robot in &mut robots {
            position_count_vec[get_index_pos(&robot.pos, height)] -= 1;
            robot.move_robot(width, height);
            position_count_vec[get_index_pos(&robot.pos, height)] += 1;
        }

        iteration += 1;
    }

    //iteration = 8053; // with 1111111111111111111111111111111
    //println!("iteration (has_christmas_tree2): {iteration}");
    //print_robots(&robots, width, height);
    Ok(iteration.to_string())
}

fn has_christmas_tree(position_count_vec: &[usize], width: usize, height: usize) -> bool {
    (0..height).par_bridge().any(|y| {
        let mut start_x = 0;
        for x in 0..width {
            let count = position_count_vec[get_index(x, y, height)];
            if count == 0 {
                start_x = x;
                continue;
            }

            if x - start_x >= 10 {
                return true;
            }
        }
        false
    })
}

fn print_robots(position_count_vec: &[usize], width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            let count = position_count_vec[get_index(x, y, height)];
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() -> miette::Result<()> {
        let input = include_str!("../input2.txt");
        assert_eq!("8053", process(&input.replace('\r', ""))?);
        Ok(())
    }
}

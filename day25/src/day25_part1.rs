use fxhash::FxHashMap;
use itertools::Itertools;
use num_traits::ToPrimitive;
use rayon::prelude::*;

use miette::{miette, Error, Result};

fn transpose(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    if matrix.is_empty() {
        return vec![];
    }
    let row_count = matrix.len();
    let col_count = matrix[0].len();
    (0..col_count)
        .map(|i| (0..row_count).map(|j| matrix[j][i]).collect())
        .collect()
}

//#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    input.split("\n\n").for_each(|block| {
        let mut lines = block.lines();
        let is_lock = match lines.next().unwrap() {
            "#####" => true,
            "....." => false,
            _ => panic!("Invalid block {}", block),
        };
        let info = transpose(
            block
                .lines()
                .map(|line| line.chars().collect_vec())
                .collect_vec(),
        )
        .into_iter()
        .map(|row| row.into_iter().filter(|ch| ch == &'#').count() - 1)
        .collect_vec();
        if is_lock {
            locks.push(info);
        } else {
            keys.push(info);
        }
    });

    println!(
        "{}*{}={}",
        locks.len(),
        keys.len(),
        locks.len() * keys.len()
    );

    let result: usize = locks
        .into_par_iter()
        .map(|lock| keys.iter().filter(|key| fits(&lock, key)).count())
        .sum();

    Ok(result.to_string())
}

fn fits(lock: &[usize], key: &[usize]) -> bool {
    lock.iter().zip(key.iter()).all(|(l, k)| l + k < 6)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> miette::Result<()> {
        let input = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
        assert_eq!("3", process(&input.replace('\r', ""))?);
        Ok(())
    }

    #[test]
    fn test_input() -> miette::Result<()> {
        let input = include_str!("../input1.txt");
        assert_eq!("3249", process(&input.replace('\r', ""))?);
        Ok(())
    }
}

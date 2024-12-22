use fxhash::FxHashMap;
use itertools::Itertools;
use num_traits::ToPrimitive;
use rayon::prelude::*;

use miette::{miette, Error, Result};

type NumberType = usize;

//#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let result: usize = input
        .lines()
        .par_bridge()
        .map(|line| {
            let secret = line.parse::<NumberType>().unwrap();
            (0..2000).fold(secret, |acc, _i| all_steps(acc))
        })
        .sum();
    Ok(result.to_string())
}

fn all_steps(secret: usize) -> usize {
    step_three(step_two(step_one(secret)))
}

fn step_one(secret: NumberType) -> NumberType {
    prune(mix(secret, secret << 6))
}

fn prune(secret: NumberType) -> NumberType {
    secret & ((1 << 24) - 1)
}

fn mix(secret: NumberType, new_secret: NumberType) -> NumberType {
    secret ^ new_secret
}

fn step_two(secret: NumberType) -> NumberType {
    prune(mix(secret, secret >> 5))
}

fn step_three(secret: NumberType) -> NumberType {
    prune(mix(secret, secret << 11))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> miette::Result<()> {
        let input = "1
10
100
2024";
        assert_eq!("37327623", process(&input.replace('\r', ""))?);
        Ok(())
    }

    #[test]
    fn test_input() -> miette::Result<()> {
        let input = include_str!("../input1.txt");
        assert_eq!("21147129593", process(&input.replace('\r', ""))?);
        Ok(())
    }
}

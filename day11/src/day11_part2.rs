use fxhash::FxHashMap;
use itertools::Itertools;
use num_traits::ToPrimitive;
use rayon::prelude::*;

use crate::custom_error::AocError;

type NumType = u64;
type CycleType = u8;

//#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let input = input
        .split(' ')
        .map(|num_str| num_str.parse::<NumType>().unwrap());
    let cycles = 75;
    let mut cache: FxHashMap<(NumType, CycleType), NumType> = FxHashMap::default();
    let result: NumType = input
        .map(|num| evolve_cached(num, cycles, &mut cache))
        .sum();
    Ok(result.to_string())
}

fn evolve_cached(
    num: NumType,
    cycles_left: CycleType,
    cache: &mut FxHashMap<(NumType, CycleType), NumType>,
) -> NumType {
    let key = (num, cycles_left);
    if let Some(result) = cache.get(&key) {
        return *result;
    }

    let result = evolve(num, cycles_left, cache);
    cache.insert(key, result);
    result
}

fn evolve(
    num: NumType,
    cycles_left: CycleType,
    cache: &mut FxHashMap<(NumType, CycleType), NumType>,
) -> NumType {
    if cycles_left == 0 {
        return 1;
    }

    if num == 0 {
        return evolve_cached(1, cycles_left - 1, cache);
    }

    let mut digit_count = 0;
    let mut temp = num;
    while temp > 0 {
        temp /= 10;
        digit_count += 1;
    }

    if digit_count % 2 != 0 {
        return evolve_cached(num * 2024, cycles_left - 1, cache);
    }

    let half_digits = digit_count / 2;
    const RADIX: NumType = 10;
    let divisor = RADIX.pow(half_digits);

    let second_half = num % divisor;
    let first_half = num / divisor;

    evolve_cached(first_half, cycles_left - 1, cache)
        + evolve_cached(second_half, cycles_left - 1, cache)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_example() -> miette::Result<()> {
        let input = "125 17";
        assert_eq!("65601038650482", process(input)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> miette::Result<()> {
        let input = include_str!("../input2.txt");
        assert_eq!("238317474993392", process(input)?);
        Ok(())
    }
}

use std::num::{NonZeroU32, NonZeroU64};

use crate::custom_error::AocError;
use dashmap::DashMap;
use fxhash::FxHashMap;
use itertools::Itertools;
use num_traits::ToPrimitive;
use rayon::prelude::*;
use smallvec::{smallvec, SmallVec};

type NumType = u64;
type CycleType = u8;
//type HashMapType = FxHashMap<(NumType, CycleType), NumType>;
type HashMapType = DashMap<(NumType, CycleType), NumType>;

//#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let input = input
        .split(' ')
        .map(|num_str| num_str.parse::<NumType>().unwrap());
    let cycles = 75;
    let cache: HashMapType = HashMapType::default();
    let result: NumType = input
        .par_bridge()
        .map(|num| evolve(num, cycles, &cache))
        .sum();
    Ok(result.to_string())
}
fn evolve(num: NumType, cycles_left: CycleType, cache: &HashMapType) -> NumType {
    let mut acc = 0;
    evolve_tail_recursive(num, cycles_left, cache, &mut acc);
    acc
}

fn evolve_tail_recursive(
    num: NumType,
    cycles_left: CycleType,
    cache: &HashMapType,
    acc: &mut NumType,
) {
    if cycles_left == 0 {
        *acc += 1;
        return;
    }

    if num == 0 {
        if cycles_left - 1 == 0 {
            *acc += 1;
            return;
        }
        evolve_tail_recursive(2024, cycles_left - 2, cache, acc);
        return;
    }

    let digit_count = digit_count(num);

    if digit_count % 2 != 0 {
        evolve_tail_recursive(num * 2024, cycles_left - 1, cache, acc);
        return;
    }

    let key = (num, cycles_left);
    if let Some(result) = cache.get(&key) {
        *acc += *result;
        return;
    }

    let result = split_in_two(digit_count, num)
        .into_par_iter()
        .map(|num| {
            let mut tmp_acc = 0;
            evolve_tail_recursive(*num, cycles_left - 1, cache, &mut tmp_acc);
            tmp_acc
        })
        .sum();
    //let result = evolve_tail_recursive(first_half, cycles_left - 1, cache) + evolve(second_half, cycles_left - 1, cache)

    cache.insert(key, result);

    *acc += result;
}

fn split_in_two(digit_count: u32, num: u64) -> SmallVec<[u64; 2]> {
    let half_digits = digit_count / 2;
    const RADIX: NumType = 10;
    let divisor = RADIX.pow(half_digits);

    let second_half = num % divisor;
    let first_half = num / divisor;
    smallvec![first_half, second_half]
}

fn split_in_two_slow(_digit_count: u32, num: u64) -> (u64, u64) {
    let num_str = num.to_string();
    let mid = num_str.len() / 2;
    let (first_half, second_half) = num_str.split_at(mid);
    let first_half = first_half.parse::<NumType>().unwrap();
    let second_half = second_half.parse::<NumType>().unwrap();
    (second_half, first_half)
}

fn digit_count(num: NumType) -> u32 {
    debug_assert_ne!(num, 0);
    num.ilog10() + 1
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

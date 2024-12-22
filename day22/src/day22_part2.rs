use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use num_traits::ToPrimitive;
use rayon::prelude::*;

use miette::{miette, Error, Result};

type NumberType = usize;
type PriceType = u8;
type ChangesType = i8;
type SequenceType = (ChangesType, ChangesType, ChangesType, ChangesType);

//#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let all_possible_changes_map_list: Vec<_> = input
        .lines()
        .par_bridge()
        .map(|line| line.parse::<NumberType>().unwrap())
        .map(get_all_possible_changes_map)
        .collect();

    let all_possible_changes: FxHashSet<SequenceType> = all_possible_changes_map_list
        .iter()
        .flat_map(|changes_map| changes_map.keys())
        .cloned()
        .collect();

    let result = all_possible_changes
        .into_par_iter()
        .map(|possible_change| {
            all_possible_changes_map_list
                .iter()
                .filter_map(|map| map.get(&possible_change))
                .map(|price| *price as usize)
                .sum::<usize>()
        })
        .max()
        .unwrap();

    Ok(result.to_string())
}

fn get_all_possible_changes_map(secret: usize) -> FxHashMap<SequenceType, PriceType> {
    let mut all_possible_changes: FxHashMap<SequenceType, PriceType> = FxHashMap::default();
    for (changes, price) in all_secrets_tuple_changes_and_price(secret) {
        all_possible_changes.entry(changes).or_insert(price);
    }
    all_possible_changes
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

fn all_secrets(secret: usize) -> impl Iterator<Item = usize> {
    (0..2000).scan(secret, |acc, _i| {
        let old_secret = *acc;
        let new_secret = all_steps(old_secret);
        *acc = new_secret;
        Some(old_secret)
    })
}

fn all_secrets_changes_and_prices(secret: usize) -> impl Iterator<Item = (ChangesType, PriceType)> {
    all_secrets(secret)
        .map(|s| (s % 10) as PriceType)
        .tuple_windows()
        .map(|(price_a, price_b)| {
            let change: ChangesType = (price_b as isize - price_a as isize) as ChangesType;
            (change, price_b)
        })
}

fn calc_price(secret: NumberType, sequence: SequenceType) -> PriceType {
    all_secrets_tuple_changes_and_price(secret)
        .filter_map(|((change_a, change_b, change_c, change_d), price_d)| {
            if (change_a, change_b, change_c, change_d) == sequence {
                Some(price_d)
            } else {
                None
            }
        })
        .next()
        .unwrap_or(0)
}

fn all_secrets_tuple_changes_and_price(
    secret: usize,
) -> impl Iterator<Item = (SequenceType, PriceType)> {
    all_secrets_changes_and_prices(secret).tuple_windows().map(
        |((change_a, _), (change_b, _), (change_c, _), (change_d, price_d))| {
            ((change_a, change_b, change_c, change_d), price_d)
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() -> miette::Result<()> {
        assert_eq!(6, calc_price(123, (-1, -1, 0, 2)));

        let sequence = (-2, 1, -1, 3);
        assert_eq!(7, calc_price(1, sequence));
        assert_eq!(7, calc_price(2, sequence));
        assert_eq!(0, calc_price(3, sequence));
        assert_eq!(9, calc_price(2024, sequence));
        Ok(())
    }

    #[test]
    fn test_example() -> miette::Result<()> {
        let input = "1
2
3
2024";
        assert_eq!("23", process(&input.replace('\r', ""))?);
        Ok(())
    }

    #[test]
    fn test_input() -> miette::Result<()> {
        let input = include_str!("../input2.txt");
        assert_eq!("2445", process(&input.replace('\r', ""))?);
        Ok(())
    }
}

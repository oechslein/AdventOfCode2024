use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use rayon::prelude::*;

use miette::Result;

type NumberType = i32;
type PriceType = u8;
type ChangesType = i8;
type SequenceType = (ChangesType, ChangesType, ChangesType, ChangesType);
const SEQUENCE_LENGTH: usize = 4;

//#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let secret_count = 2000;
    let input = input
        .lines()
        .map(|line| line.parse::<NumberType>().unwrap());

    let all_possible_changes_map_list: Vec<_> = input
        .par_bridge()
        .map(|secret| get_all_possible_changes_map(secret, secret_count))
        .collect();

    let all_possible_changes: FxHashSet<SequenceType> = all_possible_changes_map_list
        .iter()
        .flat_map(|changes_map| changes_map.keys())
        .copied()
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

fn next_secret(mut secret: NumberType) -> NumberType {
    const MASK: NumberType = (1 << 24) - 1;
    secret = (secret ^ (secret << 6)) & MASK;
    secret = (secret ^ (secret >> 5)) & MASK;
    secret = (secret ^ (secret << 11)) & MASK;
    secret
}

#[allow(clippy::cast_sign_loss, clippy::cast_lossless)]
fn get_all_possible_changes_map(
    secret: NumberType,
    secret_count: usize,
) -> FxHashMap<SequenceType, PriceType> {
    let prices = gen_secrets(secret, secret_count)
        .map(|secret| (secret % 10) as PriceType)
        .collect_vec();

    (0..prices.len() - SEQUENCE_LENGTH)
        .map(|index| {
            let changes = prices[index..=index + SEQUENCE_LENGTH]
                .iter()
                .tuple_windows()
                .map(|(&a, &b)| ((b as NumberType) - (a as NumberType)) as ChangesType)
                .collect_tuple()
                .unwrap();

            let price = prices[index + SEQUENCE_LENGTH];
            (changes, price)
        })
        .rev() // make sure that the first change is in the hashmap
        .collect()
}

fn gen_secrets(secret: NumberType, secret_count: usize) -> impl Iterator<Item = NumberType> {
    (0..secret_count).scan(secret, |acc, _i| {
        let old_secret = *acc;
        let new_secret = next_secret(old_secret);
        *acc = new_secret;
        Some(old_secret)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

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

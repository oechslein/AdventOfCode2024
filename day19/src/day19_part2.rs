use std::collections::hash_map::Entry;

use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use num_traits::ToPrimitive;
use rayon::prelude::*;

use miette::{miette, Error, Result};

//#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let (patterns, towels) = input.split_once("\n\n").ok_or(miette!("Invalid input"))?;
    let patterns = patterns.split(", ").collect_vec();
    let mut matching_patterns_map: FxHashMap<String, usize> = FxHashMap::default();
    matching_patterns_map.insert("".to_string(), 1);
    let result = towels
        .lines()
        //.par_bridge()
        .map(|towel| {
            count_matching_pattern_combinations(towel, &patterns, &mut matching_patterns_map)
        })
        .sum::<usize>();
    Ok(result.to_string())
}

fn count_matching_pattern_combinations(
    towel: &str,
    patterns: &Vec<&str>,
    matching_patterns_map: &mut FxHashMap<String, usize>,
) -> usize {
    if let Some(matched_patterns) = matching_patterns_map.get(towel) {
        return *matched_patterns;
    }

    let matched_patterns = patterns
        .iter()
        .filter(|&pattern| towel.starts_with(pattern))
        .map(|pattern| {
            count_matching_pattern_combinations(
                &towel[pattern.len()..],
                patterns,
                matching_patterns_map,
            )
        })
        .sum::<usize>();
    matching_patterns_map.insert(towel.to_string(), matched_patterns);

    matched_patterns
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> miette::Result<()> {
        let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        assert_eq!("16", process(&input.replace('\r', ""))?);
        Ok(())
    }

    #[test]
    fn test_input() -> miette::Result<()> {
        let input = include_str!("../input2.txt");
        assert_eq!("", process(&input.replace('\r', ""))?);
        Ok(())
    }
}

use fxhash::FxHashMap;
use itertools::Itertools;
use rayon::prelude::*;

use miette::{miette, Result};

use crate::cache_it;

pub fn process(input: &str) -> Result<String> {
    let (patterns, towels) = input.split_once("\n\n").ok_or(miette!("Invalid input"))?;
    let patterns = patterns.split(", ").collect_vec();
    let result = towels
        .lines()
        .par_bridge()
        .map(|towel| count_matching_pattern_combinations_cached(towel, &patterns))
        .sum::<usize>();
    Ok(result.to_string())
}

pub fn count_matching_pattern_combinations_cached(towel: &str, patterns: &Vec<&str>) -> usize {
    cache_it!(
        FxHashMap<String, usize>,
        FxHashMap::default(),
        towel.to_string(),
        count_matching_pattern_combinations(towel, patterns)
    )
}

fn count_matching_pattern_combinations(towel: &str, patterns: &Vec<&str>) -> usize {
    if towel.is_empty() {
        1
    } else {
        patterns
            .par_iter()
            .filter(|&pattern| towel.starts_with(pattern))
            .map(|pattern| {
                count_matching_pattern_combinations_cached(&towel[pattern.len()..], patterns)
            })
            .sum::<usize>()
    }
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
        assert_eq!("615388132411142", process(&input.replace('\r', ""))?);
        Ok(())
    }
}

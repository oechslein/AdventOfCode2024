use fxhash::FxHashMap;
use itertools::Itertools;

use miette::{miette, Result};

use crate::cache_it;

//#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let (patterns, towels) = input.split_once("\n\n").ok_or(miette!("Invalid input"))?;
    let patterns = patterns.split(", ").collect_vec();
    let result = towels
        .lines()
        .filter(|towel| is_matching_any_pattern_cached(towel, &patterns))
        .count();
    Ok(result.to_string())
}

fn is_matching_any_pattern_cached(towel: &str, patterns: &Vec<&str>) -> bool {
    cache_it!(
        FxHashMap<String, bool>,
        FxHashMap::default(),
        towel.to_string(),
        is_matching_any_pattern(towel, patterns)
    )
}

fn is_matching_any_pattern(towel: &str, patterns: &Vec<&str>) -> bool {
    if towel.is_empty() {
        return true;
    }
    let matched = patterns.iter().any(|pattern| {
        towel.starts_with(pattern)
            && is_matching_any_pattern_cached(&towel[pattern.len()..], patterns)
    });

    matched
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
        assert_eq!("6", process(&input.replace('\r', ""))?);
        Ok(())
    }

    #[test]
    fn test_input() -> miette::Result<()> {
        let input = include_str!("../input1.txt");
        assert_eq!("283", process(&input.replace('\r', ""))?);
        Ok(())
    }
}

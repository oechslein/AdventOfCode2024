use fxhash::FxHashSet;
use itertools::Itertools;

use miette::{miette, Result};

//#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let (patterns, towels) = input.split_once("\n\n").ok_or(miette!("Invalid input"))?;
    let patterns = patterns.split(", ").collect_vec();
    let mut miss_matching_patterns: FxHashSet<_> = FxHashSet::default();
    let result = towels
        .lines()
        //.par_bridge()
        .filter(|towel| is_matching_any_pattern(towel, &patterns, &mut miss_matching_patterns))
        .count();
    Ok(result.to_string())
}

fn is_matching_any_pattern(
    towel: &str,
    patterns: &Vec<&str>,
    miss_matching_patterns: &mut FxHashSet<String>,
) -> bool {
    if towel.is_empty() {
        return true;
    }
    if miss_matching_patterns.contains(towel) {
        return false;
    }
    let matched = patterns
        .iter()
        .any(|pattern| is_matching_pattern(towel, pattern, patterns, miss_matching_patterns));
    if !matched {
        miss_matching_patterns.insert(towel.to_string());
    }

    matched
}

fn is_matching_pattern(
    towel: &str,
    pattern: &str,
    patterns: &Vec<&str>,
    miss_matching_patterns: &mut FxHashSet<String>,
) -> bool {
    if !towel.starts_with(pattern) {
        return false;
    }

    is_matching_any_pattern(&towel[pattern.len()..], patterns, miss_matching_patterns)
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

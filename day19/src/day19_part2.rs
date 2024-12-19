use cached::proc_macro::cached;
use itertools::Itertools;
use rayon::prelude::*;

use miette::{miette, Result};

//#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let (patterns, towels) = input.split_once("\n\n").ok_or(miette!("Invalid input"))?;
    let patterns = patterns.split(", ").map(ToString::to_string).collect_vec();
    let result = towels
        .lines()
        .par_bridge()
        .map(|towel| count_matching_pattern_combinations(towel.to_string(), patterns.clone()))
        .sum::<usize>();
    Ok(result.to_string())
}

#[cached]
fn count_matching_pattern_combinations(towel: String, patterns: Vec<String>) -> usize {
    if towel.is_empty() {
        return 1;
    }

    patterns
        .par_iter()
        .filter(|&pattern| towel.starts_with(pattern))
        .map(|pattern| {
            count_matching_pattern_combinations(
                towel[pattern.len()..].to_string(),
                patterns.clone(),
            )
        })
        .sum::<usize>()
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

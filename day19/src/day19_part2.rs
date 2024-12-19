use std::sync::{LazyLock, Mutex};

use fxhash::FxHashMap;
use itertools::Itertools;
use rayon::prelude::*;

use miette::{miette, Result};

//#[tracing::instrument]
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
    // LazyLock to initialize a static Mutex<FxHashMap<String, usize>>
    // Mutex to lock the FxHashMap because of the Rayon parallel iterator
    static CACHE: LazyLock<Mutex<FxHashMap<String, usize>>> =
        LazyLock::new(|| Mutex::new(FxHashMap::default()));

    let key = towel.to_string();
    // Lock the cache in this block
    {
        let cache = CACHE.lock().unwrap();
        if let Some(counts) = cache.get(&key) {
            return *counts;
        }
    }

    let counts = count_matching_pattern_combinations(towel, patterns);

    // Lock the cache in this block
    {
        let mut cache = CACHE.lock().unwrap();
        cache.insert(key, counts);

        counts
    }
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

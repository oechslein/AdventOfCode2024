use fxhash::FxHashMap;
use itertools::Itertools;
use num_traits::ToPrimitive;
use rayon::prelude::*;
use utils::split_by_newline;

use crate::custom_error::AocError;

//#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let input: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect()
        })
        .collect();

    let result = input
        .iter()
        // .inspect(|report| {
        //     let x = check_if_safe(report, true);
        //     let y = check_if_safe(report, false);
        //     let report_diffs = report
        //     .iter()
        //     .tuple_windows()
        //     .map(|(l, r)| l - r).collect_vec();
        //     println!("{report:?}: {report_diffs:?} {x} {y}");
        // })
        .filter(|report| check_if_safe(report, true) || check_if_safe(report, false))
        .count();
    Ok(result.to_string())
}

fn check_if_safe(report: &[i32], is_decreasing: bool) -> bool {
    report
        .iter()
        .tuple_windows()
        .map(|(l, r)| l - r)
        .all(|diff| {
            (1 <= diff.abs())
                && (diff.abs() <= 3)
                && ((diff > 0 && !is_decreasing) || (diff < 0 && is_decreasing))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("2", process(input)?);
        Ok(())
    }

    #[test]
    fn test_part1() -> miette::Result<()> {
        let input = include_str!("../input1.txt");
        assert_eq!("359", process(input)?);
        Ok(())
    }
}

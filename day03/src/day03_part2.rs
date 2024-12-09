use std::{
    iter::{once, zip, Peekable},
    str::Chars,
};

use fxhash::FxHashMap;
use itertools::{Itertools, PeekingNext};
use num_traits::ToPrimitive;
use rayon::prelude::*;

use crate::custom_error::AocError;

//#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let input = input.chars().collect_vec();
    let mut index = 0;

    let mut result = 0;
    let mut ignore_muls = false;

    while index < input.len() {
        if eat_if_same("don't()", &input, &mut index) {
            ignore_muls = true;
            continue;
        }
        if eat_if_same("do()", &input, &mut index) {
            ignore_muls = false;
            continue;
        }
        if eat("mul(", &input, &mut index) && !ignore_muls {
            if let Some(number1) = number(3, &input, &mut index) {
                if eat(",", &input, &mut index) {
                    if let Some(number2) = number(3, &input, &mut index) {
                        if eat(")", &input, &mut index) {
                            result += number1 * number2;
                        }
                    }
                }
            }
        }
    }

    Ok(result.to_string())
}

fn eat(to_find: &str, input: &[char], index: &mut usize) -> bool {
    if eat_if_same(to_find, input, index) {
        return true;
    }
    *index += 1;
    false
}

fn number(digit_count: usize, input: &[char], index: &mut usize) -> Option<u32> {
    let digits = input
        .iter()
        .skip(*index)
        .take(digit_count)
        .take_while(|ch| ch.is_ascii_digit())
        .collect::<String>();

    if digits.is_empty() {
        return None;
    }

    let result = digits.parse().ok();
    *index += digits.len();
    result
}

fn eat_if_same(to_find: &str, input: &[char], index: &mut usize) -> bool {
    let found = itertools::equal(
        to_find.chars(),
        input.iter().skip(*index).take(to_find.len()).cloned(),
    );
    if !found {
        return false;
    }
    *index += to_find.len();
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_example() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> miette::Result<()> {
        let input = include_str!("../input2.txt");
        assert_eq!("75920122", process(input)?);
        Ok(())
    }
}

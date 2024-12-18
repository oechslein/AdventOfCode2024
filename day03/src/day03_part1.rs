use std::{
    iter::{once, Peekable},
    str::Chars,
};

use fxhash::FxHashMap;
use itertools::{Itertools, PeekingNext};
use num_traits::ToPrimitive;
use rayon::prelude::*;



//#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut input = input.chars().peekable();
    let mut result = 0;

    while input.peek().is_some() {
        if eat("mul(", &mut input) {
            if let Some(number1) = number(3, &mut input) {
                if eat(",", &mut input) {
                    if let Some(number2) = number(3, &mut input) {
                        if eat(")", &mut input) {
                            result += number1 * number2;
                        }
                    }
                }
            }
        }
    }

    Ok(result.to_string())
}

fn number(digit_count: i32, input: &mut Peekable<Chars>) -> Option<u32> {
    match input.next_if(|next_char| next_char.is_ascii_digit()) {
        Some(first_digit_char) => {
            let chars = (1..digit_count)
                .flat_map(|_| input.next_if(|next_char| next_char.is_ascii_digit()))
                .collect_vec();
            once(first_digit_char)
                .chain(chars)
                .collect::<String>()
                .parse()
                .ok()
        }
        None => None,
    }
}

fn eat(to_find: &str, input: &mut Peekable<Chars>) -> bool {
    for ch in to_find.chars() {
        if let Some(next_char) = input.next() {
            if next_char != ch {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() -> miette::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input)?);
        Ok(())
    }

    #[test]
    fn test_part1() -> miette::Result<()> {
        let input = include_str!("../input1.txt");
        assert_eq!("156388521", process(input)?);
        Ok(())
    }
}

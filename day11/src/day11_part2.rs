use crate::custom_error::AocError;
use cached::proc_macro::cached;
use rayon::prelude::*;

//#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let input = input
        .split(' ')
        .map(|num_str| num_str.parse::<u64>().unwrap());
    let cycles = 75;
    let result: u64 = input.par_bridge().map(|num| evolve(num, cycles)).sum();
    Ok(result.to_string())
}

#[cached]
fn evolve(num: u64, cycles_left: u64) -> u64 {
    if cycles_left == 0 {
        return 1;
    }

    if num == 0 {
        return evolve(1, cycles_left - 1);
    }

    let digit_count = digit_count(num);
    if digit_count % 2 != 0 {
        return evolve(num * 2024, cycles_left - 1);
    }

    let (first_half, second_half) = split_in_two_tuple(digit_count, num);
    evolve(first_half, cycles_left - 1) + evolve(second_half, cycles_left - 1)
}

fn split_in_two_tuple(digit_count: u32, num: u64) -> (u64, u64) {
    const RADIX: u64 = 10;
    let half_digits = digit_count / 2;
    let divisor = RADIX.pow(half_digits);

    let second_half = num % divisor;
    let first_half = num / divisor;
    (first_half, second_half)
}

fn digit_count(num: u64) -> u32 {
    debug_assert_ne!(num, 0);
    num.ilog10() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_example() -> miette::Result<()> {
        let input = "125 17";
        assert_eq!("65601038650482", process(input)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> miette::Result<()> {
        let input = include_str!("../input2.txt");
        assert_eq!("238317474993392", process(input)?);
        Ok(())
    }
}

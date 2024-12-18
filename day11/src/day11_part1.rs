

//#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let input = input
        .split(' ')
        .map(|num_str| num_str.parse::<u128>().unwrap());
    let result: u128 = input.map(|num| evolve(num, 25)).sum();
    Ok(result.to_string())
}

fn evolve(num: u128, cycles_left: usize) -> u128 {
    if cycles_left == 0 {
        return 1;
    }

    if num == 0 {
        return evolve(1, cycles_left - 1);
    }

    let mut digit_count = 0;
    let mut temp = num;
    while temp > 0 {
        temp /= 10;
        digit_count += 1;
    }

    if digit_count % 2 != 0 {
        return evolve(num * 2024, cycles_left - 1);
    }

    let half_digits = digit_count / 2;
    let divisor = 10u128.pow(half_digits);

    let second_half = num % divisor;
    let first_half = num / divisor;

    evolve(first_half, cycles_left - 1) + evolve(second_half, cycles_left - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() -> miette::Result<()> {
        let input = "125 17";
        assert_eq!("55312", process(input)?);
        Ok(())
    }

    #[test]
    fn test_part1() -> miette::Result<()> {
        let input = include_str!("../input1.txt");
        assert_eq!("200446", process(input)?);
        Ok(())
    }
}

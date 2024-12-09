use crate::custom_error::AocError;
use counter::Counter;

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut array_left: Vec<i32> = Vec::new();
    let mut array_right: Vec<i32> = Vec::new();

    for line in input.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        assert_eq!(numbers.len(), 2);
        array_left.push(numbers[0]);
        array_right.push(numbers[1]);
    }

    let array_right_counts = array_right.into_iter().collect::<Counter<_>>();

    let output: usize = array_left
        .into_iter()
        .map(|num_left| usize::try_from(num_left).unwrap() * array_right_counts[&num_left])
        .sum();

    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", process(input)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> miette::Result<()> {
        let input = include_str!("../input2.txt");
        assert_eq!("23387399", process(input)?);
        Ok(())
    }
}

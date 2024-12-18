use itertools::Itertools;


use std::iter::zip;


pub fn process(input: &str) -> miette::Result<String> {
    let mut array_left: Vec<i64> = Vec::new();
    let mut array_right: Vec<i64> = Vec::new();

    for line in input.lines() {
        let numbers: Vec<i64> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        assert_eq!(numbers.len(), 2);
        array_left.push(numbers[0]);
        array_right.push(numbers[1]);
    }

    let output: i64 = zip(array_left.into_iter().sorted(), array_right.into_iter().sorted())
        //.inspect(|(l, r)| print!("{l}+{r}=", ))
        .map(|(l, r)| (l - r).abs())
        //.inspect(|res| println!("{res}", ))
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
        assert_eq!("11", process(input)?);
        Ok(())
    }

    #[test]
    fn test_part1() -> miette::Result<()> {
        let input = include_str!("../input1.txt");
        assert_eq!("1197984", process(input)?);
        Ok(())
    }

}

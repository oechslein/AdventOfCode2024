use miette::Result;
use rayon::prelude::*;
use std::iter::successors;

type NumberType = usize;

//#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let input = input
        .lines()
        .map(|line| line.parse::<NumberType>().unwrap());

    let result: usize = input
        .par_bridge()
        .map(|secret| gen_secrets(secret).nth(2000).unwrap())
        .sum();
    Ok(result.to_string())
}

fn gen_secrets(secret: NumberType) -> impl Iterator<Item = NumberType> {
    successors(Some(secret), |&s| Some(next_secret(s)))
}

fn next_secret(mut secret: NumberType) -> NumberType {
    const MASK: NumberType = (1 << 24) - 1;
    secret = (secret ^ (secret << 6)) & MASK;
    secret = (secret ^ (secret >> 5)) & MASK;
    secret = (secret ^ (secret << 11)) & MASK;
    secret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> miette::Result<()> {
        let input = "1
10
100
2024";
        assert_eq!("37327623", process(&input.replace('\r', ""))?);
        Ok(())
    }

    #[test]
    fn test_input() -> miette::Result<()> {
        let input = include_str!("../input1.txt");
        assert_eq!("21147129593", process(&input.replace('\r', ""))?);
        Ok(())
    }
}

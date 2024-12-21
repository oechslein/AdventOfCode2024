use miette::Result;

//#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    Ok(crate::day21::solve(input, 25).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> miette::Result<()> {
        let input = "029A
980A
179A
456A
379A";
        assert_eq!("154115708116294", process(&input.replace('\r', ""))?);
        Ok(())
    }

    #[test]
    fn test_input() -> miette::Result<()> {
        let input = include_str!("../input2.txt");
        assert_eq!("231309103124520", process(&input.replace('\r', ""))?);
        Ok(())
    }
}

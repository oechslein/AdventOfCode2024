use miette::Result;

//#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    Ok(crate::day21::solve(input, 2).to_string())
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
        assert_eq!("126384", process(&input.replace('\r', ""))?);
        Ok(())
    }

    #[test]
    fn test_input() -> miette::Result<()> {
        let input = include_str!("../input1.txt");
        assert_eq!("184180", process(&input.replace('\r', ""))?);
        Ok(())
    }
}

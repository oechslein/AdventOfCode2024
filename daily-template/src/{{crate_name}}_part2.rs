use itertools::Itertools;
use num_traits::ToPrimitive;
use rayon::prelude::*;

use miette::{miette, Error, Result};

//#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    Ok(input.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> miette::Result<()> {
        let input = "";
        assert_eq!("", process(&input.replace('\r', ""))?);
        Ok(())
    }

    #[test]
    fn test_input() -> miette::Result<()> {
        let input = include_str!("../input2.txt");
        assert_eq!("", process(&input.replace('\r', ""))?);
        Ok(())
    }
}

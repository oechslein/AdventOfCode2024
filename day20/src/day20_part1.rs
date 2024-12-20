use miette::Result;

pub fn process(input: &str) -> Result<String> {
    Ok(crate::solve::solve(input, 100, 2).to_string())
    //Ok(crate::solve::solve(input, 1, 2).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> miette::Result<()> {
        let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        assert_eq!(
            14 + 14 + 2 + 4 + 2 + 3 + 1 + 1 + 1 + 1 + 1, // 44
            crate::solve::solve(input, 1, 2)
        );
        Ok(())
    }

    #[test]
    fn test_input() -> miette::Result<()> {
        let input = include_str!("../input1.txt");
        assert_eq!("1422", process(&input.replace('\r', ""))?);
        Ok(())
    }
}

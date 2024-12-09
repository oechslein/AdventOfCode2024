use crate::custom_error::AocError;

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let input = input.lines().map(|line| {
        let (result, args_str) = line.split_once(": ").unwrap();

        let args: Vec<usize> = args_str
            .split(' ')
            .map(|arg_str| arg_str.parse().unwrap())
            .collect();
        (result.parse::<usize>().unwrap(), args)
    });

    let result: usize = input
        .filter(|(result, args)| is_solvable(*result, args))
        .map(|(result, _)| result)
        .sum();

    Ok(result.to_string())
}

fn is_solvable(result: usize, args: &[usize]) -> bool {
    is_solvable_rec(result, 0, args)
}

fn is_solvable_rec(result: usize, temp_result: usize, args: &[usize]) -> bool {
    if temp_result > result {
        return false;
    }

    if args.is_empty() {
        return result == temp_result;
    }

    let next_arg = args[0];
    let remaining_args = &args[1..];

    is_solvable_rec(result, temp_result + next_arg, remaining_args)
        || is_solvable_rec(result, temp_result * next_arg, remaining_args)
        || is_solvable_rec(result, concatenate(temp_result, next_arg), remaining_args)
}

fn concatenate(arg1: usize, arg2: usize) -> usize {
    format!("{arg1}{arg2}").parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_example() -> miette::Result<()> {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(11387.to_string(), process(input)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> miette::Result<()> {
        let input = include_str!("../input2.txt");
        assert_eq!("500335179214836", process(input)?);
        Ok(())
    }
}

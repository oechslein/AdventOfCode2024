use itertools::Itertools;
use crate::custom_error::AocError;
use nalgebra::{Matrix2, Vector2};

#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
fn solve_system(
    prize_x: i128,
    prize_y: i128,
    a_x: i128,
    a_y: i128,
    b_x: i128,
    b_y: i128,
) -> Option<(i128, i128)> {
    let coeff_matrix = Matrix2::new(a_x as f64, b_x as f64, a_y as f64, b_y as f64);
    let constants = Vector2::new(prize_x as f64, prize_y as f64);
    let solution = coeff_matrix.lu().solve(&constants)?;
    let button_a = solution[0].round() as i128;
    let button_b = solution[1].round() as i128;

    ((prize_x == button_a * a_x + button_b * b_x)
        && (prize_y == button_a * a_y + button_b * b_y)).then_some((button_a, button_b))
}


//#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let var_name = "\r";
    let input = input.replace(var_name, "");
    let input = input.split("\n\n").map(|block| {
        /* block is eg
        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400
        */
        let mut lines = block.lines();
        let a = lines.next().unwrap();
        let b = lines.next().unwrap();
        let prize = lines.next().unwrap();

        let a = a.split(": ").nth(1).unwrap();
        let b = b.split(": ").nth(1).unwrap();
        let prize = prize.split(": ").nth(1).unwrap();

        let (a_x, a_y) = a
            .split(", ")
            .map(|x| x.split('+').nth(1).unwrap().parse::<i128>().unwrap())
            .collect_tuple()
            .unwrap();
        let (b_x, b_y) = b
            .split(", ")
            .map(|x| x.split('+').nth(1).unwrap().parse::<i128>().unwrap())
            .collect_tuple()
            .unwrap();
        let (prize_x, prize_y) = prize
            .split(", ")
            .map(|x| x.split('=').nth(1).unwrap().parse::<i128>().unwrap())
            .collect_tuple()
            .unwrap();

        (
            (a_x, a_y),
            (b_x, b_y),
            (10_000_000_000_000 + prize_x, 10_000_000_000_000 + prize_y),
        )
    });

    let mut result = 0;
    for ((a_x, a_y), (b_x, b_y), (prize_x, prize_y)) in input {
        if let Some((button_a, button_b)) = solve_system(prize_x, prize_y, a_x, a_y, b_x, b_y) {
            println!(
                "A: {}, B: {} => {}",
                button_a,
                button_b,
                button_a * 3 + button_b
            );
            result += button_a * 3 + button_b;
        } else {
            println!("No solution");
        }
    }

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_example() -> miette::Result<()> {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!("875318608908", process(input)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> miette::Result<()> {
        let input = include_str!("../input2.txt");
        assert_eq!("99968222587852", process(input)?);
        Ok(())
    }
}

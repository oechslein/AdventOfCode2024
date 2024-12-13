use crate::custom_error::AocError;
use itertools::Itertools;
use nalgebra::{Matrix2, Vector2};

#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
fn solve_system(prize_vector: Vector2<u64>, coeff_matrix: Matrix2<u64>) -> Option<Vector2<u64>> {
    /*
     prize_x = a_x * button_a + b_x * button_b
     prize_y = a_y * button_a + b_y * button_b
     =>
     [ prize_x ] = (a_x, b_x ] * [ button_a ]
     [ prize_y ] = (a_y, b_y ] * [ button_b ]
     =>
     [ button_a ] = (a_x, b_x ]^-1 * [ prize_x ]
     [ button_b ] = (a_y, b_y ]^-1 * [ prize_y ]
    */
    // need floats to calculate the inverse
    let coeff_matrix_float = coeff_matrix.map(|elem| elem as f64);
    coeff_matrix_float
        .try_inverse()
        .and_then(|coeff_matrix_float_inverse| {
            let prize_vector_float = prize_vector.map(|elem| elem as f64);
            let solution_float = coeff_matrix_float_inverse * prize_vector_float;
            let solution = solution_float.map(|x| x.round() as u64);
            // need to check if we have an integer solution
            if coeff_matrix * solution == prize_vector {
                Some(solution)
            } else {
                //println!("{} != {}", coeff_matrix * solution, prize_vector);
                None
            }
        })
}

#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
//#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let additional_price_vec = Vector2::new(10_000_000_000_000, 10_000_000_000_000);
    let result = parse_input(input).map(|(prize_vec, coeff_matrix)| {
        if let Some(button_vec) = solve_system(prize_vec + additional_price_vec, coeff_matrix) {
            let costs = button_vec[0] * 3 + button_vec[1];
            //println!("A/B: {button_vec} => {costs}");
            costs
        } else {
            //println!("No solution");
            0
        }
    }).sum::<u64>();

    Ok(result.to_string())
}

#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
fn parse_input(input: &str) -> impl Iterator<Item = (Vector2<u64>, Matrix2<u64>)> + '_ {
    let input = input.split("\n\n").map(|block| {
        /* block is for example
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
            .map(|x| x.split('+').nth(1).unwrap().parse::<u64>().unwrap())
            .collect_tuple()
            .unwrap();
        let (b_x, b_y) = b
            .split(", ")
            .map(|x| x.split('+').nth(1).unwrap().parse::<u64>().unwrap())
            .collect_tuple()
            .unwrap();
        let (prize_x, prize_y) = prize
            .split(", ")
            .map(|x| x.split('=').nth(1).unwrap().parse::<u64>().unwrap())
            .collect_tuple()
            .unwrap();

        let coeff_matrix = Matrix2::new(a_x, b_x, a_y, b_y);
        let prize_vec = Vector2::new(prize_x, prize_y);

        (prize_vec, coeff_matrix)
    });
    input
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
        assert_eq!("875318608908", process(&input.replace('\r', ""))?);
        Ok(())
    }

    #[test]
    fn test_part2() -> miette::Result<()> {
        let input = include_str!("../input2.txt");
        assert_eq!("99968222587852", process(&input.replace('\r', ""))?);
        Ok(())
    }
}

use itertools::Itertools;



use nalgebra::{Matrix2, Vector2};


#[allow(clippy::cast_sign_loss)]
fn solve_system(
    prize_x: u32,
    prize_y: u32,
    a_x: u32,
    a_y: u32,
    b_x: u32,
    b_y: u32,
) -> Option<(u32, u32)> {
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
    let coeff_matrix = Matrix2::new(a_x, b_x, a_y, b_y).map(f64::from);
    let constants = Vector2::new(prize_x, prize_y).map(f64::from);
    coeff_matrix.try_inverse().and_then(|coeff_matrix_inverse| {
        let solution = coeff_matrix_inverse * constants;
        let button_a = solution[0].round() as u32;
        let button_b = solution[1].round() as u32;
        ((prize_x == button_a * a_x + button_b * b_x)
            && (prize_y == button_a * a_y + button_b * b_y))
            .then_some((button_a, button_b))
    })
}


//#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let input = input.replace('\r', "");
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
            .map(|x| x.split('+').nth(1).unwrap().parse::<u32>().unwrap())
            .collect_tuple()
            .unwrap();
        let (b_x, b_y) = b
            .split(", ")
            .map(|x| x.split('+').nth(1).unwrap().parse::<u32>().unwrap())
            .collect_tuple()
            .unwrap();
        let (prize_x, prize_y) = prize
            .split(", ")
            .map(|x| x.split('=').nth(1).unwrap().parse::<u32>().unwrap())
            .collect_tuple()
            .unwrap();

        ((a_x, a_y), (b_x, b_y), (prize_x, prize_y))
    });

    let mut result = 0;
    for ((a_x, a_y), (b_x, b_y), (prize_x, prize_y)) in input {
        if let Some((button_a, button_b)) =
            solve_system(prize_x, prize_y, a_x, a_y, b_x, b_y)
        {
            /* println!(
                "A: {}, B: {} => {}",
                button_a,
                button_b,
                button_a * 3 + button_b
            ); */
            result += button_a * 3 + button_b;
        } else {
            //  println!("No solution");
        }
    }

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() -> miette::Result<()> {
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
        assert_eq!("480", process(input)?);
        Ok(())
    }

    #[test]
    fn test_part1() -> miette::Result<()> {
        let input = include_str!("../input1.txt");
        assert_eq!("29187", process(input)?);
        Ok(())
    }
}

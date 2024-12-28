use itertools::Itertools;

use miette::Result;

use crate::day24_common::{Expr, Op, Puzzle, Wire};

pub fn process(input: &str) -> Result<String> {
    let mut puzzle: Puzzle = input.parse().unwrap();
    let mut swaps = Vec::new();

    let all_wires: Vec<Wire> = puzzle.all_wires().collect();
    for _ in 0..4 {
        let baseline = progress(&puzzle);
        for (a, b) in all_wires.iter().tuple_combinations() {
            swap_wires(&mut puzzle, a, b);
            let progress = progress(&puzzle);
            if progress > baseline {
                swaps.push([a, b]);
                break;
            }
            swap_wires(&mut puzzle, a, b);
        }
    }

    let result = swaps
        .into_iter()
        .flatten()
        .map(ToString::to_string)
        .sorted()
        .join(",");

    let old_wire_values = puzzle.wire_values.clone();
    assert!(test_if_adder(&mut puzzle));
    assert_eq!(old_wire_values, puzzle.wire_values);

    Ok(result)
}

fn test_adder_count(puzzle: &mut Puzzle) -> u8 {
    (0..puzzle.inputs_count())
        .filter(|num| test_adder_output(puzzle, *num))
        .count() as u8
}
fn test_if_adder(puzzle: &mut Puzzle) -> bool {
    test_adder_count(puzzle) == puzzle.inputs_count()
}

fn test_adder_output(puzzle: &mut Puzzle, num: u8) -> bool {
    let old_wire_values = puzzle.wire_values.clone();

    let mut test_output_xyz = |x: bool, y: bool, z_0: bool, z_1: bool| {
        for wire_num in 0..puzzle.inputs_count() {
            if wire_num == num {
                puzzle.wire_values.insert(Wire::InputX(wire_num), x);
                puzzle.wire_values.insert(Wire::InputY(wire_num), y);
            } else {
                puzzle.wire_values.insert(Wire::InputX(wire_num), false);
                puzzle.wire_values.insert(Wire::InputY(wire_num), false);
            }
        }

        (0..=puzzle.inputs_count()).all(|wire_num| {
            let wire_z = Wire::OutputZ(wire_num);
            let wire_z_value = wire_z.value_of(puzzle);
            if num == wire_num {
                wire_z_value == Some(z_0)
            } else if num + 1 == wire_num {
                wire_z_value == Some(z_1)
            } else {
                wire_z_value == Some(false)
            }
        })
    };

    let result = [true, false]
        .into_iter()
        .cartesian_product([true, false])
        .all(|(x, y)| match (x, y) {
            (true, true) => test_output_xyz(x, y, false, true),
            (true, false) | (false, true) => test_output_xyz(x, y, true, false),
            (false, false) => test_output_xyz(x, y, false, false),
        });

    puzzle.wire_values.extend(old_wire_values.clone());
    debug_assert_eq!(old_wire_values, puzzle.wire_values);
    result
}

#[allow(unused_variables)]
fn is_ok_z(puzzle: &Puzzle, z_wire: &Wire, num: u8) -> bool {
    match puzzle.ops.get(z_wire) {
        Some(Expr { lhs, op, rhs }) if num == 0 && *op == Op::Xor => check_if_inputs(0, lhs, rhs),
        Some(Expr { lhs, op, rhs }) if *op == Op::Xor => {
            (is_ok_xor(puzzle, lhs, num) && is_ok_carry_bit(puzzle, rhs, num))
                || (is_ok_xor(puzzle, rhs, num) && is_ok_carry_bit(puzzle, lhs, num))
        }
        _ => false,
    }
}

#[allow(unused_variables)]
fn is_ok_xor(puzzle: &Puzzle, wire: &Wire, num: u8) -> bool {
    match puzzle.ops.get(wire) {
        Some(Expr { lhs, op, rhs }) if *op == Op::Xor => check_if_inputs(num, lhs, rhs),
        _ => false,
    }
}

#[allow(unused_variables)]
fn is_ok_carry_bit(puzzle: &Puzzle, wire: &Wire, num: u8) -> bool {
    match puzzle.ops.get(wire) {
        Some(Expr { lhs, op, rhs }) if num == 1 && *op == Op::And => check_if_inputs(0, lhs, rhs),
        Some(Expr { lhs, op, rhs }) if num > 1 && *op == Op::Or => {
            (is_ok_direct_carry(puzzle, lhs, num - 1) && is_ok_recarry(puzzle, rhs, num - 1))
                || (is_ok_direct_carry(puzzle, rhs, num - 1) && is_ok_recarry(puzzle, lhs, num - 1))
        }
        _ => false,
    }
}

#[allow(unused_variables)]
fn is_ok_direct_carry(puzzle: &Puzzle, wire: &Wire, num: u8) -> bool {
    match puzzle.ops.get(wire) {
        Some(Expr { lhs, op, rhs }) if *op == Op::And => check_if_inputs(num, lhs, rhs),
        _ => false,
    }
}

#[allow(unused_variables)]
fn is_ok_recarry(puzzle: &Puzzle, wire: &Wire, num: u8) -> bool {
    match puzzle.ops.get(wire) {
        Some(Expr { lhs, op, rhs }) if *op != Op::And => false,
        Some(Expr { lhs, op, rhs }) => {
            (is_ok_xor(puzzle, lhs, num) && is_ok_carry_bit(puzzle, rhs, num))
                || (is_ok_xor(puzzle, rhs, num) && is_ok_carry_bit(puzzle, lhs, num))
        }
        _ => false,
    }
}

fn check_if_inputs(num: u8, lhs: &Wire, rhs: &Wire) -> bool {
    debug_assert!(lhs < rhs);
    matches!((lhs, rhs), (Wire::InputX(lhs_num), Wire::InputY(rhs_num)) if lhs_num == &num && rhs_num == &num)
}

fn progress(puzzle: &Puzzle) -> u8 {
    (0..100)
        .find(|&idx| !is_ok_z(puzzle, &Wire::OutputZ(idx), idx))
        .unwrap()
}

fn swap_wires(puzzle: &mut Puzzle, a: &Wire, b: &Wire) {
    let temp = puzzle.ops.insert(a.clone(), puzzle.ops[b].clone()).unwrap();
    puzzle.ops.insert(b.clone(), temp);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() -> miette::Result<()> {
        let input = include_str!("../input2.txt");
        assert_eq!(
            "cgr,hpc,hwk,qmd,tnt,z06,z31,z37",
            process(&input.replace('\r', ""))?
        );
        Ok(())
    }
}

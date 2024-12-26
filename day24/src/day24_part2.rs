use fxhash::FxHashMap;
use itertools::Itertools;

use miette::Result;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Op {
    And,
    Or,
    Xor,
}

#[derive(Clone, Copy)]
struct Expr<'a> {
    lhs: &'a str,
    op: Op,
    rhs: &'a str,
}

pub fn process(input: &str) -> Result<String> {
    let (_, mut ops) = parse(input);
    let mut swaps = Vec::new();

    let all_wires: Vec<&str> = ops.keys().copied().collect();
    for _ in 0..4 {
        let baseline = progress(&ops);
        for (a, b) in all_wires.iter().tuple_combinations() {
            swap_wires(&mut ops, a, b);
            if progress(&ops) > baseline {
                swaps.push([*a, *b]);
                break;
            }
            swap_wires(&mut ops, a, b);
        }
    }

    let result = swaps.into_iter().flatten().sorted().join(",");

    Ok(result)
}

fn parse(input: &str) -> (FxHashMap<&str, bool>, FxHashMap<&str, Expr>) {
    let (initial, connections) = input.split_once("\n\n").unwrap();
    let wires = initial
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(": ").unwrap();
            (left, right == "1")
        })
        .collect();
    let operations = connections
        .lines()
        .map(|line| {
            let (input, output) = line.split_once(" -> ").unwrap();
            let (lhs, op, rhs) = input.split_whitespace().collect_tuple().unwrap();
            let op = match op {
                "AND" => Op::And,
                "OR" => Op::Or,
                "XOR" => Op::Xor,
                _ => panic!("at the disco"),
            };
            (output, Expr { lhs, op, rhs })
        })
        .collect();
    (wires, operations)
}

#[allow(unused_variables)]
fn is_ok_z(ops: &FxHashMap<&str, Expr>, z_wire: &str, num: i32) -> bool {
    match ops.get(z_wire) {
        Some(Expr { lhs, op, rhs }) if num == 0 && *op == Op::Xor => check_operands(0, lhs, rhs),
        Some(Expr { lhs, op, rhs }) if *op == Op::Xor => {
            (is_ok_xor(ops, lhs, num) && is_ok_carry_bit(ops, rhs, num))
                || (is_ok_xor(ops, rhs, num) && is_ok_carry_bit(ops, lhs, num))
        }
        _ => false,
    }
}

#[allow(unused_variables)]
fn is_ok_xor(ops: &FxHashMap<&str, Expr>, wire: &str, num: i32) -> bool {
    match ops.get(wire) {
        Some(Expr { lhs, op, rhs }) if *op == Op::Xor => check_operands(num, lhs, rhs),
        _ => false,
    }
}

#[allow(unused_variables)]
fn is_ok_carry_bit(ops: &FxHashMap<&str, Expr>, wire: &str, num: i32) -> bool {
    match ops.get(wire) {
        Some(Expr { lhs, op, rhs }) if num == 1 && *op == Op::And => check_operands(0, lhs, rhs),
        Some(Expr { lhs, op, rhs }) if num > 1 && *op == Op::Or => {
            (is_ok_direct_carry(ops, lhs, num - 1) && is_ok_recarry(ops, rhs, num - 1))
                || (is_ok_direct_carry(ops, rhs, num - 1) && is_ok_recarry(ops, lhs, num - 1))
        }
        _ => false,
    }
}

#[allow(unused_variables)]
fn is_ok_direct_carry(ops: &FxHashMap<&str, Expr>, wire: &str, num: i32) -> bool {
    match ops.get(wire) {
        Some(Expr { lhs, op, rhs }) if *op == Op::And => check_operands(num, lhs, rhs),
        _ => false,
    }
}

#[allow(unused_variables)]
fn is_ok_recarry(ops: &FxHashMap<&str, Expr>, wire: &str, num: i32) -> bool {
    match ops.get(wire) {
        Some(Expr { lhs, op, rhs }) if *op != Op::And => false,
        Some(Expr { lhs, op, rhs }) => {
            (is_ok_xor(ops, lhs, num) && is_ok_carry_bit(ops, rhs, num))
                || (is_ok_xor(ops, rhs, num) && is_ok_carry_bit(ops, lhs, num))
        }
        _ => false,
    }
}

fn check_operands(num: i32, lhs: &str, rhs: &str) -> bool {
    let x_wire = make_wire('x', num);
    let y_wire = make_wire('y', num);
    (lhs == x_wire) && (rhs == y_wire) || (lhs == y_wire) && (rhs == x_wire)
}

fn progress(ops: &FxHashMap<&str, Expr>) -> i32 {
    (0..100)
        .find(|&idx| !is_ok_z(ops, &make_wire('z', idx), idx))
        .unwrap()
}

fn make_wire(wire_type: char, num: i32) -> String {
    format!("{wire_type}{num:02}")
}

fn swap_wires<'a>(map: &mut FxHashMap<&'a str, Expr<'a>>, a: &'a str, b: &'a str) {
    let temp = map.insert(a, map[b]).unwrap();
    map.insert(b, temp);
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

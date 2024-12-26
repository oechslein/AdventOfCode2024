use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

use miette::Result;

//#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let (initial, connections) = input.split_once("\n\n").unwrap();
    let mut wire_values = initial
        .lines()
        .map(|line| {
            let (key, value) = line.split_once(": ").unwrap();
            (key, value.parse::<usize>().unwrap())
        })
        .collect::<FxHashMap<_, _>>();
    // println!("{:?}", wire_values);

    let connections = connections
        .lines()
        .map(|line| {
            let (input, output) = line.split_once(" -> ").unwrap();
            let (input1, op, input2) = input.split(' ').collect_tuple().unwrap();
            assert!(op == "AND" || op == "OR" || op == "XOR");
            ((op, input1, input2), output)
        })
        .collect::<Vec<_>>();

    let all_z_wires = connections
        .iter()
        .map(|(_, output)| output)
        .chain(wire_values.keys())
        .filter(|wire| wire.starts_with('z'))
        .copied()
        .collect::<FxHashSet<_>>();

    while wire_values
        .keys()
        .filter(|wire| wire.starts_with('z'))
        .count()
        < all_z_wires.len()
    {
        // println!("all_z_wires.len(): {}", all_z_wires.len());
        // println!(
        //     "acount: {}",
        //     wire_values
        //         .keys()
        //         .filter(|wire| wire.starts_with("z"))
        //         .count()
        // );
        for ((op, input1, input2), output) in &connections {
            if wire_values.contains_key(output) {
                continue;
            }

            if !wire_values.contains_key(input1) || !wire_values.contains_key(input2) {
                // println!(
                //     "skipping: {:?} {:?} {:?} -> {:?}",
                //     op, input1, input2, output
                //);
                continue;
            }

            let input1_value = wire_values[input1];
            let input2_value = wire_values[input2];

            let output_value = match *op {
                "AND" => input1_value & input2_value,
                "OR" => input1_value | input2_value,
                "XOR" => input1_value ^ input2_value,
                _ => panic!("Unknown operator: {op}"),
            };

            // println!(
            //     "{}/{} {} {}/{} -> {}",
            //     input1, input1_value, op, input2, input2_value, output_value
            // );
            wire_values.insert(output, output_value);
        }
    }

    // let x = wire_values
    //     .iter()
    //     .filter(|(wire, _value)| wire.starts_with("z"))
    //     .sorted_by_key(|(wire, _value)| *wire)
    //     .collect_vec();
    // println!("{:?}", x);

    let result = wire_values
        .into_iter()
        .filter(|(wire, _value)| wire.starts_with('z'))
        .sorted_by_key(|(wire, _value)| *wire)
        .rev()
        .fold(0usize, |acc, (_wire, value)| acc * 2 + value);

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> miette::Result<()> {
        let input = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";
        assert_eq!("4", process(&input.replace('\r', ""))?);
        Ok(())
    }

    #[test]
    fn test_example2() -> miette::Result<()> {
        let input = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";
        assert_eq!("2024", process(&input.replace('\r', ""))?);
        Ok(())
    }

    #[test]
    fn test_input() -> miette::Result<()> {
        let input = include_str!("../input1.txt");
        assert_eq!("60614602965288", process(&input.replace('\r', ""))?);
        Ok(())
    }
}

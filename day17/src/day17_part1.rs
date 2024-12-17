use fxhash::FxHashMap;
use itertools::{join, Itertools};
use num_traits::ToPrimitive;
use rayon::prelude::*;

use miette::{miette, Error, Result};

type RegistersType = Vec<usize>;

#[derive(Debug, Clone)]
enum Operation {
    DivisionA(usize),
    BitwiseXorB(usize),
    Modulo8(usize),
    Jump(usize),
    BitwiseXorBC(),
    Output(usize),
    DivisionB(usize),
    DivisionC(usize),
}

impl Operation {
    fn create(op_code: char, argument: usize) -> Result<Operation> {
        match op_code {
            '0' => Ok(Operation::DivisionA(argument)),
            '1' => Ok(Operation::BitwiseXorB(argument)),
            '2' => Ok(Operation::Modulo8(argument)),
            '3' => Ok(Operation::Jump(argument)),
            '4' => Ok(Operation::BitwiseXorBC()),
            '5' => Ok(Operation::Output(argument)),
            '6' => Ok(Operation::DivisionB(argument)),
            '7' => Ok(Operation::DivisionC(argument)),

            _ => Err(miette!("Unknown op code {op_code}")),
        }
    }

    fn parse_argument(argument: &usize, registers: &RegistersType) -> usize {
        match argument {
            _ if argument < &4 => *argument,
            4 => registers[register_name_to_index('A')],
            5 => registers[register_name_to_index('B')],
            6 => registers[register_name_to_index('C')],
            _ => panic!("Argument not possible {argument}"),
        }
    }

    fn execute(&self, registers: &mut RegistersType, instruction_ptr: &mut usize) -> Option<usize> {
        match self {
            Operation::DivisionA(argument) => {
                Operation::execute_division('A', argument, registers);
                *instruction_ptr += 1;
                None
            }
            Operation::BitwiseXorB(argument) => {
                Operation::execute_bitwise_xor('B', argument, registers);
                *instruction_ptr += 1;
                None
            }
            Operation::Modulo8(argument) => {
                let converted_argument = Operation::parse_argument(argument, registers);
                registers[register_name_to_index('B')] = &converted_argument % 8;
                *instruction_ptr += 1;
                None
            }
            Operation::Jump(argument) => {
                if registers[register_name_to_index('A')] > 0 {
                    let converted_argument = *argument; //Operation::parse_argument(argument, registers);
                    *instruction_ptr = converted_argument;
                } else {
                    *instruction_ptr += 1;
                }
                None
            }
            Operation::BitwiseXorBC() => {
                let converted_argument = registers[register_name_to_index('C')];
                Operation::execute_bitwise_xor('B', &converted_argument, registers);
                *instruction_ptr += 1;
                None
            }
            Operation::Output(argument) => {
                let converted_argument = Operation::parse_argument(argument, registers);
                *instruction_ptr += 1;
                Some(converted_argument % 8)
            }
            Operation::DivisionB(argument) => {
                Operation::execute_division('B', argument, registers);
                *instruction_ptr += 1;
                None
            }
            Operation::DivisionC(argument) => {
                Operation::execute_division('C', argument, registers);
                *instruction_ptr += 1;
                None
            }
        }
    }

    fn execute_division(register: char, argument: &usize, registers: &mut RegistersType) {
        let converted_argument = Operation::parse_argument(argument, registers);
        registers[register_name_to_index(register)] =
            registers[register_name_to_index('A')] / 2_usize.pow(converted_argument as u32);
    }

    fn execute_bitwise_xor(register: char, converted_argument: &usize, registers: &mut [usize]) {
        registers[register_name_to_index(register)] ^= converted_argument;
    }
}

fn register_name_to_index(register: char) -> usize {
    match register {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        _ => panic!("Unknown register {register}"),
    }
}

//#[tracing::instrument]
pub fn process(input: &str) -> Result<String> {
    let (mut registers, program) = parse(input)?;

    println!("{registers:?}");
    println!("{program:?}");

    let mut instruction_ptr = 0;
    let mut result = Vec::new();
    while instruction_ptr < program.len() {
        if let Some(output) = program[instruction_ptr].execute(&mut registers, &mut instruction_ptr)
        {
            result.push(output);
        }
        println!("instruction_ptr: {instruction_ptr}");
        println!("Registers: {registers:?}");
        println!("Result: {}", result.iter().join(","));
    }

    Ok(result.into_iter().join(","))
}

fn parse(input: &str) -> Result<(RegistersType, Vec<Operation>), Error> {
    let (registers_str, program_str) = input.split_once("\n\n").ok_or(miette!("Input wrong"))?;
    let registers: RegistersType = registers_str
        .lines()
        .map(|line| {
            let (name_str, value_str) = line["Register ".len()..].split_once(' ').unwrap();
            (name_str.chars().next().unwrap(), value_str.parse().unwrap())
        })
        .sorted_by_key(|(register_name, _)| register_name_to_index(*register_name))
        .map(|(_, value)| value)
        .collect();
    let program_str = &program_str["Program: ".len()..];
    let program: Vec<Operation> = program_str
        .split(',')
        .chunks(2)
        .into_iter()
        .map(|mut sub_iter| {
            let op_str = sub_iter.next().unwrap();
            let argument_str = sub_iter.next().unwrap();
            Operation::create(
                op_str.chars().next().unwrap(),
                argument_str.parse().unwrap(),
            )
            .unwrap()
        })
        .collect();
    Ok((registers, program))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> miette::Result<()> {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!("4,6,3,5,6,3,5,2,1,0", process(&input.replace('\r', ""))?);
        Ok(())
    }

    #[test]
    fn test_input() -> miette::Result<()> {
        let input = include_str!("../input1.txt");
        assert_eq!("3,5,0,1,5,1,5,1,0", process(&input.replace('\r', ""))?);
        Ok(())
    }
}

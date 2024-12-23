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
                let converted_argument = Operation::parse_argument(argument, registers);
                registers[register_name_to_index('A')] =
                    registers[register_name_to_index('A')] >> converted_argument;
                *instruction_ptr += 1;
                None
            }
            Operation::BitwiseXorB(argument) => {
                registers[register_name_to_index('B')] ^= argument;
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
                    *instruction_ptr = *argument;
                } else {
                    *instruction_ptr += 1;
                }
                None
            }
            Operation::BitwiseXorBC() => {
                registers[register_name_to_index('B')] ^= registers[register_name_to_index('C')];
                *instruction_ptr += 1;
                None
            }
            Operation::Output(argument) => {
                *instruction_ptr += 1;
                Some(Operation::parse_argument(argument, registers) % 8)
            }
            Operation::DivisionB(argument) => {
                let converted_argument = Operation::parse_argument(argument, registers);
                registers[register_name_to_index('B')] =
                    registers[register_name_to_index('A')] >> converted_argument;
                *instruction_ptr += 1;
                None
            }
            Operation::DivisionC(argument) => {
                let converted_argument = Operation::parse_argument(argument, registers);
                registers[register_name_to_index('C')] =
                    registers[register_name_to_index('A')] >> converted_argument;
                *instruction_ptr += 1;
                None
            }
        }
    }

    fn generate_code_argument(argument: &usize) -> String {
        match argument {
            _ if argument < &4 => format!("{argument}"),
            4 => "a".to_string(),
            5 => "b".to_string(),
            6 => "b".to_string(),
            _ => panic!("Argument not possible {argument}"),
        }
    }

    fn generate_code(&self) -> String {
        match self {
            Operation::DivisionA(argument) => {
                format!(
                    "a >>= {}; instr += 1;",
                    Operation::generate_code_argument(argument)
                )
            }
            Operation::BitwiseXorB(argument) => format!("b ^= {}; instr += 1;", argument),
            Operation::Modulo8(argument) => {
                format!(
                    "b = {} % 8; instr += 1;",
                    Operation::generate_code_argument(argument)
                )
            }
            Operation::Jump(argument) => {
                format!("instr = if a > 0 {{{argument}}} else {{instr+1}};")
            }
            Operation::BitwiseXorBC() => "b ^= c; instr += 1;".to_string(),
            Operation::Output(argument) => {
                format!(
                    "return Some({} % 8);",
                    Operation::generate_code_argument(argument)
                )
            }
            Operation::DivisionB(argument) => {
                format!(
                    "b = a >> {}; instr += 1;",
                    Operation::generate_code_argument(argument)
                )
            }
            Operation::DivisionC(argument) => {
                format!(
                    "c = a >> {}; instr += 1;",
                    Operation::generate_code_argument(argument)
                )
            }
        }
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
    let (orig_registers, program, program_vec, program_str) = parse(input)?;

    //generate_source_code(&program);

    if program_str == "0,3,5,4,3,0" {
        let result = search_for_a_example(&program_vec);
        Ok(result.to_string())
    } else if program_str == "2,4,1,5,7,5,1,6,4,1,5,5,0,3,3,0" {
        let result = search_for_a_input(&program_vec);
        Ok(result.to_string())
    } else {
        let result = search_for_a(&program_vec, &orig_registers, &program);
        Ok(result.to_string())
    }
}

fn generate_source_code(program: &[Operation]) {
    println!("while instr < {} {{", program.len());
    for (index, op) in program.iter().enumerate() {
        println!("\t// {op:?}");
        println!("\tif instr == {index} {{{}}}", op.generate_code());
    }
    println!("}}");
}

fn search_for_a(program_vec: &[usize], registers: &[usize], program: &[Operation]) -> usize {
    let mut search: Vec<usize> = vec![0];

    for min_length in (0..program_vec.len()).rev() {
        let expected_output = program_vec[min_length];
        let mut next: Vec<usize> = Vec::new();
        for new_a_value in search
            .iter()
            .flat_map(|prev_a| (0..8).map(move |delta| delta + (prev_a * 8)))
        {
            match solve(new_a_value, registers, program) {
                Some(new_output) if new_output == expected_output => {
                    next.push(new_a_value); // 15/3, 14/24;25;29;31
                }
                _ => {}
            }
        }
        search = next;
    }

    search.into_iter().min().unwrap()
}

fn search_for_a_input(program_vec: &[usize]) -> usize {
    let mut search: Vec<usize> = vec![0];

    for min_length in (0..program_vec.len()).rev() {
        let expected_output = program_vec[min_length];
        let mut next: Vec<usize> = Vec::new();
        for new_a_value in search
            .iter()
            .flat_map(|prev_a| (0..8).map(move |delta| delta + (prev_a * 8)))
        {
            match solve_fast_input(new_a_value) {
                Some(new_output) if new_output == expected_output => {
                    next.push(new_a_value); // 15/3, 14/24;25;29;31
                }
                _ => {}
            }
        }
        search = next;
    }

    search.into_iter().min().unwrap()
}

fn search_for_a_example(program_vec: &[usize]) -> usize {
    let mut search: Vec<usize> = vec![0];

    for min_length in (0..program_vec.len()).rev() {
        let expected_output = program_vec[min_length];
        let mut next: Vec<usize> = Vec::new();
        for new_a_value in search
            .iter()
            .flat_map(|prev_a| (0..8).map(move |delta| delta + (prev_a * 8)))
        {
            match solve_fast_example(new_a_value) {
                Some(new_output) if new_output == expected_output => {
                    next.push(new_a_value); // 15/3, 14/24;25;29;31
                }
                _ => {}
            }
        }
        search = next;
    }

    search.into_iter().min().unwrap()
}

fn solve(a_value: usize, registers: &[usize], program: &[Operation]) -> Option<usize> {
    let mut registers = registers.to_vec();
    registers[register_name_to_index('A')] = a_value;
    let mut instruction_ptr = 0;
    while instruction_ptr < program.len() {
        println!(
            "{instruction_ptr} {} {} {}",
            registers[register_name_to_index('A')],
            registers[register_name_to_index('B')],
            registers[register_name_to_index('C')]
        );
        if let Some(output) = program[instruction_ptr].execute(&mut registers, &mut instruction_ptr)
        {
            return Some(output);
        }
    }

    None
}

fn solve_fast_input(a_value: usize) -> Option<usize> {
    let mut instr = 0;
    let mut a = a_value;
    let mut b = 0;
    let mut c = 0;

    ////// generated code:
    while instr < 8 {
        // Modulo8(4)
        if instr == 0 {
            b = a % 8;
            instr += 1;
        }
        // BitwiseXorB(5)
        if instr == 1 {
            b ^= 5;
            instr += 1;
        }
        // DivisionC(5)
        if instr == 2 {
            c = a >> b;
            instr += 1;
        }
        // BitwiseXorB(6)
        if instr == 3 {
            b ^= 6;
            instr += 1;
        }
        // BitwiseXorBC
        if instr == 4 {
            b ^= c;
            instr += 1;
        }
        // Output(5)
        if instr == 5 {
            return Some(b % 8);
        }
        // DivisionA(3)
        if instr == 6 {
            a >>= 3;
            instr += 1;
        }
        // Jump(0)
        if instr == 7 {
            instr = if a > 0 { 0 } else { instr + 1 }
        }
    }
    ////// generated code

    None
}

fn solve_fast_example(a_value: usize) -> Option<usize> {
    let mut instr = 0;
    let mut a = a_value;
    //let mut b = 0;
    //let mut c = 0;

    ////// generated code:
    while instr < 3 {
        // DivisionA(3)
        if instr == 0 {
            a >>= 3;
            instr += 1;
        }
        // Output(4)
        if instr == 1 {
            return Some(a % 8);
        }
        // Jump(0)
        if instr == 2 {
            instr = if a > 0 { 0 } else { instr + 1 }
        }
    }
    ////// generated code

    None
}

#[allow(clippy::type_complexity)]
fn parse(input: &str) -> Result<(RegistersType, Vec<Operation>, Vec<usize>, &str), Error> {
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
    let program_vec = program_str
        .split(',')
        .map(|prg_sub_str| prg_sub_str.chars().next().unwrap().to_digit(10).unwrap() as usize)
        .collect_vec();
    Ok((registers, program, program_vec, program_str))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> miette::Result<()> {
        let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        assert_eq!("117440", process(&input.replace('\r', ""))?);
        Ok(())
    }

    #[test]
    fn test_input() -> miette::Result<()> {
        let input = include_str!("../input2.txt");
        assert_eq!("107413700225434", process(&input.replace('\r', ""))?);
        Ok(())
    }
}

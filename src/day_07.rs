use itertools::{repeat_n, Itertools};
use std::path::Path;

use rayon::prelude::*;

#[derive(Debug)]
struct Equation {
    result: usize,
    inputs: Vec<usize>,
}

#[derive(Debug)]
enum Operation {
    Add,
    Mul,
    Concat,
}

fn load_equations(input_file: &str) -> Result<Vec<Equation>, String> {
    let raw = crate::load_input_utf8(Path::new(input_file))?;

    raw.lines()
        .map(|line| {
            let mut parts = line.split(':');
            Ok(Equation {
                result: parts
                    .next()
                    .ok_or("Error: No ':' found!")?
                    .parse()
                    .map_err(|err| format!("Error: Can't read result ({err})"))?,

                inputs: parts
                    .next()
                    .ok_or("Error: Missing inputs!")?
                    .split_whitespace()
                    .map(|input| {
                        input
                            .parse()
                            .map_err(|err| format!("Error: Can't read input ({err})"))
                    })
                    .collect::<Result<_, String>>()?,
            })
        })
        .collect()
}

fn correctable(eq: &Equation) -> bool {
    repeat_n([Operation::Add, Operation::Mul].iter(), eq.inputs.len() - 1)
        .multi_cartesian_product()
        .map(|op_trial| {
            op_trial
                .iter()
                .zip(&eq.inputs[1..])
                .fold(eq.inputs[0], |acc, (op, input)| match op {
                    Operation::Add => acc + input,
                    Operation::Mul => acc * input,
                    _ => panic!(), // Can't get here unless the array above isn't covered
                })
        })
        .any(|result| result == eq.result)
}

pub fn part_1() -> Result<usize, String> {
    let data = load_equations("input/input_07.txt")?;
    Ok(data
        .iter()
        .filter_map(|eq| correctable(eq).then_some(eq.result))
        .sum())
}

#[test]
fn test_part_1() {
    let obfuscated_answer = part_1().map(|answer| dbg!(answer) ^ 0x55555555);
    assert_eq!(obfuscated_answer, Ok(665034625230));
}

fn concat_usize(a: usize, b: usize) -> usize {
    format!("{}{}", a, b).parse().unwrap()
}

fn correctable2(eq: &Equation) -> bool {
    repeat_n(
        [Operation::Add, Operation::Mul, Operation::Concat].iter(),
        eq.inputs.len() - 1,
    )
    .multi_cartesian_product()
    .map(|op_trial| {
        op_trial
            .iter()
            .zip(&eq.inputs[1..])
            .fold(eq.inputs[0], |acc, (op, input)| match op {
                Operation::Add => acc + input,
                Operation::Mul => acc * input,
                Operation::Concat => concat_usize(acc, *input),
            })
    })
    .any(|result| result == eq.result)
}

pub fn part_2() -> Result<usize, String> {
    let data = load_equations("input/input_07.txt")?;
    Ok(data
        .par_iter()
        .filter_map(|eq| correctable2(eq).then_some(eq.result))
        .sum())
}

#[test]
fn test_part_2() {
    let obfuscated_answer = part_2().map(|answer| dbg!(answer) ^ 0x5555_5555_5555_5555);
    assert_eq!(obfuscated_answer, Ok(6148874973171459244));
}

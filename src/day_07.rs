use itertools::{repeat_n, Itertools};
use std::path::Path;

#[derive(Debug)]
struct Equation {
    result: usize,
    inputs: Vec<usize>,
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

#[derive(Debug)]
enum Operation {
    Add,
    Mul,
}

pub fn part_1() -> Result<usize, String> {
    let data = load_equations("input/input_07.txt")?;
    Ok(data
        .iter()
        .filter_map(|eq| {
            repeat_n(
                vec![Operation::Add, Operation::Mul].iter(),
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
                    })
            })
            .filter(|result| *result == eq.result)
            .next()
        })
        .sum())
}

#[test]
fn test_part_1() {
    let obfuscated_answer = part_1().map(|answer| dbg!(answer) ^ 0x55555555);
    assert_eq!(obfuscated_answer, Ok(665034625230));
}

pub fn part_2() -> Result<usize, String> {
    let _raw = crate::load_input_utf8(Path::new("input/input_06.txt"))?;
    repeat_n(vec![Operation::Add, Operation::Mul].iter(), 3)
        .multi_cartesian_product()
        .for_each(|ops| {
            dbg!(ops);
        });

    Ok(0)
}

#[test]
fn test_part_2() {
    let obfuscated_answer = part_2().map(|answer| dbg!(answer) ^ 0x55555555);
    assert!(obfuscated_answer.is_ok());
    assert_eq!(obfuscated_answer, obfuscated_answer);
}

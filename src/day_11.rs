use std::{
    collections::HashMap,
    iter::{once, successors},
    path::Path,
};

#[derive(Debug)]
enum DecimalParity {
    Odd(usize),
    Even(usize, usize),
}

impl DecimalParity {
    fn from(n: usize) -> Self {
        let s = format!("{}", n);
        match s.chars().count() % 2 == 0 {
            true => {
                let left = s[..s.len() / 2].parse().unwrap();
                let right = s[s.len() / 2..].parse().unwrap();
                DecimalParity::Even(left, right)
            }
            false => DecimalParity::Odd(n),
        }
    }
}

#[derive(Debug)]
struct Stones {
    values: Vec<usize>,
}

impl Iterator for Stones {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let values: Vec<usize> = self
            .values
            .iter()
            .flat_map(|n| match DecimalParity::from(*n) {
                DecimalParity::Odd(0) => Box::new(once(1usize)) as Box<dyn Iterator<Item = usize>>,
                DecimalParity::Odd(n) => {
                    Box::new(once(n * 2024)) as Box<dyn Iterator<Item = usize>>
                }
                DecimalParity::Even(l, r) => {
                    Box::new(once(l).chain(once(r))) as Box<dyn Iterator<Item = usize>>
                }
            })
            .collect();
        let ret = values.len();
        self.values = values;
        Some(ret)
    }
}

fn load_stones() -> Result<Stones, String> {
    let raw = crate::load_input_utf8(Path::new("input/input_11.txt"))?;
    let values = raw
        .split_whitespace()
        .map(|stone| {
            stone
                .parse()
                .map_err(|err| format!("Input Error: Stone is not a number ({err})"))
        })
        .collect::<Result<Vec<usize>, String>>()?;

    Ok(Stones { values })
}

pub fn part_1() -> Result<usize, String> {
    let mut init = load_stones()?;
    Ok(init.nth(25 - 1).unwrap())
}

#[test]
fn test_part_1() {
    let obfuscated_answer = part_1().map(|answer| dbg!(answer) ^ 0x55555555);
    assert_eq!(obfuscated_answer, Ok(1431727167));
}

fn blink(counts: &HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut ret = HashMap::new();
    for (val, count) in counts {
        match DecimalParity::from(*val) {
            DecimalParity::Odd(0) => *ret.entry(1).or_default() += count,
            DecimalParity::Odd(n) => *ret.entry(n * 2024).or_default() += count,
            DecimalParity::Even(l, r) => {
                *ret.entry(l).or_default() += count;
                *ret.entry(r).or_default() += count;
            }
        }
    }
    ret
}

pub fn part_2() -> Result<usize, String> {
    let counts = {
        let mut counts: HashMap<usize, usize> = HashMap::new();
        for val in load_stones()?.values {
            *counts.entry(val).or_default() += 1;
        }
        counts
    };

    Ok(successors(Some(counts), |i| Some(blink(i)))
        .nth(75)
        .unwrap()
        .values()
        .copied()
        .sum())
}

#[test]
fn test_part_2() {
    let obfuscated_answer = part_2().map(|answer| dbg!(answer) ^ 0x55555555);
    assert!(obfuscated_answer.is_ok());
    assert_eq!(obfuscated_answer, obfuscated_answer);
}

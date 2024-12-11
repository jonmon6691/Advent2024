pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_09;
pub mod day_10;
pub mod day_11;

use regex::Regex;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead, Read};
use std::path::Path;

pub fn load_input_utf8(input_file: &Path) -> Result<String, String> {
    let file = File::open(input_file)
        .map_err(|err| format!("Error: Can't open file \"{input_file:?}\" ({err})"))?;
    let raw = io::BufReader::new(file)
        .bytes()
        .map_while(Result::ok)
        .collect();
    String::from_utf8(raw).map_err(|err| format!("Error: Unable to decode UTF8 ({err})"))
}

pub fn load_input_vec_of_vecs(input_file: &Path) -> Result<Vec<Vec<i32>>, String> {
    let lines = read_lines(input_file)
        .map_err(|err| format!("Error reading file {input_file:?} ({err})"))?;

    lines
        .map_while(Result::ok)
        .enumerate()
        .map(|(line_num, line)| {
            line.split_whitespace()
                .map(|val| -> Result<i32, String> {
                    val.parse()
                        .map_err(|err| format!("Input error: Line {line_num} in {input_file:?}: Can't parse value \"{val}\" ({err})"))
                }).collect::<Result<Vec<i32>, String>>()
        }).collect()
}

pub fn load_input_2_cols(input_file: &Path) -> Result<(Vec<i32>, Vec<i32>), String> {
    let lines = read_lines(input_file)
        .map_err(|err| format!("Error reading file {input_file:?} ({err})"))?;

    // Matches two integers separated by whitespace
    let line_parser = Regex::new(r"^\s*(\d+)\s+(\d+)\s*$").unwrap();

    Ok(lines
        .map_while(Result::ok) // Handles the Result<line, _> TODO: What happens to the error?
        .enumerate() // For line numbers in the error report
        .map(|(line_num, line)| {
            match line_parser.captures(&line) {
                Some(matches) => {
                    // These unwraps can't fail because of how the regex is written, errors will all fall into the None match arm below
                    let left = matches.get(1).unwrap().as_str().parse().unwrap();
                    let right = matches.get(2).unwrap().as_str().parse().unwrap();
                    Ok((left, right))
                },
                None => {
                    let line_num = line_num + 1;
                    Err(format!("Input Error: Line {line_num} in {input_file:?}: Expected two whitespace-separated ints, but got \"{line}\""))
                }}
            })
        .collect::<Result<Vec<(i32, i32)>, String>>()?
        .into_iter()
        .unzip())
}

fn get_counts(data: &[i32]) -> HashMap<i32, isize> {
    let mut counts = HashMap::new();
    for num in data {
        counts.entry(*num).and_modify(|v| *v += 1).or_insert(1);
    }
    counts
}

/// Return a new vec with all the elements of data but skip the i'th element.
///
/// Panics if i is not a valid index for data
pub fn drop_i(data: &[i32], i: usize) -> Vec<i32> {
    data.iter()
        .enumerate()
        .filter_map(|(j, v)| (j != i).then_some(*v))
        .collect::<Vec<i32>>()
}

// From the Rust stdlib documentation
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Print a vec in a single row
pub fn pvec<T>(v: &[T])
where
    T: Debug,
{
    for i in v {
        print!("{i:?} ");
    }
    println!();
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn moved(&self, dir: Direction) -> Option<Position> {
        Some(match dir {
            Direction::Up => Position {
                x: self.x.checked_sub(1)?,
                y: self.y,
            },
            Direction::Down => Position {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Left => Position {
                x: self.x,
                y: self.y.checked_sub(1)?,
            },
            Direction::Right => Position {
                x: self.x,
                y: self.y + 1,
            },
        })
    }

    pub fn distance(&self, other: &Position) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn turn(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

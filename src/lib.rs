pub mod day_01;

use itertools::Itertools;
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::{fs::File, path::Path};

pub fn load_input_2_cols(input_file: &Path) -> Result<(Vec<i32>, Vec<i32>), ()> {
    let lines = read_lines(input_file).map_err(|err| {
        eprintln!("Error reading file {input_file:?} ({err})");
    })?;
    Ok(lines
        .flatten()
        .map(|line: String| {
            row_to_2_ints(&line)
        })
        // I wish I knew how to do this without allocating storage :/
        .collect::<Result<Vec<(i32, i32)>, ()>>()?
        .into_iter()
        .unzip())
}

fn row_to_2_ints(row: &str) -> Result<(i32, i32), ()> {
    row.split_whitespace()
        .map(|cell| cell.parse().map_err(|err| {
            eprintln!("Error parsing int from string \"{cell}\" ({err})")
        }))
        // I wish I knew how to do this without allocating storage :/
        .collect::<Result<Vec<i32>, ()>>()?
        .into_iter()
        .tuples()
        .next()
        .ok_or_else(|| eprintln!("Error parsing columns from row \"{row}\", expecting 2 ints separated by whitespace"))
}

fn get_counts(data: &[i32]) -> HashMap<i32, isize> {
    let mut counts = HashMap::new();
    for num in data {
        counts.entry(num.clone())
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }
    counts
}

// From the Rust stdlib documentation
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

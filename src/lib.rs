pub mod day_01;

use regex::Regex;
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::{fs::File, path::Path};

pub fn load_input_2_cols(input_file: &Path) -> Result<(Vec<i32>, Vec<i32>), String> {
    let lines = read_lines(input_file)
        .map_err(|err| format!("Error reading file {input_file:?} ({err})"))?;

    // Matches two integers separated by whitespace
    let line_parser = Regex::new(r"^\s*(\d+)\s+(\d+)\s*$").unwrap();

    Ok(lines
        .flatten() // Handles the Result<line, _> TODO: What happens to the error?
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
        counts
            .entry(num.clone())
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

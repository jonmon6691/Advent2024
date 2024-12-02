use itertools::Itertools;
use std::{ops::Not, path::Path};

pub fn do_d02_1() -> Result<usize, String> {
    let readings = crate::load_input_vec_of_vecs(Path::new("input/input02-1.txt"))?;
    Ok(readings
        .iter()
        .map(|r| {
            // Subtract the reading from itself but shifted by one 'level', this creates a vector of increments/decraments
            let deltas: Vec<i32> = r.iter().tuple_windows().map(|(a, b)| *b - *a).collect();
            // Calculate min/max of the deltas
            match crate::VecStats::from_vec(&deltas) {
                None => false, // Empty list case
                Some(d) => (d.min >= 1 && d.max <= 3) || (d.max <= -1 && d.min >= -3),
            }
        })
        .filter(|result| *result)
        .count())
}

fn test_delta(is_incr: bool, delta: i32) -> bool {
    (is_incr && (1..=3).contains(&delta)) || (is_incr.not() && (-3..=-1).contains(&delta))
}

pub fn do_d02_2() -> Result<usize, String> {
    let readings = crate::load_input_vec_of_vecs(Path::new("input/input02-1.txt"))?;

    Ok(readings
        .iter()
        .map(|r| {
            // Subtract the reading from itself but shifted by one 'level', this creates a vector of increments/decraments
            let deltas: Vec<i32> = r.iter().tuple_windows().map(|(a, b)| *b - *a).collect();
            let is_incr = deltas[0] > 0; // TODO: Handle empty list

            // Contains a list of indexes where a violation occured
            let violations: Vec<usize> = deltas
                .iter()
                .enumerate()
                .filter_map(|(i, d)| {
                    if test_delta(is_incr, *d) {
                        Some(i)
                    } else {
                        None
                    }
                })
                .collect();

            match violations.len() {
                0 => true, // Not repairs needed
                1 => {
                    // One repair needed
                    let d = match violations[0] {
                        0 => deltas[violations[0]] + deltas[violations[0] + 1],
                        1.. => deltas[violations[0] - 1] + deltas[violations[0]],
                    };
                    test_delta(is_incr, d)
                }
                2.. => false, // 2 many 2 furious
            }
        })
        .filter(|result| *result)
        .count())
}

#[test]
fn test_day_02_1() {
    let ans = do_d02_1();
    let day01_01_obfuscated_answer = ans.map(|answer| dbg!(answer) ^ 0x55555555);
    assert_eq!(day01_01_obfuscated_answer, Ok(1431655823));
}

#[test]
fn test_day_02_2() {
    let ans = do_d02_2();
    let day01_02_obfuscated_answer = ans.map(|answer| dbg!(answer) ^ 0x55555555);
    assert_eq!(day01_02_obfuscated_answer, Ok(1419059134));
}
